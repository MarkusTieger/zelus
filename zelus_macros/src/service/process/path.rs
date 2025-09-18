/*
 * Copyright (C) 2025 Markus Probst
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */
use crate::service::parse::FunctionArgument;
use manyhow::{Emitter, ErrorMessage};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::LitStr;

pub fn process(
    emitter: &mut Emitter,
    crate_prefix: &TokenStream,
    (path, path_span): (&str, Span),
    fn_args_identified: &[FunctionArgument],
    http_args: &mut TokenStream,
    func_args: &mut Vec<TokenStream>,
    operations: &mut TokenStream,
    schema_extra: &mut TokenStream,
    client_def_body: &mut TokenStream,
    fn_def_args: &mut TokenStream,
    fn_def_call: &mut TokenStream,
) -> Result<(), ()> {
    let fn_args_path: Vec<_> = fn_args_identified
        .iter()
        .filter_map(|arg| {
            if let FunctionArgument::Path {
                variable_name,
                variable_type,
                no_schema,
            } = arg
            {
                Some((variable_name.clone(), variable_type.clone(), *no_schema))
            } else {
                None
            }
        })
        .collect();

    let mut urlencode = TokenStream::new();

    if !fn_args_path.is_empty() {
        let mut path_names = TokenStream::new();
        let mut path_types = TokenStream::new();

        for (arg_name, arg_type, no_schema) in &fn_args_path {
            path_names.extend(quote! { #arg_name, });
            path_types.extend(quote! { #arg_type, });

            let schema_if = if *no_schema {
                TokenStream::new()
            } else {
                quote! {
                    .schema(Some(
                                #crate_prefix utoipa::openapi::schema::RefBuilder::new()
                                    .ref_location_from_schema_name(< #arg_type as #crate_prefix utoipa::ToSchema >::name())
                                    .build()
                    ))
                }
            };

            let variable_literal = LitStr::new(&arg_name.to_string(), arg_name.span());
            operations.extend(quote! {
                operations = operations.parameter(
                    #crate_prefix utoipa::openapi::path::ParameterBuilder::from(
                        #crate_prefix utoipa::openapi::path::Parameter::new(#variable_literal)
                    )
                    .parameter_in(#crate_prefix utoipa::openapi::path::ParameterIn::Path)
                    .description(Some([< variable_path_ #arg_name:snake >]::DESCRIPTION))
                    #schema_if,
                );
            });
            if !*no_schema {
                schema_extra.extend(quote! {
                    schemas.insert(< #arg_type as #crate_prefix utoipa::ToSchema >::name().to_string(), < #arg_type as #crate_prefix utoipa::PartialSchema >::schema());
                    let mut schemas_vec = Vec::new();
                    < #arg_type as #crate_prefix utoipa::ToSchema >::schemas(&mut schemas_vec);
                    schemas.extend(schemas_vec);
                });
            }

            fn_def_args.extend(quote! { #arg_name: #arg_type, });
            fn_def_call.extend(quote! { #arg_name, });

            urlencode.extend(quote! {
                let #arg_name: String = #arg_name.serialize(#crate_prefix internal::StringSerializer).unwrap().unwrap_or_default();
                let #arg_name = #crate_prefix urlencoding::encode(& #arg_name);
            });
        }

        http_args.extend(quote! {
            #crate_prefix axum::extract::Path((#path_names)): #crate_prefix axum::extract::Path<(#path_types)>,
        });
        func_args.push(quote! { #crate_prefix axum::extract::Path<(#path_types)> });
    }

    let Some(path) = path.strip_prefix("/") else {
        emitter.emit(
            ErrorMessage::new(path_span, "Path for a route must start with /")
                .note("The global path prefix of the service has been taken into account"),
        );
        return Err(());
    };

    let path = LitStr::new(path, Span::call_site());
    client_def_body.extend(quote! {
        use #crate_prefix serde::Serialize;
        #urlencode
        let zelus_result_url: String = format!(#path);
        let mut zelus_result_url = <Self as #crate_prefix sdk::ZelusClientImpl>::base_url(self).join(&zelus_result_url).expect("Expected url");
    });
    Ok(())
}

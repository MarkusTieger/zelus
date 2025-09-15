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
pub mod define;

use convert_case::{Case, Casing as _};
use either::Either;
use proc_macro2::{Delimiter, Group, Ident, Punct, Span, TokenStream, TokenTree};
use quote::quote;
use std::collections::{HashMap, HashSet};
use syn::parse::{Parse, ParseStream};
use syn::token::{Brace, Bracket};
use syn::{LitStr, Token};

// Taken from https://stackoverflow.com/a/73727708
// Allows multiple errors with the same status code
const INVISIBLE_CHARS_HEX: &[u32] = &[
    0x00AD, 0x061C, 0x180E, 0x200B, 0x200C, 0x200D, 0x200E, 0x200F, 0x202A, 0x202B, 0x202C, 0x202D,
    0x202E, 0x2060, 0x2061, 0x2062, 0x2063, 0x2064, 0x2067, 0x2066, 0x2068, 0x2069, 0x206A, 0x206B,
    0x206C, 0x206D, 0x206E, 0x206F, 0xFEFF, 0x1D173, 0x1D174, 0x1D175, 0x1D176, 0x1D177, 0x1D178,
    0x1D179, 0x1D17A, 0xE0001, 0xE0020, 0xE0021, 0xE0022, 0xE0023, 0xE0024, 0xE0025, 0xE0026,
    0xE0027, 0xE0028, 0xE0029, 0xE002A, 0xE002B, 0xE002C, 0xE002D, 0xE002E, 0xE002F, 0xE0030,
    0xE0031, 0xE0032, 0xE0033, 0xE0034, 0xE0035, 0xE0036, 0xE0037, 0xE0038, 0xE0039, 0xE003A,
    0xE003B, 0xE003C, 0xE003D, 0xE003E, 0xE003F, 0xE0040, 0xE0041, 0xE0042, 0xE0043, 0xE0044,
    0xE0045, 0xE0046, 0xE0047, 0xE0048, 0xE0049, 0xE004A, 0xE004B, 0xE004C, 0xE004D, 0xE004E,
    0xE004F, 0xE0050, 0xE0051, 0xE0052, 0xE0053, 0xE0054, 0xE0055, 0xE0056, 0xE0057, 0xE0058,
    0xE0059, 0xE005A, 0xE005B, 0xE005C, 0xE005D, 0xE005E, 0xE005F, 0xE0060, 0xE0061, 0xE0062,
    0xE0063, 0xE0064, 0xE0065, 0xE0066, 0xE0067, 0xE0068, 0xE0069, 0xE006A, 0xE006B, 0xE006C,
    0xE006D, 0xE006E, 0xE006F, 0xE0070, 0xE0071, 0xE0072, 0xE0073, 0xE0074, 0xE0075, 0xE0076,
    0xE0077, 0xE0078, 0xE0079, 0xE007A, 0xE007B, 0xE007C, 0xE007D, 0xE007E, 0xE007F,
];

#[expect(clippy::indexing_slicing, reason = "this is a const evaluation")]
const INVISIBLE_CHARS: [char; INVISIBLE_CHARS_HEX.len()] = {
    let mut chars = [' '; INVISIBLE_CHARS_HEX.len()];

    let mut pos = 0;
    while pos < INVISIBLE_CHARS_HEX.len() {
        if let Some(char) = char::from_u32(INVISIBLE_CHARS_HEX[pos]) {
            chars[pos] = char;
        } else {
            panic!("Invisible Char is not actually a char");
        }
        pos += 1;
    }
    chars
};

type ErrorAttributes = Vec<(Ident, Span, Option<Ident>)>;
type ErrorInner = Either<(Ident, Ident, LitStr), (Ident, Ident)>;

pub struct ErrorContent {
    pub errors: Vec<(ErrorInner, ErrorAttributes)>,
    pub ident: Ident,
}

impl Parse for ErrorContent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut errors = Vec::new();
        let ident: Ident = input.parse()?;

        while !input.is_empty() {
            if input.peek(Token![,]) {
                let _: Punct = input.parse()?;
            }
            if input.peek(syn::Ident) {
                let category: Ident = input.parse()?;
                let content: Group = input.parse()?;
                if content.delimiter() != Delimiter::Parenthesis {
                    return Err(syn::Error::new(content.span(), "Expected parenthesis"));
                }
                let content: ErrorCategoryContent = syn::parse2(content.stream())?;
                for (error, attributes) in content.vec {
                    errors.push((Either::Right((category.clone(), error)), attributes));
                }
            } else if input.peek(Brace) {
                let custom: Group = input.parse()?;
                let content: ErrorCustomContent = syn::parse2(custom.stream())?;
                for (error, statuscode, description, attributes) in content.vec {
                    errors.push((Either::Left((error, statuscode, description)), attributes));
                }
            } else {
                let token: TokenTree = input.parse()?;
                return Err(syn::Error::new(
                    token.span(),
                    "Expected identifier or braces",
                ));
            }

            if input.peek(Token![,]) {
                let _: Punct = input.parse()?;
            }
        }

        Ok(Self { errors, ident })
    }
}

pub struct ErrorCategoryContent {
    pub vec: Vec<(Ident, ErrorAttributes)>,
}

impl Parse for ErrorCategoryContent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut vec = Vec::new();

        while !input.is_empty() {
            let error: Ident = input.parse()?;
            let attributes = if input.peek(Bracket) {
                let group: Group = input.parse()?;
                let attributes: ErrorAttributesContent = syn::parse2(group.stream())?;
                attributes.vec
            } else {
                Vec::new()
            };
            vec.push((error, attributes));
        }

        Ok(Self { vec })
    }
}

pub struct ErrorCustomContent {
    pub vec: Vec<(Ident, Ident, LitStr, ErrorAttributes)>,
}

impl Parse for ErrorCustomContent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut vec = Vec::new();

        while !input.is_empty() {
            if input.peek(Token![,]) {
                let _: Punct = input.parse()?;
            }
            let error: Ident = input.parse()?;
            let description: LitStr = input.parse()?;
            let statuscode: Ident = input.parse()?;
            let attributes = if input.peek(Bracket) {
                let group: Group = input.parse()?;
                let attributes: ErrorAttributesContent = syn::parse2(group.stream())?;
                attributes.vec
            } else {
                Vec::new()
            };
            vec.push((error, statuscode, description, attributes));

            if input.peek(Token![,]) {
                let _: Punct = input.parse()?;
            }
        }

        Ok(Self { vec })
    }
}

pub struct ErrorAttributesContent {
    pub vec: Vec<(Ident, Span, Option<Ident>)>,
}

impl Parse for ErrorAttributesContent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut vec = Vec::new();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let punct: Punct = input.parse()?;
            if punct.as_char() != ':' {
                return Err(syn::Error::new(punct.span(), "Expected :"));
            }
            if input.peek(Token![*]) {
                let value: Punct = input.parse()?;
                vec.push((key, value.span(), None));
            } else {
                let value: Ident = input.parse()?;
                vec.push((key, value.span(), Some(value)));
            }
        }

        Ok(Self { vec })
    }
}

pub fn error0(crate_prefix: &TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    let ErrorContent { ident, mut errors } = syn::parse2(input)?;

    let mut content = TokenStream::new();
    let mut values_impl = TokenStream::new();
    let mut response = TokenStream::new();
    let mut from = TokenStream::new();
    let mut openapi = TokenStream::new();
    let mut impls = TokenStream::new();

    errors.insert(
        0,
        (
            Either::Left((
                Ident::new("Communication", Span::call_site()),
                Ident::new("BAD_REQUEST", Span::call_site()),
                LitStr::new("Invalid Request/Response", Span::call_site()),
            )),
            Vec::new(),
        ),
    );

    let mut attributes_parsed = HashMap::new();
    let mut errors2 = Vec::with_capacity(errors.len());
    for (index, (error, attributes)) in errors.into_iter().enumerate() {
        let error_suffix = INVISIBLE_CHARS
            .get(index)
            .ok_or_else(|| {
                syn::Error::new(
                    ident.span(),
                    "Too many errors. Not enough invisible chars hardcoded",
                )
            })?
            .to_string(); // So I can have multiple errors in openapi with the same status code

        let error_suffix = LitStr::new(&error_suffix, Span::call_site());
        let entry = match error {
            Either::Left((error_ident, statuscode, msg)) => {
                let error_id = LitStr::new(
                    &error_ident.to_string().to_case(Case::Snake),
                    Span::call_site(),
                );

                if error_ident.to_string().eq("Communication") {
                    content.extend(quote! {
                        #error_ident (#crate_prefix reqwest::Error),
                    });

                    response.extend(quote! {
                        #error_ident(_err) => panic!("You tried to use the communication error entry as response"),
                    });
                } else {
                    content.extend(quote! {
                        #error_ident,
                    });

                    from.extend(quote! {
                        #error_id => Ok(#error_ident),
                    });

                    response.extend(quote! {
                        #error_ident => (#crate_prefix http::StatusCode::#statuscode,#crate_prefix axum::extract::Json(serde_json::json!({
                            "error": {
                                "id": #error_id,
                                "msg": #msg,
                            }
                        }))).into_response(),
                    });

                    openapi.extend(quote! {
                        responses = responses.response(
                            format!("{}{}", #crate_prefix http::StatusCode::#statuscode.as_str(), #error_suffix),
                            #crate_prefix utoipa::openapi::ResponseBuilder::new()
                                .description(#msg)
                                .content(
                                    "application/json",
                                    #crate_prefix utoipa::openapi::content::Content::new(Some(
                                        #crate_prefix utoipa::openapi::RefOr::T(#crate_prefix internal::error_schema(#error_id, #msg))
                                    )),
                                )
                                .build(),
                        );

                    });

                    values_impl.extend(quote! {
                        #crate_prefix error::ZelusErrorValue {
                            id: #error_id,
                            msg: #msg,
                            code: #crate_prefix http::StatusCode::#statuscode,
                            instance: &Self::#error_ident,
                        },
                    });
                }
                quote! { [< #error_ident >] }
            }
            Either::Right((category, error_ident)) => {
                let error_id = LitStr::new(&format!("{category}/{error_ident}"), Span::call_site());

                from.extend(quote! {
                    #error_id => Ok([< #category:camel #error_ident:camel >]),
                });

                content.extend(quote! {
                    [< #category:camel #error_ident:camel >],
                });

                response.extend(quote! {
                    [< #category:camel #error_ident:camel >] => ( [< error_ #category _ #error_ident >]::STATUSCODE, #crate_prefix axum::extract::Json(serde_json::json!({
                        "error": {
                            "id": #error_id,
                            "msg": [< error_ #category _ #error_ident >]::DESCRIPTION,
                        }
                    }))).into_response(),
                });

                openapi.extend(quote! {
                    responses = responses.response(
                        format!("{}{}", [< error_ #category _ #error_ident >]::STATUSCODE.as_str(), #error_suffix),
                        #crate_prefix utoipa::openapi::ResponseBuilder::new()
                            .description([< error_ #category _ #error_ident >]::DESCRIPTION)
                            .content(
                                "application/json",
                                #crate_prefix utoipa::openapi::content::Content::new(Some(
                                    #crate_prefix utoipa::openapi::RefOr::T(#crate_prefix internal::error_schema(#error_id, [< error_ #category _ #error_ident >]::DESCRIPTION))
                                )),
                            )
                            .build(),
                    );
                });

                impls.extend(quote! {
                    #[automatically_derived]
                    impl [< error_ #category _ #error_ident >]::[< From #category:camel #error_ident:camel >] for #ident {

                        fn from() -> Self {
                            Self::[< #category:camel #error_ident:camel >]
                        }

                    }
                });
                values_impl.extend(quote! {
                    #crate_prefix error::ZelusErrorValue {
                        id: #error_id,
                        msg: [< error_ #category _ #error_ident >]::DESCRIPTION,
                        code: [< error_ #category _ #error_ident >]::STATUSCODE,
                        instance: &Self::[< #category:camel #error_ident:camel >],
                    },
                });
                quote! { [< #category:camel #error_ident:camel >] }
            }
        };

        for (attr, key_span, key) in &attributes {
            if key.is_none() {
                if attributes_parsed.contains_key(&attr.to_string()) {
                    return Err(syn::Error::new(
                        *key_span,
                        format!(
                            "There is already an {attr} attribute with an default entry in this error"
                        ),
                    ));
                }
                attributes_parsed
                    .entry(attr.to_string())
                    .or_insert_with(|| (attr.span(), entry.clone(), Vec::new()));
            }
        }
        errors2.push((attributes, entry));
    }
    for (attributes, entry) in errors2 {
        for (attr, _key_span, key) in attributes {
            let Some(key) = key else {
                continue;
            };
            if let Some((_, _, vec)) = attributes_parsed.get_mut(&attr.to_string()) {
                vec.push((key, entry.clone()));
            } else {
                return Err(syn::Error::new(
                    attr.span(),
                    "You cannot use an specific error attribute, without having an default one",
                ));
            }
        }
    }

    for (attr, (_, _, vec)) in &attributes_parsed {
        let mut set = HashSet::new();
        for (entry, _) in vec {
            if !set.insert(entry.to_string()) {
                return Err(syn::Error::new(
                    entry.span(),
                    format!("Duplicate entry {entry} for attribute {attr} in error"),
                ));
            }
        }
    }

    for (attr, (attr_span, def, other)) in attributes_parsed {
        match attr.as_str() {
            #[cfg(feature = "io")]
            "io" => {
                let mut matches = TokenStream::new();
                for (kind, variant) in other {
                    matches.extend(quote! { ErrorKind::#kind => Self::#variant, });
                }
                impls.extend(quote! {
                    impl From<std::io::Error> for #ident {

                        fn from(err: std::io::Error) -> Self {
                            use std::io::ErrorKind;
                            match err.kind() {
                                #matches
                                _ => Self::#def,
                            }
                        }

                    }
                });
            }
            #[cfg(feature = "sqlx")]
            "sql" => {
                let mut matches = TokenStream::new();
                let mut database_matches = TokenStream::new();
                for (kind, variant) in other {
                    match kind.to_string().as_str() {
                        "not_found" => {
                            matches.extend(quote! {
                                if let #crate_prefix sqlx::Error::RowNotFound = err {
                                    return Self::#variant;
                                }
                            });
                        }
                        "check" => {
                            database_matches.extend(quote! {
                                    if database.is_check_violation() {
                                        return Self::#variant;
                                    }
                            });
                        }
                        "foreign_key" => {
                            database_matches.extend(quote! {
                                    if database.is_foreign_key_violation() {
                                        return Self::#variant;
                                    }
                            });
                        }
                        "unique" => {
                            database_matches.extend(quote! {
                                    if database.is_unique_violation() {
                                        return Self::#variant;
                                    }
                            });
                        }
                        _ => {
                            return Err(syn::Error::new(
                                kind.span(),
                                format!("Unknown entry {kind} for {attr} in error"),
                            ));
                        }
                    }
                }
                if !database_matches.is_empty() {
                    matches.extend(quote! {
                        if let #crate_prefix sqlx::Error::Database(ref database) = err {
                            #database_matches
                        }
                    });
                }
                impls.extend(quote! {
                    #[automatically_derived]
                    impl From<#crate_prefix sqlx::Error> for #ident {

                        fn from(err: #crate_prefix sqlx::Error) -> Self {
                            #matches
                            Self::#def
                        }

                    }
                });
            }
            #[cfg(feature = "redis")]
            "redis" => {
                if let Some((kind, _variant)) = other.first() {
                    return Err(syn::Error::new(
                        kind.span(),
                        format!("Unknown entry {kind} for {attr} in error"),
                    ));
                }
                impls.extend(quote! {
                    #[automatically_derived]
                    impl From<#crate_prefix redis::RedisError> for #ident {

                        fn from(_err: #crate_prefix redis::RedisError) -> Self {
                            Self::#def
                        }

                    }
                });
            }
            _ => {
                return Err(syn::Error::new(attr_span, "Unknown error attribute"));
            }
        }
    }

    Ok(quote! {
        #crate_prefix paste! {
            #[derive(Debug)]
            #[automatically_derived]
            pub enum #ident {
                #content
            }

            #[automatically_derived]
            impl std::str::FromStr for #ident {
                type Err = ();

                fn from_str(s: &str) -> Result<Self, ()> {
                    use #ident::*;
                    match s {
                        #from
                        _ => Err(()),
                    }
                }

            }

            #[automatically_derived]
            impl #crate_prefix axum::response::IntoResponse for #ident {

                fn into_response(self) -> #crate_prefix http::response::Response<#crate_prefix axum::body::Body> {
                    use #ident::*;
                    #[allow(unreachable_code)]
                    match self {
                        #response
                    }
                }

            }

            #[automatically_derived]
            impl #crate_prefix responses::DocumentedResponse for #ident {

                fn openapi(mut responses: #crate_prefix utoipa::openapi::ResponsesBuilder, _schemas: &mut std::collections::HashMap<String, #crate_prefix utoipa::openapi::RefOr<#crate_prefix utoipa::openapi::schema::Schema>>) -> #crate_prefix utoipa::openapi::ResponsesBuilder {
                    #openapi
                    responses
                }

            }

            #[automatically_derived]
            impl #crate_prefix error::ZelusError for #ident {

                type Error = Self;

                fn error_values() -> &'static [#crate_prefix error::ZelusErrorValue<Self::Error>] {
                    &[ #values_impl ]
                }

            }

            #[automatically_derived]
            impl From<#crate_prefix reqwest::Error> for #ident {

                fn from(err: #crate_prefix reqwest::Error) -> Self {
                    Self::Communication(err)
                }

            }

            #impls
        }
    })
}

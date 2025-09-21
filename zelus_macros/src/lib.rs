// SPDX-License-Identifier: AGPL-3.0-only
// #![forbid(clippy::expect_used)]
// #![forbid(clippy::unwrap_used)]
// #![forbid(clippy::panic)]
#![forbid(
    clippy::inline_always,
    reason = "This is a macro. Compile time is more important here than minor speed improvements."
)]

use crate::service::args::ServiceArgs;
use manyhow::Emitter;

#[cfg(feature = "error")]
mod error;
#[cfg(feature = "service")]
mod service;
#[cfg(any(feature = "error", feature = "service"))]
mod utils;

#[cfg(feature = "service")]
#[manyhow::manyhow]
#[proc_macro_attribute]
pub fn service(
    attr: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
    emitter: &mut Emitter,
) -> Result<proc_macro2::TokenStream, manyhow::Error> {
    let args: ServiceArgs = syn::parse2(attr)?;
    service::service0(emitter, &utils::crate_prefix(), &args, input)
}

#[cfg(feature = "error")]
#[manyhow::manyhow]
#[proc_macro]
pub fn error(attr: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    error::error0(&utils::crate_prefix(), attr)
}

#[cfg(feature = "error")]
#[proc_macro]
pub fn define_error(attr: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(error::define::define_error0(
        &utils::crate_prefix(),
        proc_macro2::TokenStream::from(attr),
    ))
}

// SPDX-License-Identifier: AGPL-3.0-only
use proc_macro2::TokenStream;
use quote::quote;

pub fn crate_prefix() -> TokenStream {
    quote! {
        ::zelus::
    }
}

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
#![expect(
    clippy::arbitrary_source_item_ordering,
    reason = "This order is more readable"
)]

pub mod sdk;
pub mod types;

#[doc(hidden)]
pub mod internal;
pub mod responses;
pub mod router;
pub mod special;
pub(crate) mod utils;

pub use async_trait::async_trait;
pub use axum;
pub use axum_extra;
pub use http;
pub use pastey::paste;
#[cfg(feature = "redis")]
pub use redis;
pub use reqwest;
pub use serde;
pub use serde_json;
pub use serde_urlencoded;
#[cfg(feature = "sqlx")]
pub use sqlx;
pub use tap;
pub use url;
pub use urlencoding;
pub use utoipa;
pub use utoipa_axum;
pub use zelus_macros::service;
#[cfg(feature = "error")]
pub mod error;

extern crate self as zelus;

pub(crate) const SUCCESS_DESCRIPTION: &str = "Success";

#[macro_export]
macro_rules! define_path_variable {
    ($name:ident $description:literal) => {
        $crate::paste! {
            pub mod [< variable_path_ $name:snake >] {

                pub const DESCRIPTION: &str = $description;

            }
        }
    };
}

#[macro_export]
macro_rules! define_query_variable {
    ($name:ident $description:literal) => {
        $crate::paste! {
            pub mod [< variable_query_ $name:snake >] {

                pub const DESCRIPTION: &str = $description;

            }
        }
    };
}

#[macro_export]
macro_rules! define_header_variable {
    ($name:ident $description:literal) => {
        $crate::paste! {
            pub mod [< variable_header_ $name:snake >] {

                pub const DESCRIPTION: &str = $description;

            }
        }
    };
}

#[macro_export]
macro_rules! framework_router {
    ($base:tt $variable:ident ($($variant:ident,)* ) $content:tt) => {
        ($(
            {
                $crate::framework_router_inner!($base $variable $variant $content)
            }
        ,)*)
    };
}

#[macro_export]
macro_rules! framework_router_inner {
    ($base:tt $variable:ident $variant:ident { $($tr:path),* $( { $($feat:literal $tr2:path),* } )* }) => {
        {
            use $crate::tap::Pipe;
            $crate::paste! {
                $crate::router::ZelusRouter::new()
                    $(
                        .merge(<$base as $tr>:: [< routes_ $variant >] (&$variable))
                    )*
                    $(
                        $(
                            .pipe(|router| {
                                #[cfg(feature = $feat)]
                                {
                                    router.merge(<$base as $tr2>::routes(&$variable))
                                }
                                #[cfg(not(feature = $feat))]
                                {
                                    router
                                }
                            })
                        )*
                    )*
            }
        }
    };
}

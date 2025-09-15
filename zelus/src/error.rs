use http::StatusCode;
pub use zelus_macros::define_error;
pub use zelus_macros::error;

#[derive(Copy, Clone)]
pub struct ZelusErrorValue<T: ZelusError + 'static> {
    pub id: &'static str,
    pub msg: &'static str,
    pub code: StatusCode,
    pub instance: &'static T,
}

pub trait ZelusError: Send + Sync {
    type Error: ZelusError;

    fn error_values() -> &'static [ZelusErrorValue<Self::Error>]
    where
        Self: Sized;
}

impl<V: Send + Sync, E: ZelusError<Error = E>> ZelusError for Result<V, E> {
    type Error = E;

    fn error_values() -> &'static [ZelusErrorValue<Self::Error>]
    where
        Self: Sized,
    {
        E::error_values()
    }
}

error!(BlankError);

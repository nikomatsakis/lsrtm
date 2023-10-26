use super::auth;
use duchess::java;
use duchess::prelude::*;
use thiserror::Error;

pub use crate::part3::{Authenticated, HttpRequest};

#[derive(Debug, Error, duchess::ToRust)]
#[java(java.lang.Throwable)]
pub enum AuthenticateError {
    #[error("Unathenticated({user_message})")]
    #[java(auth.AuthenticationExceptionUnauthenticated)]
    Unathenticated { user_message: String },

    #[error("InvalidSignature")]
    #[java(auth.AuthenticationExceptionInvalidSignature)]
    InvalidSignature,

    #[error("Generic({get_message})")]
    #[java(auth.AuthenticationException)]
    Generic { get_message: String },

    #[error("InternalError({get_message})")]
    #[java(java.lang.Throwable)]
    InternalError { get_message: String },
}

pub fn main() -> anyhow::Result<()> {
    let request = HttpRequest {
        verb: "GET".into(),
        path: "/foo".into(),
    };

    let auth: Result<Authenticated, AuthenticateError> = auth::HttpAuth::new()
        .authenticate(&request)
        .assert_not_null()
        .catch::<java::lang::Throwable>() // <-- intercepts thrown exceptions
        .to_rust()
        .execute() // <-- result is now `Result<Result<O, E>>`, why?
        .unwrap(); // <-- Q: why unwrap here?

    let auth = auth?; // <-- handle the error (Q: can we move this elsewhere?)

    println!("account-id: {:?}", &auth.account_id);

    Ok(())
}

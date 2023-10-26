use super::auth;
use duchess::prelude::*;
use duchess::Global;

pub use crate::part4::{AuthenticateError, Authenticated, HttpRequest};

pub struct HttpAuth(Global<auth::HttpAuth>);

impl HttpAuth {
    pub fn new() -> duchess::GlobalResult<Self> {
        let auth = auth::HttpAuth::new().global().execute()?;
        Ok(Self(auth))
    }

    pub fn authenticate(&self, request: &HttpRequest) -> Result<Authenticated, AuthenticateError> {
        self.0
            .authenticate(request)
            .assert_not_null()
            .catch::<duchess::java::lang::Throwable>()
            .to_rust()
            .execute()
            .unwrap()
    }
}

pub fn main() -> anyhow::Result<()> {
    let request = HttpRequest {
        verb: "GET".into(),
        path: "/foo".into(),
    };

    let auth = HttpAuth::new()?.authenticate(&request)?;

    println!("account-id: {:?}", &auth.account_id);

    Ok(())
}

use super::auth;
use duchess::prelude::*;
use duchess::Global;

pub use crate::part2::HttpRequest;

#[derive(duchess::ToRust, duchess::ToJava)] // <-- derive ToRust as well
#[java(auth.Authenticated)] // <-- as before, declare the Java class
pub struct Authenticated {
    pub account_id: String, // <-- defaults to invoking accessor
    pub user: String,

    // Q: what's this field?
    this: Global<auth::Authenticated>,
}

pub fn main() -> anyhow::Result<()> {
    let request = HttpRequest {
        verb: "GET".into(),
        path: "/foo".into(),
    };

    let auth: Authenticated = auth::HttpAuth::new()
        .authenticate(&request)
        .assert_not_null()
        .to_rust()
        .execute()?;

    println!("account-id: {:?}", &auth.account_id);

    Ok(())
}

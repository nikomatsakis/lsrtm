use super::auth;
use duchess::java;
use duchess::prelude::*;
use duchess::Global;

#[derive(Debug, duchess::ToJava)] // <-- derive ToJava
#[java(auth.HttpRequest)] // <-- declares the Java class we convert into
pub struct HttpRequest {
    pub verb: String,
    pub path: String,
}

pub fn main() -> anyhow::Result<()> {
    let request = HttpRequest {
        verb: "GET".into(),
        path: "/foo".into(),
    };

    let auth: Global<auth::Authenticated> = auth::HttpAuth::new()
        .authenticate(&request) // <-- look ma, no Java types
        .assert_not_null()
        .global()
        .execute()?;

    let account_id: Option<String> = auth.account_id().to_rust().execute()?;

    println!("account-id: {:?}", account_id);

    Ok(())
}

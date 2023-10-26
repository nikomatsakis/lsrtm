use super::auth;
use duchess::prelude::*;
use duchess::Global;

pub fn main() -> anyhow::Result<()> {
    // Q: What happens if we remove `global` and why?
    // Q: What does the Err represent?
    let request: Global<auth::HttpRequest> =
        auth::HttpRequest::new("GET", "/").global().execute()?;

    let auth: Global<auth::Authenticated> = auth::HttpAuth::new()
        .authenticate(&request)
        .assert_not_null() // Q: What's this about?
        .global()
        .execute()?;

    let account_id: Option<String> = auth.account_id().to_rust().execute()?;

    println!("account-id: {:?}", account_id);

    Ok(())
}

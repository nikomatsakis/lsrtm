duchess::java_package! {
    package auth;

    class Authenticated { * }
    class AuthenticateRequest { * }
    class HttpAuth { * }
    class HttpRequest { * }

    class AuthenticationException { * }
    class AuthenticationExceptionUnauthenticated { * }
    class AuthenticationExceptionInvalidSignature { * }
}

mod part1;

fn main() -> anyhow::Result<()> {
    part1::main()?;
    Ok(())
}

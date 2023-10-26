use duchess::{java, prelude::*, Global};
use std::collections::HashMap;
use thiserror::Error;

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

fn main() {
    println!("Hello, world!");
}

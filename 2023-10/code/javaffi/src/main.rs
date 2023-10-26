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
mod part2;
mod part3;
mod part4;
mod part5;

fn main() -> anyhow::Result<()> {
    println!("-- Part1 ---------------");
    part1::main()?;

    println!("-- Part2 ---------------");
    part2::main()?;

    println!("-- Part3 ---------------");
    part3::main()?;

    println!("-- Part4 ---------------");
    part4::main()?;

    println!("-- Part5 ---------------");
    part5::main()?;

    Ok(())
}

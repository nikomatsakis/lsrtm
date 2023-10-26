fn main() -> std::io::Result<()> {
    println!("cargo:rustc-env=CLASSPATH=java");
    Ok(())
}

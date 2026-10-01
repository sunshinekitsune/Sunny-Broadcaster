#[cfg(windows)]
fn main() {
    println!("cargo:rerun-if-changed=assets/icon.rc");
    println!("cargo:rerun-if-changed=assets/icon.ico");

    if let Err(error) =
        embed_resource::compile("assets/icon.rc", embed_resource::NONE).manifest_optional()
    {
        eprintln!("Failed to compile Windows resource: {error}");
    } else {
        println!("Compiled Windows resources");
    }
}

#[cfg(not(windows))]
fn main() {}

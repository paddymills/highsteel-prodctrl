
use std::path::Path;

#[cfg(windows)]
use winres::WindowsResource;

fn main() -> std::io::Result<()> {

    #[cfg(windows)] {
    WindowsResource::new()
        .set_icon("assets/ferris.ico")
        .compile()?;
    }

    if !Path::new("assets/db.toml").exists() {
        println!("cargo:warning=⚙️ db.toml does not exist. Run `cargo run --all-features --bin pc cfg --generate` to generate");
    }

    Ok(())
}

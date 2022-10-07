
#[cfg(windows)]
use winres::WindowsResource;

fn main() -> std::io::Result<()> {

    #[cfg(windows)] {
        WindowsResource::new()
            .set_icon("assets/ferris.ico")
            .compile()?;
    }

    println!("cargo:warning=⚙️ Main build script OK");

    Ok(())
}
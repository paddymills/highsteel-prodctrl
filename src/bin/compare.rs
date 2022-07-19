
use prodctrl::{compare::BomWoDxfCompare, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    BomWoDxfCompare::new().await
                .main().await?
                .export()?;

    Ok(())
}

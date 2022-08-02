
mod cmp;

use cmp::BomWoDxfCompare;
use prodctrl::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    BomWoDxfCompare::new().await
        .main().await?
        .export()?;

    Ok(())
}

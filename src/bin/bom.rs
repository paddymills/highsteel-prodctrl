
use prodctrl::{App, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = App::new().await;

    app
        .init_bom("1210052A", 1i32)
        .await?
        .into_iter()
        .for_each( |part| println!("{:#?}", part) );

    Ok(())
}

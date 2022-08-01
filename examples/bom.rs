
use clap::Parser;
use prodctrl::Error;
use prodctrl::ui::cli::{App, CliMenuApp, Menu};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = App::<Bom>::new().await;

    app
        .init_bom("1210052A", 1i32)
        .await?
        .into_iter()
        .for_each( |part| println!("{:#?}", part) );

    Ok(())
}

#[derive(Parser, Debug, Default)]
#[clap(author, version, about)]
pub struct Bom {
    #[clap(short, long, value_parser)]
    job: String,

    #[clap(short, long, value_parser, default_value_t=1)]
    shipment: u8,
}

impl CliMenuApp for Bom {}

impl Menu for Bom {
    fn menu() -> Self {
        Self { ..Default::default() }
    }
}

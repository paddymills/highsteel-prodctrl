
mod app;
mod components;

use std::{
    fs::File,
    io::Error
};

fn main() -> Result<(), Error> {
    let opts = eframe::NativeOptions {
        decorated: false,

        icon_data: Some(get_icon()?),

        min_window_size: Some(egui::Vec2::new(500., 300.)),

        ..Default::default()
    };

    eframe::run_native(
        "prodctrl-extra-sheet",
        opts,
        app::ExtrasSheet::new()
    );

    Ok(())
}

fn get_icon() -> Result<eframe::IconData, Error> {
    let ico_file = File::open("assets/ferris.ico")?;
    let icons = ico::IconDir::read(ico_file)?;

    let icon = icons.entries()[0].decode()?;

    Ok(
        eframe::IconData {
            rgba: icon.rgba_data().to_vec(),
            width: icon.width(),
            height: icon.height()
        }
    )
}

// TODO: web impl (wasm)

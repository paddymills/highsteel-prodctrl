// use prodctrl::config as prodctrl_config;

fn main() {
    let cfg = prodctrl::config::DbConfig::from_embed();

    println!("{:#?}", cfg);
}
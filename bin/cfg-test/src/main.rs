// use prodctrl::config as prodctrl_config;

fn main() {
    let cfg = prodctrl::config::Databases::from_embed();

    println!("{:#?}", cfg);
}

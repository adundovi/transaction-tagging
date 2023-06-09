use clap::App;

mod db;
mod menu;
mod transactions;
mod tags;

use menu::{Menu, main_menu};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get all submenus
    let submenus: Vec<Menu> = vec![
        db::menu(),
        transactions::menu(),
        tags::menu(),
    ];

    // ...generate calp::App from them...
    let menu_apps: Vec<App> =
        submenus.iter()
                .map(|m| m.generate())
                .collect();

    // ...build and parse...
    let cli_builder =
        main_menu()
        .subcommands(menu_apps)
        .get_matches();

    // ...and finally, run the command.
    for m in submenus {
        m.process(&cli_builder);
    }

    Ok(())
}

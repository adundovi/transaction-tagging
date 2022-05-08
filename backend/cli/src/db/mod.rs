use clap::{Arg, App};
use std::collections::HashMap;

use crate::menu::{Menu, Subcommand};

mod new;
mod import;

pub fn menu<'a>() -> Menu<'a> {
    let mut m = Menu{
        name: "db",
        about: "",
        author: "Andrej Dundović <andrej.dundovic@udruga-point.hr>",
        version: "0.1",
        subcommands: HashMap::new()
    };

    let menu_new_db = Subcommand {
            app: App::new("new")
                .about("Create new database")
                .arg(Arg::new("TITLE")
                     .help("Database title")
                     .required(true)
                     .index(1)
                     ),
            f: &new::f
    };
    m.push_subcommand("new", menu_new_db);

    let menu_import = Subcommand {
            app: App::new("import")
                .about("Import transactions from CSV")
                .arg(Arg::new("FILE")
                     .help("CSV file")
                     .required(true)
                     .index(1)
                     ),
            f: &import::f
    };
    m.push_subcommand("import", menu_import);

    m
}

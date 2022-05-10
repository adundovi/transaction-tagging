use clap::{Arg, App};
use std::collections::HashMap;

use crate::menu::{Menu, Subcommand};

mod list;
mod tags;

pub fn menu<'a>() -> Menu<'a> {
    let mut m = Menu{
        name: "trans",
        about: "Add, modify, remove and list transactions",
        author: "Andrej Dundović <andrej.dundovic@udruga-point.hr>",
        version: "0.1",
        subcommands: HashMap::new()
    };
    
    let menu_list = Subcommand {
        app: App::new("list")
            .about("List all transactions"),
        f: &list::f
    };
    m.push_subcommand("list", menu_list);
    
    let menu_tags = Subcommand {
        app: App::new("tags")
            .about("Modify tags")
            .subcommand(
                App::new("list")
                      .about("List tags")
                      .arg(
                         Arg::new("ID")
                         .help("Transaction ID")
                         .required(true)
                         .index(1)
                        )
                )
            .subcommand(
                App::new("add")
                    .about("Add new tag")
                    .arg(Arg::new("ID")
                         .help("Transaction ID")
                         .required(true)
                         .index(1)
                         )
                    .arg(Arg::new("TAG")
                         .help("Some string")
                         .required(true)
                         .index(2)
                         )
            )
            .subcommand(
                App::new("remove")
                    .about("Remove tag")
                    .arg(Arg::new("ID")
                         .help("Transaction ID")
                         .required(true)
                         .index(1)
                         )
                    .arg(Arg::new("TAG")
                         .help("Some string")
                         .required(true)
                         .index(2)
                         )
            ),
        f: &tags::f
    };
    m.push_subcommand("tags", menu_tags);

    m
}

use clap::{Arg, App};
use std::collections::HashMap;

use crate::menu::{Menu, Subcommand};

mod comment;
mod list;
mod export;
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
            .about("List transactions")
            .arg(
                Arg::new("filter")
                    .help("Filter transaction")
            ),
        f: &list::f
    };
    m.push_subcommand("list", menu_list);
    
    let menu_export = Subcommand {
        app: App::new("export")
            .about("Export transactions"),
        f: &export::f
    };
    m.push_subcommand("export", menu_export);
    
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
    
    let menu_comment = Subcommand {
        app: App::new("comment")
            .about("Modify comments")
            .subcommand(
                App::new("show")
                      .about("Display comment")
                      .arg(
                         Arg::new("ID")
                         .help("Transaction ID")
                         .required(true)
                         .index(1)
                        )
            )
            .subcommand(
                App::new("new")
                    .about("Write a comment")
                    .arg(Arg::new("ID")
                         .help("Transaction ID")
                         .required(true)
                         .index(1)
                         )
                    .arg(Arg::new("COMMENT")
                         .help("String")
                         .required(true)
                         .index(2)
                         )
            )
            .subcommand(
                App::new("remove")
                    .about("Remove comment")
                    .arg(Arg::new("ID")
                         .help("Transaction ID")
                         .required(true)
                         .index(1)
                         )
            ),
        f: &comment::f
    };
    m.push_subcommand("comment", menu_comment);

    m
}

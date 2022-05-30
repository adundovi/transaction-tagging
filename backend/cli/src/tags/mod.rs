use clap::{Arg, App};
use std::collections::HashMap;

use crate::menu::{Menu, Subcommand};

mod list;
mod new;

pub fn menu<'a>() -> Menu<'a> {
    let mut m = Menu{
        name: "tags",
        about: "Add, modify, remove and list tags",
        author: "Andrej Dundović <andrej.dundovic@udruga-point.hr>",
        version: "0.1",
        subcommands: HashMap::new()
    };
    
    let menu_list = Subcommand {
        app: App::new("list")
            .about("List all tags"),
        f: &list::f
    };
    m.push_subcommand("list", menu_list);
    
    let menu_list = Subcommand {
        app: App::new("new")
            .about("Create a new tag")
            .arg(Arg::new("NAME")
                .help("Tag name")
                .required(true)
            ),
        f: &new::f
    };
    m.push_subcommand("new", menu_list);
    
    m
}

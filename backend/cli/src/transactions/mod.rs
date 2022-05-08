use clap::{Arg, App};
use std::collections::HashMap;

use crate::menu::{Menu, Subcommand};

pub fn menu<'a>() -> Menu<'a> {
    let mut m = Menu{
        name: "cat",
        about: "Add, modify, remove and list categories",
        author: "Andrej Dundović <andrej.dundovic@udruga-point.hr>",
        version: "0.1",
        subcommands: HashMap::new()
    };
    m
}

use std::collections::HashMap;
use clap::{Arg, App};

pub struct Subcommand<'a> {
    pub app: App<'a>,
    pub f: &'a dyn Fn(&clap::ArgMatches) -> ()
}

pub struct Menu<'a> {
    pub name: &'a str,
    pub about: &'a str,
    pub author: &'a str,
    pub version: &'a str,
    pub subcommands: HashMap<&'a str, Subcommand<'a>>,
}

impl<'a> Menu<'a> {
    pub fn push_subcommand(&mut self, name: &'a str, sub: Subcommand<'a>) -> () {
        self.subcommands.insert(name, sub);
    }
    pub fn generate(&self) -> App<'a> {

        let only_menus: Vec<App> =
            self.subcommands.values()
                    .map(|s| {s.app.clone()}).collect();

        App::new(self.name)
            .about(self.about)
            .version(self.version)
            .author(self.author)
            .subcommands(only_menus)
    }
    pub fn process(&self, args: &clap::ArgMatches) -> () {
        if let Some(ref args) = args.subcommand_matches(self.name) {
            for (k, v) in &self.subcommands {
                if let Some(ref ops) = args.subcommand_matches(k) {
                    (v.f)(ops);
                }
            }
        }
    }
}

pub fn main_menu<'a>() -> App<'a> {
    App::new("CLI for still unnamed app")
        .version("0.1")
        .author("Andrej Dundović <andrej.dundovic@udruga-point.hr>")
        .about("To interact with web app through command line interface")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::new("v")
            .short('v')
            .multiple_occurrences(true)
            .help("Sets the level of verbosity"))
}

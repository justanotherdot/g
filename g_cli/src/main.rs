extern crate clap;
extern crate g_lib;
use clap::{App, Arg, SubCommand};

fn main() {
    let _matches = App::new("g")
        .version("0.4.0")
        .author("Ryan James Spencer <spencer.ryanjames@gmail.com>")
        .about("The Haskell toolchain installer")
        .after_help(
            "g installs The Glorious Glasgow Haskell Compilation System,\n\
             enabling you to easily switch between various versions of the\n\
             compiler and keep them updated.",
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Turns on verbose output"),
        )
        .subcommand(
            SubCommand::with_name("install")
                .about("installs a specific GHC or cabal version")
                .arg(
                    Arg::with_name("cabal")
                        .short("c")
                        .long("cabal")
                        .help("install cabal instead of the default of GHC"),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("lists all current installations of GHC or cabal")
                .arg(
                    Arg::with_name("cabal")
                        .short("c")
                        .long("cabal")
                        .help("list cabal installations"),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("removes a specific GHC or cabal version")
                .arg(
                    Arg::with_name("cabal")
                        .short("c")
                        .long("cabal")
                        .help("removes a cabal installation instead of the default of GHC"),
                ),
        )
        .subcommand(
            SubCommand::with_name("switch")
                .about("switches to a specific GHC or cabal version")
                .arg(Arg::with_name("cabal").short("c").long("cabal").help(
                    "switches to a specific cabal installation instead of the default of GHC",
                )),
        )
        .get_matches();
    //println!("{:?}", matches);
}

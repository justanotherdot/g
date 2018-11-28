extern crate clap;
use clap::{Arg, App};

fn main() {
    let _matches = App::new("g")
                    .version("0.4.0")
                    .author("Ryan James Spencer <spencer.ryanjames@gmail.com>")
                    .about("The Haskell toolchain installer")
                    .after_help(
                        "g installs The Glorious Glasgow Haskell Compilation System,\n\
                        enabling you to easily switch between various versions of the\n\
                        compiler and keep them updated.")
                    .get_matches();

}

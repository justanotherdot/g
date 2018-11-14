const USAGE: &str = r#"
g 0.4.0
The Haskell toolchain installer

USAGE:
    g [OPTIONS] <SUBCOMMAND>

OPTIONS:
    --verbose, -v  Run with verbose output

SUBCOMMANDS:
    install        Install a version of GHC
    switch         Switch to an installed version of GHC
    list           List all installed versions of GHC

DISCUSSION:
    g installs The Glorious Glasgow Haskell Compilation System,
    enabling you to easily switch between various versions of the
    compiler and keep them updated.
"#;

fn display_usage() {
    println!("{}", USAGE);
}

fn main() {
    display_usage();
}

/// Take OS as string from `uname -m`
/// and convert it into appropriate deb
/// n.b. this function gets drastically changed when we start
/// dynamically building lookup tables for installations
#[allow(dead_code)]
fn os_to_target() {
}

/// Cleanup a tmp directory
/// This may be pointless now that this is a compiled program.
#[allow(dead_code)]
fn cleanup() {
}

#[allow(dead_code)]
fn ghc_verify_checksums() {
}

/// Stick downloaded ghc install in the right place
#[allow(dead_code)]
fn ghc_install() {
}

#[allow(dead_code)]
fn ghc_download_and_install() {
}

#[allow(dead_code)]
fn cabal_download_and_install() {
}

#[allow(dead_code)]
fn ghc_list_available_versions() {
}

#[allow(dead_code)]
fn ghc_switch_version() {
}

#[allow(dead_code)]
fn add_path_to_prefix() {
}

#[allow(dead_code)]
fn ghc_remove_version() {
}

#[allow(dead_code)]
fn ghc_switch_to_next_version() {
}

// New functionality stubs.
// might be nice to have these dep cache things
// on a struct and have these as methods off it.
// A dep cache could be built at program start
// and we could pass it around as we see fit.

#[allow(dead_code)]
fn build_dep_cache() {
}

#[allow(dead_code)]
fn write_dep_cache() {
}

#[allow(dead_code)]
fn read_dep_cache() {
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

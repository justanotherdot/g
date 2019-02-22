use std::env;
use std::process::{Command, Stdio};

#[allow(dead_code)]
const GHC_DOWNLOAD_BASE_URL: &str = "https://downloads.haskell.org/~ghc";

#[derive(Debug, Clone)]
struct UnsupportedOS;

impl std::fmt::Display for UnsupportedOS {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for UnsupportedOS {
    fn description(&self) -> &str {
        "this OS is not supported"
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

/// Take OS as string from `uname -m`
/// and convert it into appropriate deb
/// n.b. this function gets drastically changed when we start
/// dynamically building lookup tables for installations
#[allow(dead_code)]
fn os_to_target() -> Result<String, Box<std::error::Error>> {
    let uname_out = Command::new("uname").output()?;
    let uname_machine_out = Command::new("uname").arg("-m").output()?;

    let os = String::from_utf8(uname_out.stdout)?;
    let machine = String::from_utf8(uname_machine_out.stdout)?;

    // n.b. These are regexes, not sure if this ideal, now.
    // Instead, we should be basing everything around the cache:
    //   The cache gets populated at runtime, and passed into here for reference.
    //   It doens't matter if the cache population is from disk or from the web.
    //   Since you need internet to run this application, it makes sense the
    //   cache should be network
    match os.as_ref() {
        "darwin" => Ok(format!("{}-apple.darwin.+tar.xz$", machine)),
        "linux" => Ok(format!(
            "{}-deb[89]-linux|[^l]+linux-deb7)[^-]+tar.xz$",
            machine
        )),
        _ => Err(Box::new(UnsupportedOS)),
    }
}

/// Cleanup a tmp directory
/// This may be pointless now that this is a compiled program.
#[allow(dead_code)]
fn cleanup(tmp_dir: String) -> Result<(), Box<std::error::Error>> {
    println!("Cleaning up ... ");

    Command::new("rm")
        .args(&["-rf", tmp_dir.as_ref()])
        .output()?;

    Ok(())
}

fn shasum(filename: String) -> Result<String, Box<std::error::Error>> {
    let shasum_cmd = Command::new("shasum")
        .args(&["-a", "256", filename.as_ref()])
        .stdout(Stdio::piped())
        .spawn()?;
    let awk_cmd = Command::new("awk")
        .stdin(shasum_cmd.stdout.unwrap())
        .arg("{print $1}")
        .output()?;

    let shasum = String::from_utf8(awk_cmd.stdout)?;
    Ok(shasum)
}

#[allow(dead_code)]
// TODO These probably need to be osStrs
fn verify_checksums(
    local_filename: String,
    remote_filename: String,
) -> Result<bool, Box<std::error::Error>> {
    println!("Verifying checksums ... ");

    let local_sha = shasum(local_filename)?;
    let remote_sha = shasum(remote_filename)?;

    if local_sha.trim().is_empty() || remote_sha.trim().is_empty() {
        println!("One of the checksums is empty");
        println!("   local: {}", local_sha);
        println!("  remote: {}", remote_sha);
        return Ok(false);
    } else if local_sha != remote_sha {
        println!("Checksums do not match");
        println!("   local: {}", local_sha);
        println!("  remote: {}", remote_sha);
        Ok(false)
    } else {
        println!("Checksums match");
        println!("   local: {}", local_sha);
        println!("  remote: {}", remote_sha);
        Ok(local_sha == remote_sha)
    }
}

/// Stick downloaded ghc install in the right place
#[allow(dead_code)]
// TODO Should be osstr here.
fn ghc_install(filename: String) -> Result<(), Box<std::error::Error>> {
    println!("Unpacking {}", filename);

    Command::new("tar").args(&["xf", &filename]).output()?;

    Command::new("rm").arg(&filename).output()?;

    // XXX This may not be the only thing in the dir!
    let ls_out = Command::new("ls").output()?;

    // n.b. This pretends that all directory name strings are going
    // to be valid utf-8 but this may not be the case!
    // It's possible we could use from_utf8_lossy here
    // but that may accrue a bug downstream that wouldn't be ideal.
    let dir_name = String::from_utf8(ls_out.stdout)?;

    Command::new("cd").arg(&dir_name).output()?;

    // This should be a `Path`
    // TODO we'll need a `resolve_prefix` func
    // instead of directly accessing the G_PREFIX env var.
    let g_prefix = env::var("G_PREFIX")?;
    let prefix = format!("{:?}/{}", g_prefix, &dir_name);

    Command::new("./configure")
        .arg(format!("--prefix={}", prefix))
        .output()?;

    Command::new("make").arg("install").output()?;

    Ok(())
}

#[allow(dead_code)]
fn ghc_download_and_install() {}

#[allow(dead_code)]
fn cabal_download_and_install() {}

#[allow(dead_code)]
fn ghc_list_available_versions() {}

#[allow(dead_code)]
fn ghc_switch_version() {}

#[allow(dead_code)]
fn add_path_to_prefix() {}

#[allow(dead_code)]
fn ghc_remove_version() {}

#[allow(dead_code)]
fn ghc_switch_to_next_version() {}

// New functionality stubs.
// might be nice to have these dep cache things
// on a struct and have these as methods off it.
// A dep cache could be built at program start
// and we could pass it around as we see fit.

#[allow(dead_code)]
type URL = String;
#[allow(dead_code)]
type TargetName = String;

#[allow(dead_code)]
struct TargetCache(std::collections::HashMap<TargetName, URL>);

#[allow(dead_code)]
fn build_dep_cache() {}

#[allow(dead_code)]
fn write_dep_cache() {}

#[allow(dead_code)]
fn read_dep_cache() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

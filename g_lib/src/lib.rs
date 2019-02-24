extern crate regex;
extern crate reqwest;
extern crate select;

use regex::Regex;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashMap;
use std::env;
use std::error::Error;
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

impl Error for UnsupportedOS {
    fn description(&self) -> &str {
        "this OS is not supported"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

#[allow(dead_code)]
fn system_and_machine() -> Result<(String, String), Box<Error>> {
    let uname_out = Command::new("uname").output()?;
    let uname_machine_out = Command::new("uname").arg("-m").output()?;

    let system = String::from_utf8(uname_out.stdout)?;
    let machine = String::from_utf8(uname_machine_out.stdout)?;

    Ok((system.trim().to_owned(), machine.trim().to_owned()))
}

#[deprecated(since = "0.4.0", note = "please use `populate_metadata` instead")]
#[allow(dead_code)]
fn os_to_target() -> Result<String, Box<Error>> {
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
    match os.trim() {
        "Darwin" => Ok(format!("{}-apple.darwin.+tar.xz$", machine)),
        "Linux" => Ok(format!(
            "{}-deb[89]-linux|[^l]+linux-deb7)[^-]+tar.xz$",
            machine
        )),
        _ => Err(Box::new(UnsupportedOS)),
    }
}

/// Cleanup a tmp directory
/// This may be pointless now that this is a compiled program.
#[allow(dead_code)]
fn cleanup(tmp_dir: String) -> Result<(), Box<Error>> {
    println!("Cleaning up ... ");

    Command::new("rm")
        .args(&["-rf", tmp_dir.as_ref()])
        .output()?;

    Ok(())
}

fn shasum(filename: String) -> Result<String, Box<Error>> {
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
fn verify_checksums(local_filename: String, remote_filename: String) -> Result<bool, Box<Error>> {
    println!("Verifying checksums ... ");

    let local_sha = shasum(local_filename)?;
    let remote_sha = shasum(remote_filename)?;

    if local_sha.trim().is_empty() || remote_sha.trim().is_empty() {
        println!("One of the checksums is empty");
        println!("   local: {}", local_sha);
        println!("  remote: {}", remote_sha);
        Ok(false)
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
fn ghc_install(filename: String) -> Result<(), Box<Error>> {
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
fn ghc_download_and_install() {
    // This will download from the given TargetCache URL for GHC
    // We firstly grab the SHA256SUMS file, get our target's sha256
    // Which we will later use to verify post copy_to from reqwest
    // Lastly we'll unpack and install the file to the PREFIX location

    // // This is mostly the 'download' step
    // // TODO Remove.
    // let target_url2 = format!("{}/{}", &GHC_DOWNLOAD_BASE_URL, "7.10.1");

    // let inner_resp = reqwest::get(&target_url2)?.text()?;
    // let inner_document = Document::from(inner_resp.as_ref());
    // // TODO Same thing here for 'SHA256SUMS'
    // let re2 = Regex::new(
    //     format!(
    //         "ghc-{}-{}.+{}.+xz$",
    //         "7.10.1",
    //         machine,
    //         system.to_lowercase()
    //     )
    //     .as_ref(),
    // )?;
    // let items = inner_document
    //     .find(Name("a"))
    //     .filter(|t| re2.is_match(&t.text()));
    // for item in items {
    //     println!("{:#?}", item);
    // }
}

#[allow(dead_code)]
fn cabal_download_and_install() {
    // Has a different URL for installation which
    // should be reflected in the TargetCache
}

#[allow(dead_code)]
fn ghc_list_available_versions() {
    // `ls` the install prefix dir
}

#[allow(dead_code)]
fn ghc_switch_version() {
    // Switch symlinks around.
}

#[allow(dead_code)]
fn add_prefix_to_path() {
    // May be useless now; was just a way to automatically add
    // the PREFIX to the PATH for execution's sake.
}

#[allow(dead_code)]
fn ghc_remove_version() {
    // rm -rvf a given version.
}

#[allow(dead_code)]
fn ghc_switch_to_next_version() {
    // ghc_switch_version but by finding next highest target number?
}

#[allow(dead_code)]
#[derive(Default)]
pub struct TargetCache(HashMap<String, String>);

impl TargetCache {
    #[allow(dead_code)]
    pub fn new() -> Self {
        TargetCache(HashMap::new())
    }

    #[allow(dead_code)]
    pub fn build(&mut self) -> Result<(), Box<Error>> {
        let resp = reqwest::get(GHC_DOWNLOAD_BASE_URL)?.text()?;
        let document = Document::from(resp.as_ref());
        let re = Regex::new(r"(latest|master|^[0-9]+)")?;
        let targets = document.find(Name("a")).filter(|t| re.is_match(&t.text()));

        // let (system, machine) = system_and_machine()?;

        targets.for_each(|target| {
            let target_url = format!("{}/{}", &GHC_DOWNLOAD_BASE_URL, target.text());
            self.0.insert(target.text().clone(), target_url);
        });

        println!("{:#?}", self.0);

        Ok(())
    }

    #[allow(dead_code)]
    fn flush() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn target_cache_builds() {
        match TargetCache::new().build() {
            Ok(x) => println!("{:#?}", x),
            Err(e) => println!("{:?}", e),
        }
    }
}

use std::env;
use std::error::Error;
use std::process::{Command, Stdio};

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

// TODO I think it might be worth having a `run` wrapper around Command

// TODO This should be a single function per concern.
// `system` and `machine` rather than `system_and_machine`
#[allow(dead_code)]
pub fn system() -> Result<String, Box<Error>> {
    let uname_out = Command::new("uname").output()?;
    let system = String::from_utf8(uname_out.stdout)?;
    // TODO These ought to be proper types.
    // e.g. Linux, Mac, Windows, or Error on UnsupportedOs
    // Machine is tricky, i383 or x86_64 pretty much
    Ok(system.trim().to_owned())
}

#[allow(dead_code)]
pub fn machine() -> Result<String, Box<Error>> {
    let uname_machine_out = Command::new("uname").arg("-m").output()?;
    let machine = String::from_utf8(uname_machine_out.stdout)?;
    // TODO These ought to be proper types.
    // e.g. Linux, Mac, Windows, or Error on UnsupportedOs
    // Machine is tricky, i383 or x86_64 pretty much
    Ok(machine.trim().to_owned())
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
fn verify_checksums(local_filename: String, remote_sha: String) -> Result<bool, Box<Error>> {
    println!("Verifying checksums ... ");

    let local_sha = shasum(local_filename)?;

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
    // let (system, machine) = system_and_machine()?;
    //
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
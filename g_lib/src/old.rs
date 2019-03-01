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

// TODO: I think it might be worth having a `run` wrapper around Command

// TODO: `system` and `machine` ought to probably go in a sys_info module

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

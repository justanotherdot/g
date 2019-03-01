use super::old::{machine, system};
use super::target_cache::{Target, TargetCache};
use std::env;
use std::error::Error;
use std::process::{Command, Stdio};
use target_cache::TargetTy;

#[allow(dead_code)]
#[derive(Debug, Default)]
struct GMetadata {
    machine: String,
    system: String,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct G {
    metadata: GMetadata,
    target_cache: TargetCache,
    target: Option<Target>,
}

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
enum Machine {
    Linux,
    Windows,
    Mac,
}

#[allow(dead_code, non_camel_case_types)]
enum System {
    x86_64,
    i368,
}

impl G {
    #[allow(dead_code)]
    pub fn new() -> Self {
        // TODO Use the types, luke.
        let machine = match machine() {
            Ok(m) => m,
            Err(_) => "".to_owned(),
        };

        // TODO Use the types, luke.
        let system = match system() {
            Ok(s) => s,
            Err(_) => "".to_owned(),
        };

        let metadata = GMetadata { machine, system };
        let target_cache = TargetCache::new();
        let target = None;

        G {
            metadata,
            target_cache,
            target,
        }
    }

    // TODO It may make sense to drop this functionality into `new`
    // as new instances should probably not exist without being passed
    // a target.
    //
    // At least then we can stop earlier in the chain if someone has passed
    // incorrect args.
    #[allow(dead_code)]
    pub fn target(self, target: Target) -> Self {
        G {
            target: Some(target),
            ..self
        }
    }

    #[allow(dead_code)]
    pub fn download_and_install(self, filename: String) -> Result<Self, Box<Error>> {
        // DOWNLOAD
        //
        // GHC download ...
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

        // INSTALL
        match self
            .target
            .clone()
            .expect("could not find target in install phase")
        {
            Target {
                target_ty: TargetTy::GHC,
                version: _,
            } => {
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
            }
            _ => panic!("unimplemented case: cabal installation"),
        }
        Ok(self)
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

    // TODO: This could probably be killed off...
    #[allow(dead_code)]
    fn cleanup(tmp_dir: String) -> Result<(), Box<Error>> {
        println!("Cleaning up ... ");

        Command::new("rm")
            .args(&["-rf", tmp_dir.as_ref()])
            .output()?;

        Ok(())
    }
    // TODO: Might make sense to put SHA stuff into its own module.

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

        let local_sha = Self::shasum(local_filename)?;

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

    #[allow(dead_code)]
    pub fn switch() {}

    #[allow(dead_code)]
    pub fn remove() {}

    #[allow(dead_code)]
    pub fn adjust_shell_path() {}

    #[allow(dead_code)]
    pub fn list_installed() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn g_instantiates() {
        let expected_target = Target::new("ghc", "8.10.2").unwrap();
        let g = G::new().target(expected_target.clone());
        assert_eq!(g.target, Some(expected_target));
    }
}

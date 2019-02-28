use super::old::{machine, system};
use super::target_cache::{Target, TargetCache};
use std::env;
use std::error::Error;
use std::process::Command;
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

impl G {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let machine = match machine() {
            Ok(m) => m,
            Err(_) => "".to_owned(),
        };

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
    pub fn download() {}

    // TODO It may make sense to combine download and install into
    // download_and_install to make the step atomic, rather than
    // it failing in case someone forgets to download first.
    #[allow(dead_code)]
    pub fn install(self, filename: String) -> Result<Self, Box<Error>> {
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

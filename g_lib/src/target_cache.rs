use regex::Regex;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashMap;
use std::error::Error;

#[allow(dead_code)]
const GHC_DOWNLOAD_BASE_URL: &str = "https://downloads.haskell.org/~ghc";

#[allow(dead_code)]
#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum TargetTy {
    GHC,
    Cabal,
}

#[allow(dead_code)]
#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct Target {
    pub target_ty: TargetTy,
    pub version: String,
}

impl Target {
    #[allow(dead_code)]
    pub fn new(target_ty_str: &str, version_str: &str) -> Option<Self> {
        let version = version_str.to_string();
        match target_ty_str.trim().to_lowercase().as_ref() {
            "ghc" => {
                let target_ty = TargetTy::GHC;
                Some(Target { target_ty, version })
            }
            "cabal" => {
                let target_ty = TargetTy::Cabal;
                Some(Target { target_ty, version })
            }
            _ => None,
        }
    }
}

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct TargetCache(HashMap<Target, String>);

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

        targets.for_each(|target| {
            let target_url = format!("{}/{}", &GHC_DOWNLOAD_BASE_URL, target.text());
            let target = Target {
                target_ty: TargetTy::GHC,
                version: target.text().clone(),
            };
            self.0.insert(target, target_url);
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

    // #[test]
    // fn target_cache_builds() {
    //     match TargetCache::new().build() {
    //         Ok(x) => println!("{:#?}", x),
    //         Err(e) => println!("{:?}", e),
    //     }
    // }

    #[test]
    fn target_ctor_succeeds_with_ghc_strs() {
        let names = vec![" ghc", "ghc ", "ghc", "GHC"];
        names.iter().for_each(|name| {
            let target_ty = TargetTy::GHC;
            let version = "8.10.2";
            let t = Target::new(name, version);
            assert_eq!(
                t,
                Some(Target {
                    target_ty,
                    version: version.to_string()
                })
            )
        });
    }

    #[test]
    fn target_ctor_succeeds_with_cabal_strs() {
        let names = vec![" cabal", "cabal ", "cabal", "CABAL"];
        names.iter().for_each(|name| {
            let target_ty = TargetTy::Cabal;
            let version = "8.10.2";
            let t = Target::new(name, version);
            assert_eq!(
                t,
                Some(Target {
                    target_ty,
                    version: version.to_string()
                })
            )
        });
    }
}

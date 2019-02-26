use super::old::{machine, system};
use super::target_cache::{Target, TargetCache};

#[allow(dead_code)]
#[derive(Debug)]
struct GMetadata {
    machine: String,
    system: String,
}

#[allow(dead_code)]
#[derive(Debug)]
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn g_instantiates() {
        let g = G::new();
        assert_eq!(g.metadata.machine, "x86_64");
    }
}

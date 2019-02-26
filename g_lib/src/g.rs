use super::old::{machine, system};

#[allow(dead_code)]
struct GMetadata {
    machine: String,
    system: String,
}

#[allow(dead_code)]
struct G {
    metadata: GMetadata,
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

        G { metadata }
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

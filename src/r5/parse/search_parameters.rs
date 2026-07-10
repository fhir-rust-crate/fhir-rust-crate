// Namespace conveniences

pub static DIR: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| crate::r5::parse::DIR.join("search_parameters"));

pub static DEFINITIONS_FILE: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| crate::DEFINITIONS_DIR.join("search-parameters.json"));

// Submodules

pub mod base {
    pub mod base;
}
pub use base::base::Base;

pub mod bundle {
    pub mod bundle;
}
pub use bundle::bundle::Bundle;

pub mod component {
    pub mod component;
}
pub use component::component::Component;

pub mod differential {
    pub mod differential;
}
pub use differential::differential::Differential;

pub mod element {
    pub mod element;
    pub mod element_into_rust_struct_attribute;
}
pub use element::element::Element;
pub use element::element_into_rust_struct_attribute::element_into_rust_struct_attribute;

pub mod entry {
    pub mod entry;
}
pub use entry::entry::Entry;

pub mod resource {
    pub mod resource;
}
pub use resource::resource::Resource;

pub mod snapshot {
    pub mod snapshot;
}
pub use snapshot::snapshot::Snapshot;

#[cfg(test)]
mod tests {
    use super::*;
    type T = crate::r5::parse::search_parameters::Bundle;

    #[test]
    fn test_serde_json_from_reader() {
        let file = std::fs::File::open(&*DEFINITIONS_FILE).unwrap();
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}

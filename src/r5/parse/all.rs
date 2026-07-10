// Namespace conveniences

pub static DIR: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| crate::r5::parse::DIR.join("all"));

pub static DEFINITIONS_FILE: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| crate::DEFINITIONS_DIR.join("all.json"));

// Submodules

pub mod additional {
    pub mod additional;
}
pub use additional::additional::Additional;

pub mod additional_attribute {
    pub mod additional_attribute;
}
pub use additional_attribute::additional_attribute::AdditionalAttribute;

pub mod binding {
    pub mod binding;
}
pub use binding::binding::Binding;

pub mod codeable_concept {
    pub mod codeable_concept;
}
pub use codeable_concept::codeable_concept::CodeableConcept;

pub mod coding {
    pub mod coding;
}
pub use coding::coding::Coding;

pub mod constraint {
    pub mod constraint;
}
pub use constraint::constraint::Constraint;

pub mod contact {
    pub mod contact;
}
pub use contact::contact::Contact;

pub mod depends_on {
    pub mod depends_on;
}
pub use depends_on::depends_on::DependsOn;

pub mod example {
    pub mod example;
}
pub use example::example::Example;

pub mod extension {
    pub mod extension;
}
pub use extension::extension::Extension;

pub mod element_type {
    pub mod element_type;
}
pub use element_type::element_type::ElementType;

pub mod identifier {
    pub mod identifier;
}
pub use identifier::identifier::Identifier;

pub mod jurisdiction {
    pub mod jurisdiction;
}
pub use jurisdiction::jurisdiction::Jurisdiction;

pub mod meta {
    pub mod meta;
}
pub use meta::meta::Meta;

pub mod period {
    pub mod period;
}
pub use period::period::Period;

pub mod property {
    pub mod property;
}
pub use property::property::Property;

pub mod quantity {
    pub mod quantity;
}
pub use quantity::quantity::Quantity;

pub mod range {
    pub mod range;
}
pub use range::range::Range;

pub mod related_artifact {
    pub mod related_artifact;
}
pub use related_artifact::related_artifact::RelatedArtifact;

pub mod target {
    pub mod target;
}
pub use target::target::Target;

pub mod topic {
    pub mod topic;
}
pub use topic::topic::Topic;

pub mod r#type {
    pub mod r#type;
}
pub use r#type::r#type::r#Type;

pub mod underscore_code {
    pub mod underscore_code;
}
pub use underscore_code::underscore_code::UnderscoreCode;

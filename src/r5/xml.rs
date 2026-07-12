//! FHIR XML serialization (feature `xml`).
//!
//! FHIR XML differs from JSON: a primitive is `<field value="…"/>`, an element's
//! `id` (and an extension's `url`) are XML attributes, complex elements nest, and
//! repeating elements are simply repeated. This module converts between a
//! resource and FHIR XML by way of `serde_json::Value`, driven by the
//! [`meta`](crate::r5::meta) table (to know an element's type and cardinality
//! when reading string attributes back into typed JSON).
//!
//! ```
//! use fhir::r5::resources::Patient;
//! use fhir::r5::xml::{to_xml, from_xml};
//!
//! let patient: Patient = serde_json::from_value(serde_json::json!({
//!     "resourceType": "Patient", "id": "pat-1", "active": true,
//!     "name": [{ "family": "Chalmers", "given": ["Peter"] }]
//! })).unwrap();
//!
//! let xml = to_xml(&patient, "Patient");
//! assert!(xml.contains("<active value=\"true\"/>"));
//!
//! let back: Patient = from_xml(&xml).unwrap();
//! assert_eq!(back, patient);
//! ```
//!
//! Scope: this handles primitives, complex elements, arrays, element `id` and
//! extension `url` attributes, and `value[x]` choices. Not yet handled (round-
//! trips through JSON but not to canonical FHIR XML): primitive extensions
//! (`_field`), embedded XHTML `Narrative.div`, and contained resources.

use ::serde::Serialize;
use ::serde::de::DeserializeOwned;
use ::serde_json::{Map, Value};

use crate::r5::meta;

const FHIR_NS: &str = "http://hl7.org/fhir";

/// FHIR primitive type codes whose JSON form is a number.
fn is_number_type(code: &str) -> bool {
    matches!(code, "integer" | "decimal" | "positiveInt" | "unsignedInt")
}

// ---- Writing -----------------------------------------------------------------

/// Serialize a resource to a FHIR XML string.
#[must_use]
pub fn to_xml<T: Serialize>(resource: &T, resource_type: &str) -> String {
    let value = ::serde_json::to_value(resource).unwrap_or(Value::Null);
    let empty = Map::new();
    let obj = value.as_object().unwrap_or(&empty);
    let mut out = String::new();
    write_complex(&mut out, resource_type, obj, true);
    out
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn scalar_str(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Bool(b) => Some(b.to_string()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

fn write_complex(out: &mut String, name: &str, obj: &Map<String, Value>, is_resource_root: bool) {
    // Attributes: element id (except at the resource root, where id is a child),
    // and an extension's url.
    let mut consumed: Vec<&str> = Vec::new();
    out.push('<');
    out.push_str(name);
    if is_resource_root {
        out.push_str(" xmlns=\"");
        out.push_str(FHIR_NS);
        out.push('"');
    }
    if (name == "extension" || name == "modifierExtension")
        && let Some(Value::String(url)) = obj.get("url")
    {
        out.push_str(" url=\"");
        out.push_str(&escape(url));
        out.push('"');
        consumed.push("url");
    }
    if !is_resource_root
        && let Some(Value::String(id)) = obj.get("id")
    {
        out.push_str(" id=\"");
        out.push_str(&escape(id));
        out.push('"');
        consumed.push("id");
    }

    let children: Vec<(&String, &Value)> = obj
        .iter()
        .filter(|(k, _)| k.as_str() != "resourceType" && !consumed.contains(&k.as_str()))
        .collect();
    if children.is_empty() {
        out.push_str("/>");
        return;
    }
    out.push('>');
    for (key, value) in children {
        write_child(out, key, value);
    }
    out.push_str("</");
    out.push_str(name);
    out.push('>');
}

fn write_child(out: &mut String, key: &str, value: &Value) {
    match value {
        Value::Null => {}
        Value::Array(items) => {
            for item in items {
                write_child(out, key, item);
            }
        }
        Value::Object(o) => write_complex(out, key, o, false),
        scalar => {
            if let Some(s) = scalar_str(scalar) {
                out.push('<');
                out.push_str(key);
                out.push_str(" value=\"");
                out.push_str(&escape(&s));
                out.push_str("\"/>");
            }
        }
    }
}

// ---- Reading -----------------------------------------------------------------

/// The error type for the XML reader.
#[derive(Debug)]
pub enum XmlError {
    /// Malformed XML.
    Xml(quick_xml::Error),
    /// The parsed value did not deserialize into the target type.
    Deserialize(String),
    /// A structural problem (e.g. no root element).
    Custom(String),
}

impl std::fmt::Display for XmlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XmlError::Xml(e) => write!(f, "XML error: {e}"),
            XmlError::Deserialize(m) | XmlError::Custom(m) => f.write_str(m),
        }
    }
}

impl std::error::Error for XmlError {}

impl From<quick_xml::Error> for XmlError {
    fn from(e: quick_xml::Error) -> Self {
        XmlError::Xml(e)
    }
}

/// Parse a FHIR XML string into a resource.
///
/// # Errors
/// Returns an error if the XML is malformed or does not deserialize into `T`.
pub fn from_xml<T: DeserializeOwned>(xml: &str) -> Result<T, XmlError> {
    let value = xml_to_value(xml)?;
    ::serde_json::from_value(value).map_err(|e| XmlError::Deserialize(e.to_string()))
}

/// Parse FHIR XML into a `serde_json::Value` (a resource object).
///
/// # Errors
/// Returns an error if the XML is malformed.
pub fn xml_to_value(xml: &str) -> Result<Value, XmlError> {
    use quick_xml::events::Event;
    let mut reader = quick_xml::Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    // Find the root start element (the resourceType).
    loop {
        match reader.read_event()? {
            Event::Start(e) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                let (id, _url, _val) = read_attrs(&e);
                let mut obj = Map::new();
                obj.insert("resourceType".to_string(), Value::String(name.clone()));
                if let Some(id) = id {
                    obj.insert("id".to_string(), Value::String(id));
                }
                read_children(&mut reader, &name, &name, &mut obj)?;
                return Ok(Value::Object(obj));
            }
            Event::Eof => return Err(XmlError::Custom("no root element".into())),
            _ => {}
        }
    }
}

/// Read the `value`, `id`, and `url` attributes of an element.
fn read_attrs(e: &quick_xml::events::BytesStart) -> (Option<String>, Option<String>, Option<String>) {
    let (mut id, mut url, mut val) = (None, None, None);
    for attr in e.attributes().flatten() {
        let key = attr.key.as_ref();
        let value = String::from_utf8_lossy(&attr.value).into_owned();
        match key {
            b"id" => id = Some(value),
            b"url" => url = Some(value),
            b"value" => val = Some(value),
            _ => {}
        }
    }
    (id, url, val)
}

/// Read child elements of a complex element into `obj`, until the matching end
/// tag. `elem_name` is the current element's tag; `context` is its FHIR type
/// path (for metadata lookups).
fn read_children(
    reader: &mut quick_xml::Reader<&[u8]>,
    elem_name: &str,
    context: &str,
    obj: &mut Map<String, Value>,
) -> Result<(), XmlError> {
    use quick_xml::events::Event;
    loop {
        match reader.read_event()? {
            Event::Start(e) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                let child = read_element(reader, &name, context, &e, false)?;
                insert_child(obj, &name, context, child);
            }
            Event::Empty(e) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                let child = read_element(reader, &name, context, &e, true)?;
                insert_child(obj, &name, context, child);
            }
            Event::End(e) => {
                if String::from_utf8_lossy(e.name().as_ref()) == elem_name {
                    return Ok(());
                }
            }
            Event::Eof => return Ok(()),
            _ => {}
        }
    }
}

/// Read a single child element into a `Value`.
fn read_element(
    reader: &mut quick_xml::Reader<&[u8]>,
    name: &str,
    parent_context: &str,
    start: &quick_xml::events::BytesStart,
    is_empty: bool,
) -> Result<Value, XmlError> {
    let (id, url, val) = read_attrs(start);
    let path = format!("{parent_context}.{name}");
    let el = element_meta(&path, parent_context, name);
    let type_code = el.as_ref().and_then(|e| e.types.first().map(|t| t.code)).unwrap_or("");

    // A leaf primitive: a `value` attribute and (when Empty) no children.
    if is_empty {
        if let Some(v) = val {
            return Ok(coerce(&v, type_code));
        }
        // Empty element with only id/url (e.g. an extension with a nested value
        // that was itself empty) — represent as an object.
        let mut obj = Map::new();
        if let Some(id) = id {
            obj.insert("id".to_string(), Value::String(id));
        }
        if let Some(url) = url {
            obj.insert("url".to_string(), Value::String(url));
        }
        return Ok(Value::Object(obj));
    }

    // A complex element: gather id/url attributes plus children.
    let mut obj = Map::new();
    if let Some(id) = id {
        obj.insert("id".to_string(), Value::String(id));
    }
    if let Some(url) = url {
        obj.insert("url".to_string(), Value::String(url));
    }
    // The child context: a named datatype switches context to that type; a
    // backbone (or unknown) keeps the path.
    let child_context = if is_datatype(type_code) { type_code.to_string() } else { path };
    read_children(reader, name, &child_context, &mut obj)?;
    Ok(Value::Object(obj))
}

/// Insert a child value into `obj`, wrapping in an array when the element is a
/// repeating (`is_multiple`) element or already present.
fn insert_child(obj: &mut Map<String, Value>, name: &str, context: &str, child: Value) {
    let multiple = element_meta(&format!("{context}.{name}"), context, name)
        .is_some_and(meta::ElementMeta::is_multiple);
    match obj.get_mut(name) {
        Some(Value::Array(arr)) => arr.push(child),
        Some(existing) => {
            let prev = existing.take();
            *existing = Value::Array(vec![prev, child]);
        }
        None => {
            if multiple {
                obj.insert(name.to_string(), Value::Array(vec![child]));
            } else {
                obj.insert(name.to_string(), child);
            }
        }
    }
}

/// Look up element metadata, resolving `value[x]` choice keys (e.g.
/// `valueString`) to the choice element.
fn element_meta(path: &str, context: &str, name: &str) -> Option<&'static meta::ElementMeta> {
    if let Some(e) = meta::element(path) {
        return Some(e);
    }
    // A choice variant: find `<context>.<base>[x]` whose name prefixes `name`.
    meta::elements_of(context).find(|e| {
        e.path.ends_with("[x]") && {
            let base = &e.path[context.len() + 1..e.path.len() - 3];
            name.len() > base.len()
                && name.starts_with(base)
                && name[base.len()..].chars().next().is_some_and(char::is_uppercase)
        }
    })
}

/// Whether a type code names a complex datatype (vs a primitive or backbone).
fn is_datatype(code: &str) -> bool {
    !code.is_empty()
        && code.chars().next().is_some_and(char::is_uppercase)
        && code != "BackboneElement"
        && code != "Element"
}

/// Coerce a FHIR XML `value` attribute (always a string) into the JSON type the
/// element's primitive type implies.
fn coerce(value: &str, type_code: &str) -> Value {
    if type_code == "boolean" && let Ok(b) = value.parse::<bool>() {
        return Value::Bool(b);
    }
    if is_number_type(type_code) && let Ok(n) = value.parse::<::serde_json::Number>() {
        return Value::Number(n);
    }
    Value::String(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r5::resources::{Observation, Patient};

    fn roundtrip_patient(json: ::serde_json::Value) {
        let patient: Patient = ::serde_json::from_value(json).unwrap();
        let xml = to_xml(&patient, "Patient");
        let back: Patient = from_xml(&xml).unwrap_or_else(|e| panic!("from_xml failed: {e}\n{xml}"));
        assert_eq!(back, patient, "\nXML was:\n{xml}");
    }

    #[test]
    fn patient_roundtrips() {
        roundtrip_patient(::serde_json::json!({
            "resourceType": "Patient",
            "id": "pat-1",
            "active": true,
            "gender": "male",
            "birthDate": "1970-03-25",
            "name": [{ "family": "Chalmers", "given": ["Peter", "James"] }],
            "telecom": [{ "system": "phone", "value": "555" }]
        }));
    }

    #[test]
    fn primitive_written_as_value_attribute() {
        let patient: Patient = ::serde_json::from_value(::serde_json::json!({
            "resourceType": "Patient", "active": true
        }))
        .unwrap();
        let xml = to_xml(&patient, "Patient");
        assert!(xml.contains("<active value=\"true\"/>"), "{xml}");
        assert!(xml.contains("xmlns=\"http://hl7.org/fhir\""), "{xml}");
    }

    #[test]
    fn choice_element_roundtrips() {
        // Observation.value[x] — a choice — plus a required 1..1 code.
        let obs: Observation = ::serde_json::from_value(::serde_json::json!({
            "resourceType": "Observation",
            "status": "final",
            "code": { "text": "temp" },
            "valueString": "warm"
        }))
        .unwrap();
        let xml = to_xml(&obs, "Observation");
        let back: Observation = from_xml(&xml).unwrap_or_else(|e| panic!("{e}\n{xml}"));
        assert_eq!(back, obs, "\nXML was:\n{xml}");
    }
}

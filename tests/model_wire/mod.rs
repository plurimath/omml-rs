//! Writes a parsed model in the tagged wire form that plurimath-testsuite
//! `scripts/generate-xml-stage-cases.rb` records for the omml gem:
//! `{node: <Ruby class>, fields: [[name, value], ...]}` with the declared
//! attributes that are set, in declaration order, plus `@element_order`
//! where the model keeps the order of its XML children.
//!
//! The model is read through its `model-serde` JSON. Class names, attribute
//! order and types, and XML element names come from
//! `model/runtime-graph-v1.json`, the graph the models are generated from.
//!
//! Some Ruby state never reaches `to_xml` and is not kept by this crate, so
//! the expected wire is normalized before the comparison
//! ([`Graph::normalize`]); the replay report counts the rows each rule
//! touched:
//! - `unwritten_element_order`: Lutaml records element_order on every model
//!   read from XML, but writes it back only for ordered or mixed models.
//! - `ordered_whitespace_text`: on ordered models that are not mixed,
//!   element_order also lists the whitespace-only text between elements;
//!   `to_xml` does not write it and the reader drops it.
//! - `unread_element_order_entry`: element_order also lists a child element
//!   no attribute read (an `m:lastRenderedPageBreak` where the model maps
//!   `w:lastRenderedPageBreak`). An Element entry beyond the number of values
//!   its attribute holds is dropped.
//! - `unrendered_default_value`: an attribute explicitly set to its declared
//!   default (`m:val="on"`) is not `using_default?` in Ruby, but the mapping
//!   never renders the default, so `to_xml` writes the same bytes as when it
//!   is absent. The reader fills the default in, so both sides drop a field
//!   that holds its unrendered default.
//! - `ascii_empty_string`: an empty Ruby string can carry the US-ASCII
//!   encoding; that is the same value as "".
use serde_json::{json, Value};
use std::collections::{BTreeSet, HashMap};

enum Kind {
    Model(String),
    Scalar(String),
}

struct Attribute {
    name: String,
    kind: Kind,
    /// The declared default when the mapping never renders it.
    default: Option<Value>,
}

struct Model {
    name: String,
    attributes: Vec<Attribute>,
    /// Attribute -> XML element local name, first mapping wins.
    names: Vec<(String, String)>,
    keeps_order: bool,
    mixed: bool,
}

pub struct Graph {
    models: HashMap<String, Model>,
    by_name: HashMap<String, String>,
}

fn element_names(rule: &Value, names: &mut Vec<(String, String)>) {
    if rule["kind"] == "element" {
        if let (Some(to), Some(name)) = (rule["to"].as_str(), rule["name"][0].as_str()) {
            if !names.iter().any(|(attribute, _)| attribute == to) {
                names.push((to.to_owned(), name.to_owned()));
            }
        }
    }
    for member in rule["members"].as_array().into_iter().flatten() {
        element_names(member, names);
    }
}

fn order_entry(pairs: &[(&str, &str)]) -> Value {
    let pairs: Vec<Value> = pairs
        .iter()
        .map(|(key, value)| json!([{ "str": key }, { "str": value }]))
        .collect();
    json!({ "hash": pairs })
}

fn scalar(base: &str, value: &Value) -> Value {
    match (base, value) {
        (_, Value::Bool(b)) => json!({ "bool": b }),
        ("integer", Value::String(s)) => json!({ "int": s }),
        ("integer", Value::Number(n)) => json!({ "int": n.to_string() }),
        ("decimal", Value::String(s)) => json!({ "decimal": s }),
        ("decimal", Value::Number(n)) => json!({ "decimal": n.to_string() }),
        (_, Value::String(s)) => json!({ "str": s }),
        _ => panic!("no tagged form for a {base} value {value}"),
    }
}

impl Graph {
    pub fn load(path: &str) -> Self {
        let graph: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        let value_types = &graph["value_types"];
        let mut models = HashMap::new();
        let mut by_name = HashMap::new();
        for (id, model) in graph["models"].as_object().unwrap() {
            let xml = &model["mappings"]["xml"];
            let rendered = |name: &str| {
                xml["rules"].as_array().is_some_and(|rules| {
                    rules
                        .iter()
                        .any(|rule| rule["to"] == name && rule["render"]["default"] == true)
                })
            };
            let attributes = model["attributes"]
                .as_array()
                .unwrap()
                .iter()
                .map(|attribute| {
                    let name = attribute["name"].as_str().unwrap().to_owned();
                    let kind = &attribute["type"];
                    let kind = match kind["kind"].as_str().unwrap() {
                        "model" => Kind::Model(kind["id"].as_str().unwrap().to_owned()),
                        "value" => Kind::Scalar(
                            value_types[kind["id"].as_str().unwrap()]["base"]
                                .as_str()
                                .unwrap_or("string")
                                .to_owned(),
                        ),
                        _ => Kind::Scalar(kind["name"].as_str().unwrap_or("string").to_owned()),
                    };
                    let default = (attribute["default"]["present"] == true
                        && !rendered(name.as_str()))
                    .then(|| attribute["default"]["value"].clone());
                    Attribute {
                        name,
                        kind,
                        default,
                    }
                })
                .collect();
            let mut names = Vec::new();
            for rule in xml["rules"].as_array().into_iter().flatten() {
                element_names(rule, &mut names);
            }
            for rule in xml["sequence"].as_array().into_iter().flatten() {
                element_names(rule, &mut names);
            }
            let name = model["model"].as_str().unwrap().to_owned();
            by_name.insert(name.clone(), id.clone());
            models.insert(
                id.clone(),
                Model {
                    name,
                    attributes,
                    names,
                    keeps_order: xml["ordered"] == true || xml["mixed"] == true,
                    mixed: xml["mixed"] == true,
                },
            );
        }
        Self { models, by_name }
    }

    fn tag(&self, kind: &Kind, value: &Value) -> Value {
        match value {
            Value::Null => json!({ "nil": true }),
            Value::Array(items) => {
                let items: Vec<Value> = items.iter().map(|item| self.tag(kind, item)).collect();
                json!({ "array": items })
            }
            _ => match kind {
                Kind::Model(id) => self.wire(id, value),
                Kind::Scalar(base) => scalar(base, value),
            },
        }
    }

    /// The Ruby wire of the model `id` whose `model-serde` JSON is `value`.
    pub fn wire(&self, id: &str, value: &Value) -> Value {
        let model = self
            .models
            .get(id)
            .unwrap_or_else(|| panic!("model {id} is not in the graph"));
        let object = value
            .as_object()
            .unwrap_or_else(|| panic!("{}: {value} is not an object", model.name));
        let mut fields = Vec::new();
        for attribute in &model.attributes {
            let Some(field) = object.get(&attribute.name) else {
                continue;
            };
            // The reader fills a collection with no element in the input
            // with [], which is the attribute's default in Ruby too.
            if field.as_array().is_some_and(Vec::is_empty)
                || attribute.default.as_ref() == Some(field)
            {
                continue;
            }
            fields.push(json!([attribute.name, self.tag(&attribute.kind, field)]));
        }
        let tokens = object.get("__lml_content").and_then(Value::as_array);
        if let (true, Some(tokens)) = (model.keeps_order, tokens) {
            let mut order = Vec::new();
            for token in tokens {
                match token["kind"].as_str().unwrap() {
                    "element" => {
                        let attribute = token["attribute"].as_str().unwrap();
                        let name = model
                            .names
                            .iter()
                            .find(|(known, _)| known == attribute)
                            .map(|(_, name)| name.as_str())
                            .unwrap_or_else(|| {
                                panic!("{}: no element name for {attribute}", model.name)
                            });
                        order.push(order_entry(&[("type", "Element"), ("name", name)]));
                    }
                    "text" => {
                        let text = match token.get("literal") {
                            Some(literal) => literal.as_str().unwrap(),
                            None => {
                                let member = &object[token["attribute"].as_str().unwrap()];
                                let index = token["index"].as_u64().unwrap() as usize;
                                match member {
                                    Value::Array(items) => items[index].as_str().unwrap(),
                                    other => other.as_str().unwrap(),
                                }
                            }
                        };
                        order.push(order_entry(&[
                            ("type", "Text"),
                            ("name", "text"),
                            ("text", text),
                        ]));
                    }
                    "attribute" => {}
                    other => panic!("{}: no wire form for a {other} token", model.name),
                }
            }
            if !order.is_empty() {
                fields.push(json!(["@element_order", { "array": order }]));
            }
        }
        json!({ "node": model.name, "fields": fields })
    }

    /// The expected wire with the Ruby-only state described above removed;
    /// `used` collects the names of the rules applied.
    pub fn normalize(&self, value: &Value, used: &mut BTreeSet<&'static str>) -> Value {
        if value["encoding"] == "US-ASCII" && value["bytes"] == "" {
            used.insert("ascii_empty_string");
            return json!({ "str": "" });
        }
        if let Some(items) = value.get("array").and_then(Value::as_array) {
            let items: Vec<Value> = items
                .iter()
                .map(|item| self.normalize(item, used))
                .collect();
            return json!({ "array": items });
        }
        if let Some(node) = value.get("node").and_then(Value::as_str) {
            return self.normalize_model(node, value, used);
        }
        value.clone()
    }

    fn normalize_model(
        &self,
        node: &str,
        value: &Value,
        used: &mut BTreeSet<&'static str>,
    ) -> Value {
        let id = self
            .by_name
            .get(node)
            .unwrap_or_else(|| panic!("expected model {node} is not in the graph"));
        let model = &self.models[id];
        let given = value["fields"].as_array().unwrap();
        let mut fields = Vec::new();
        for field in given {
            let name = field[0].as_str().unwrap();
            let field_value = &field[1];
            if name != "@element_order" {
                let default = model
                    .attributes
                    .iter()
                    .find(|attribute| attribute.name == name)
                    .and_then(|attribute| attribute.default.as_ref());
                if let (Some(default), Some(text)) = (default, field_value["str"].as_str()) {
                    if default.as_str() == Some(text) {
                        used.insert("unrendered_default_value");
                        continue;
                    }
                }
                fields.push(json!([name, self.normalize(field_value, used)]));
                continue;
            }
            if !model.keeps_order {
                used.insert("unwritten_element_order");
                continue;
            }
            let mut held: HashMap<&str, usize> = HashMap::new();
            for (attribute, element) in &model.names {
                let count = given
                    .iter()
                    .find(|other| other[0] == attribute.as_str())
                    .map_or(0, |other| other[1]["array"].as_array().map_or(1, Vec::len));
                *held.entry(element.as_str()).or_default() += count;
            }
            let mut seen: HashMap<&str, usize> = HashMap::new();
            let mut kept = Vec::new();
            for entry in field_value["array"].as_array().unwrap() {
                let pairs: HashMap<&str, &str> = entry["hash"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|pair| {
                        (
                            pair[0]["str"].as_str().unwrap(),
                            pair[1]["str"].as_str().unwrap(),
                        )
                    })
                    .collect();
                if pairs["type"] == "Element" {
                    let count = seen.entry(pairs["name"]).or_default();
                    let index = *count;
                    *count += 1;
                    if index >= held.get(pairs["name"]).copied().unwrap_or(0) {
                        used.insert("unread_element_order_entry");
                        continue;
                    }
                } else if !model.mixed && pairs["type"] == "Text" && pairs["text"].trim().is_empty()
                {
                    used.insert("ordered_whitespace_text");
                    continue;
                }
                kept.push(entry.clone());
            }
            if !kept.is_empty() {
                fields.push(json!([name, { "array": kept }]));
            }
        }
        json!({ "node": node, "fields": fields })
    }
}

/// Where `actual` first differs from `expected`, with a model path.
pub fn first_difference(actual: &Value, expected: &Value, path: &str) -> Option<String> {
    if actual == expected {
        return None;
    }
    match (actual, expected) {
        (Value::Object(left), Value::Object(right)) => {
            let at = match left.get("node").and_then(Value::as_str) {
                Some(node) => format!("{path}/{}", node.trim_start_matches("Omml::Models::")),
                None => path.to_owned(),
            };
            for key in left.keys().chain(right.keys()) {
                let found = first_difference(
                    left.get(key).unwrap_or(&Value::Null),
                    right.get(key).unwrap_or(&Value::Null),
                    &at,
                );
                if found.is_some() {
                    return found;
                }
            }
        }
        (Value::Array(left), Value::Array(right)) if left.len() == right.len() => {
            for (item, other) in left.iter().zip(right) {
                let at = match item.get(0).and_then(Value::as_str) {
                    Some(name) => format!("{path}/{name}"),
                    None => path.to_owned(),
                };
                let found = first_difference(item, other, &at);
                if found.is_some() {
                    return found;
                }
            }
        }
        (Value::Array(left), Value::Array(right)) => {
            return Some(format!(
                "{path}: {} entries, expected {}: {actual}",
                left.len(),
                right.len()
            ));
        }
        _ => {}
    }
    Some(format!("{path}: {actual}, expected {expected}"))
}

//! Replays the model-roundtrip rows of a pinned plurimath-testsuite omml
//! generation extracted by `.github/scripts/extract-corpus.py`. Set
//! `OMML_CORPUS_CASES` to the extracted JSON; `OMML_CORPUS_REPORT`
//! optionally receives every outcome.
//!
//! Each row's input goes through the oMath or oMathPara codec its root names, and the written XML must equal the
//! row's expected `to_xml` output byte for byte. A row whose expected outcome
//! is a refusal must be refused. A failing output is classed as
//! `output-bytes` (same element tree, different bytes) or `output-tree`.
//! The parsed model must also equal the row's expected Ruby model wire
//! (`model_wire`), or the row fails as `model-wire`. That needs the
//! `model-serde` feature.
//!
//! The ledger is append-only: a row whose id has an `-r2` sibling is
//! superseded by it and only the sibling runs, as in the Ruby gem's replay.
mod model_wire;

use lutaml_rs::xml::moxml_binding::moxml::document::{ContentRow, Document, ElementRow};
use model_wire::{first_difference, Graph};
use omml::document::{OmmlDocument, OmmlRoot};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet, HashSet};

fn tree(rows: &[ElementRow], ordinal: usize) -> Value {
    let element = &rows[ordinal];
    let mut attributes: Vec<_> = element
        .attributes
        .iter()
        .map(|attr| {
            (
                attr.name.namespace.as_deref(),
                &attr.name.local,
                &attr.value,
            )
        })
        .collect();
    attributes.sort();
    let has_elements = element
        .content
        .iter()
        .any(|node| matches!(node, ContentRow::Element(_)));
    let children: Vec<_> = element
        .content
        .iter()
        .filter_map(|node| match node {
            ContentRow::Element(ordinal) => Some(tree(rows, *ordinal)),
            ContentRow::Text(text) if has_elements && text.trim().is_empty() => None,
            ContentRow::Text(text) => Some(json!(["text", text])),
            ContentRow::CData(text) => Some(json!(["cdata", text])),
            ContentRow::Comment(text) => Some(json!(["comment", text])),
            ContentRow::ProcessingInstruction { target, data } => {
                Some(json!(["instruction", target, data]))
            }
        })
        .collect();
    json!([
        element.name.namespace.as_deref(),
        element.name.local,
        attributes,
        children
    ])
}

fn char_floor(text: &str, at: usize) -> usize {
    let mut at = at.min(text.len());
    while !text.is_char_boundary(at) {
        at -= 1;
    }
    at
}

fn xml_tree(xml: &str) -> Option<Value> {
    let doc = Document::parse(xml.as_bytes()).ok()?;
    Some(tree(&doc.element_rows(), 0))
}

/// What `Omml.parse(input)` and its `to_xml` do.
fn roundtrip(input: &str) -> Result<(String, OmmlDocument), String> {
    let doc = omml::document::parse(input.as_bytes()).map_err(|e| e.to_string())?;
    let xml = doc.to_xml().map_err(|e| e.to_string())?;
    Ok((xml, doc))
}

/// The Ruby model wire of a parsed document.
fn model_wire(graph: &Graph, doc: &OmmlDocument) -> Value {
    match &doc.root {
        OmmlRoot::OMath(model) => graph.wire(
            "Omml::Models::OMath@omml",
            &serde_json::to_value(&**model).expect("model-serde JSON"),
        ),
        OmmlRoot::OMathPara(model) => graph.wire(
            "Omml::Models::OMathPara@omml",
            &serde_json::to_value(&**model).expect("model-serde JSON"),
        ),
    }
}

/// Rows whose input is not well-formed XML. The Ruby gem's parser recovers
/// from them and writes output; this crate parses strictly (W3C XML 1.0,
/// no recovery), so it refuses them. Each must stay refused: a row listed
/// here that the crate accepts fails the replay, and the list goes away once
/// the testsuite records these rows as refusals.
const KNOWN_REFUSALS: &[(&str, &str)] = &[
    (
        "omml-equation-arrays-007.model-roundtrip",
        "bare '&' in text is not well-formed",
    ),
    (
        "omml-matrices-017.model-roundtrip",
        "'&nbsp;' is not an XML entity",
    ),
    (
        "omml-unclosed-para.model-roundtrip",
        "m:oMathPara is never closed",
    ),
    (
        "omml-crossed-close.model-roundtrip",
        "end tag m:r crosses the open m:t",
    ),
];

/// `None` when the row passes, otherwise `(kind, detail)`.
fn failure(
    row: &Value,
    graph: &Graph,
    profiles: &mut BTreeMap<&'static str, usize>,
) -> Option<(&'static str, String)> {
    let input = row["input"]["str"].as_str().unwrap();
    assert!(
        row["options"].as_object().is_none_or(|o| o.is_empty()),
        "{}: unsupported options",
        row["id"]
    );
    let result = roundtrip(input);
    let id = row["id"].as_str().unwrap();
    if KNOWN_REFUSALS.iter().any(|(known, _)| *known == id) {
        return result.ok().map(|(xml, _)| ("known-refusal-accepted", xml));
    }
    if row["expected"].get("refusal").is_some() {
        return result.ok().map(|(xml, _)| ("accepted", xml));
    }
    let expected = row["expected"]["output"]["str"].as_str().unwrap();
    match result {
        Err(error) => Some(("refused", error)),
        Ok((actual, doc)) if actual == expected => {
            let mut used = BTreeSet::new();
            let normalized = graph.normalize(&row["expected"]["model"], &mut used);
            for name in used {
                *profiles.entry(name).or_default() += 1;
            }
            first_difference(&model_wire(graph, &doc), &normalized, "")
                .map(|difference| ("model-wire", difference))
        }
        Ok((actual, _)) => {
            let same_tree = xml_tree(&actual).is_some() && xml_tree(&actual) == xml_tree(expected);
            let at = actual
                .bytes()
                .zip(expected.bytes())
                .take_while(|(a, e)| a == e)
                .count();
            let from = char_floor(&actual, at.saturating_sub(80));
            let to = char_floor(&actual, (at + 160).min(actual.len()));
            let kind = if same_tree {
                "output-bytes"
            } else {
                "output-tree"
            };
            Some((
                kind,
                format!("differs at byte {at}: …{}", &actual[from..to]),
            ))
        }
    }
}

#[test]
#[ignore = "pinned generation replay runs in CI"]
fn pinned_generation_replay() {
    let path = std::env::var("OMML_CORPUS_CASES").expect("OMML_CORPUS_CASES");
    let corpus: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let rows = corpus["cases"].as_array().unwrap();
    let ids: HashSet<&str> = rows.iter().map(|r| r["id"].as_str().unwrap()).collect();
    let superseded: HashSet<String> = ids
        .iter()
        .filter(|id| id.contains("-r2"))
        .map(|id| id.replacen("-r2", "", 1))
        .collect();
    let graph = Graph::load(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/model/runtime-graph-v1.json"
    ));
    let mut profiles = BTreeMap::<&'static str, usize>::new();
    let mut outcomes = Vec::new();
    let mut failures = 0usize;
    for row in rows {
        assert_eq!(row["boundary"], "model-roundtrip", "{}", row["id"]);
        if superseded.contains(row["id"].as_str().unwrap()) {
            continue;
        }
        let failure = failure(row, &graph, &mut profiles);
        failures += usize::from(failure.is_some());
        let (kind, detail) = match failure {
            Some((kind, detail)) => (
                Some(kind),
                Some(detail.chars().take(800).collect::<String>()),
            ),
            None => (None, None),
        };
        outcomes.push(
            json!({"id": row["id"], "payload": row["payload"], "kind": kind, "failure": detail}),
        );
    }
    assert_eq!(outcomes.len() + superseded.len(), rows.len());
    for (known, reason) in KNOWN_REFUSALS {
        assert!(
            ids.contains(known),
            "known refusal {known} ({reason}) is not in the corpus"
        );
    }
    let mut kinds = std::collections::BTreeMap::<String, usize>::new();
    for outcome in &outcomes {
        if let Some(kind) = outcome["kind"].as_str() {
            *kinds.entry(kind.into()).or_default() += 1;
        }
    }
    let report = json!({
        "language": corpus["language"],
        "generation": corpus["generation"],
        "revision": corpus["revision"],
        "executed": outcomes.len(),
        "superseded": superseded.len(),
        "failed": failures,
        "known_refusals": KNOWN_REFUSALS.iter().map(|(id, reason)| json!({"id": id, "reason": reason})).collect::<Vec<_>>(),
        "kinds": kinds,
        "model_wire_profiles": profiles,
        "outcomes": outcomes,
    });
    if let Ok(out) = std::env::var("OMML_CORPUS_REPORT") {
        std::fs::write(out, serde_json::to_string_pretty(&report).unwrap()).unwrap();
    }
    println!(
        "{} rows executed, {} superseded, {failures} failed {kinds:?}",
        outcomes.len(),
        superseded.len()
    );
    println!("model wire normalizations (rows): {profiles:?}");
    let shown: Vec<_> = outcomes
        .iter()
        .filter(|o| !o["kind"].is_null())
        .take(20)
        .collect();
    assert_eq!(
        failures,
        0,
        "{failures} of {} rows failed {kinds:?}: {shown:#?}",
        outcomes.len()
    );
}

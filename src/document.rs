//! `Omml.parse(input).to_xml` for native callers: the root picks the oMath
//! or oMathPara model, and writing it back gives the bytes the Ruby gem
//! writes for the same input.
//!
//! The generated codecs write compact XML that declares only the namespaces
//! the model uses. The Ruby gem also keeps the namespace declarations the
//! input root carried (in input order) and indents element-only content by
//! two spaces. A root declaration the writer itself places lower down (the
//! WordprocessingML prefix on `w:rPr`) stays there and is not repeated on
//! the root.
use crate::contracts::{ContractResult, ErrorReason};
use crate::models::{Omml_Models_OMath, Omml_Models_OMathPara};
use crate::xml_codecs::{
    read_Omml_Models_OMathPara_xml, read_Omml_Models_OMath_xml, write_Omml_Models_OMathPara_xml,
    write_Omml_Models_OMath_xml,
};
use crate::xml_rt::moxml_binding::moxml::document::{
    ContentRow, ContentSpec, Document, ElementRow, ElementSpec, NamespaceDecl, SerializeOptions,
};
use crate::xml_rt::moxml_binding::{fail, moxml};

/// The OMML namespace both document roots live in.
pub const OMML_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/math";

const ID: &str = "Omml::Document";

/// The model a document root reads into.
#[derive(Debug, Clone)]
pub enum OmmlRoot {
    OMath(Box<Omml_Models_OMath>),
    OMathPara(Box<Omml_Models_OMathPara>),
}

/// A parsed OMML document: its model and the input root's namespace
/// declarations.
#[derive(Debug, Clone)]
pub struct OmmlDocument {
    pub root: OmmlRoot,
    pub root_namespaces: Vec<NamespaceDecl>,
}

fn xml_error(
    reason: ErrorReason,
    location: &str,
    error: moxml::document::Error,
) -> Box<crate::contracts::ContractError> {
    fail(reason, ID, None, None, location, error)
}

/// Parses an `m:oMath` or `m:oMathPara` document; any other root is refused.
pub fn parse(input: &[u8]) -> ContractResult<OmmlDocument> {
    let doc =
        Document::parse(input).map_err(|e| xml_error(ErrorReason::InvalidDocument, "/input", e))?;
    let root_id = doc.root();
    let name = doc
        .name(root_id)
        .map_err(|e| xml_error(ErrorReason::InvalidDocument, "/input", e))?;
    let root = match (name.namespace.as_deref(), name.local.as_str()) {
        (Some(OMML_NS), "oMath") => OmmlRoot::OMath(Box::new(read_Omml_Models_OMath_xml(input)?)),
        (Some(OMML_NS), "oMathPara") => {
            OmmlRoot::OMathPara(Box::new(read_Omml_Models_OMathPara_xml(input)?))
        }
        (namespace, local) => {
            return Err(fail(
                ErrorReason::InvalidDocument,
                ID,
                None,
                None,
                "/input",
                format!("unsupported root {namespace:?} {local}"),
            ))
        }
    };
    let root_namespaces = doc
        .namespace_declarations(root_id)
        .map_err(|e| xml_error(ErrorReason::InvalidDocument, "/input", e))?;
    Ok(OmmlDocument {
        root,
        root_namespaces,
    })
}

impl OmmlDocument {
    /// Writes the model back as the Ruby gem's `to_xml` does.
    pub fn to_xml(&self) -> ContractResult<String> {
        let compact = match &self.root {
            OmmlRoot::OMath(m) => write_Omml_Models_OMath_xml(m)?,
            OmmlRoot::OMathPara(m) => write_Omml_Models_OMathPara_xml(m)?,
        };
        let bad = |e: moxml::document::Error| xml_error(ErrorReason::NotSerializable, "/output", e);
        let rows = Document::parse(compact.as_bytes())
            .map_err(bad)?
            .element_rows();
        let mut root = spec(&rows, 0);
        root.namespaces = merge_root_namespaces(&self.root_namespaces, &rows);
        Document::build(root)
            .map_err(bad)?
            .to_xml_with(&SerializeOptions {
                xml_declaration: false,
                indent: Some(2),
            })
            .map_err(bad)
    }
}

/// The input root's declarations in input order, minus any the writer
/// declares below the root, then whatever the writer declares on the root
/// that the input did not.
fn merge_root_namespaces(input: &[NamespaceDecl], rows: &[ElementRow]) -> Vec<NamespaceDecl> {
    let below: Vec<&NamespaceDecl> = rows.iter().skip(1).flat_map(|r| &r.namespaces).collect();
    let mut merged: Vec<NamespaceDecl> = input
        .iter()
        .filter(|d| !below.contains(d))
        .cloned()
        .collect();
    for decl in &rows[0].namespaces {
        match merged.iter().position(|d| d.prefix == decl.prefix) {
            Some(at) => merged[at].uri.clone_from(&decl.uri),
            None => merged.push(decl.clone()),
        }
    }
    merged
}

fn spec(rows: &[ElementRow], ordinal: usize) -> ElementSpec {
    let row = &rows[ordinal];
    ElementSpec {
        name: row.name.clone(),
        namespaces: row.namespaces.clone(),
        attributes: row.attributes.clone(),
        content: row
            .content
            .iter()
            .map(|node| match node {
                ContentRow::Element(child) => ContentSpec::Element(spec(rows, *child)),
                ContentRow::Text(t) => ContentSpec::Text(t.clone()),
                ContentRow::CData(t) => ContentSpec::CData(t.clone()),
                ContentRow::Comment(t) => ContentSpec::Comment(t.clone()),
                ContentRow::ProcessingInstruction { target, data } => {
                    ContentSpec::ProcessingInstruction {
                        target: target.clone(),
                        data: data.clone(),
                    }
                }
            })
            .collect(),
    }
}

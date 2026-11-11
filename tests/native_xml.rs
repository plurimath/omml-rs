use omml::xml_codecs::{
    read_Omml_Models_OMath_xml, write_Omml_Models_OMath_xml,
    read_Omml_Models_CTAnchor_xml, write_Omml_Models_CTAnchor_xml,
};

const NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/math";

#[test]
fn math_root_uses_generated_native_models() {
    let xml = format!(r#"<m:oMath xmlns:m="{NS}"><m:r><m:t>x</m:t></m:r></m:oMath>"#);
    let model = read_Omml_Models_OMath_xml(xml.as_bytes()).unwrap();
    let written = write_Omml_Models_OMath_xml(&model).unwrap();
    assert_eq!(read_Omml_Models_OMath_xml(written.as_bytes()).unwrap(), model);
}

#[test]
fn anchor_keeps_coordinate_child_and_boolean_separate() {
    let xml = format!(r#"<m:CT_Anchor xmlns:m="{NS}" m:simplePos="true"><m:simplePos m:x="125" m:y="-250"/></m:CT_Anchor>"#);
    let model = read_Omml_Models_CTAnchor_xml(xml.as_bytes()).unwrap();
    let written = write_Omml_Models_CTAnchor_xml(&model).unwrap();
    assert!(written.contains("simplePos"));
    assert_eq!(read_Omml_Models_CTAnchor_xml(written.as_bytes()).unwrap(), model);
}

#[test]
fn parser_policy_refuses_invalid_xml_and_reuses_generated_reader() {
    assert!(read_Omml_Models_OMath_xml(b"<!DOCTYPE r><m:oMath/>").is_err());
    assert!(read_Omml_Models_OMath_xml(b"<broken>").is_err());
    let xml = format!(r#"<m:oMath xmlns:m="{NS}"/>"#);
    assert!(read_Omml_Models_OMath_xml(xml.as_bytes()).is_ok());
}

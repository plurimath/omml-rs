// Generated dispatch over the SDK-emitted, statically typed OMML codecs.
use crate::xml_codecs;
use lutaml_rs::contracts::ContractError;
use serde_json::{json, Value};

fn contract(error: Box<ContractError>) -> Value {
    json!({"category":"contract","contract":error})
}

pub fn roundtrip(root: &str, xml: &[u8]) -> Result<Value, Value> {
    match root {
        "CTAcc" => {
            let model = xml_codecs::read_Omml_Models_CTAcc_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTAcc_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTAccPr" => {
            let model = xml_codecs::read_Omml_Models_CTAccPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTAccPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTAnchor" => {
            let model = xml_codecs::read_Omml_Models_CTAnchor_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTAnchor_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTAttr" => {
            let model = xml_codecs::read_Omml_Models_CTAttr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTAttr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTBar" => {
            let model = xml_codecs::read_Omml_Models_CTBar_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTBar_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTBarPr" => {
            let model = xml_codecs::read_Omml_Models_CTBarPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTBarPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTBookmark" => {
            let model = xml_codecs::read_Omml_Models_CTBookmark_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTBookmark_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTBorder" => {
            let model = xml_codecs::read_Omml_Models_CTBorder_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTBorder_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTBorderBox" => {
            let model = xml_codecs::read_Omml_Models_CTBorderBox_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTBorderBox_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTBorderBoxPr" => {
            let model = xml_codecs::read_Omml_Models_CTBorderBoxPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTBorderBoxPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTBox" => {
            let model = xml_codecs::read_Omml_Models_CTBox_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTBox_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTBoxPr" => {
            let model = xml_codecs::read_Omml_Models_CTBoxPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTBoxPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTBr" => {
            let model = xml_codecs::read_Omml_Models_CTBr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTBr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTBreakBin" => {
            let model = xml_codecs::read_Omml_Models_CTBreakBin_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTBreakBin_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTBreakBinSub" => {
            let model = xml_codecs::read_Omml_Models_CTBreakBinSub_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTBreakBinSub_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTCalendarType" => {
            let model = xml_codecs::read_Omml_Models_CTCalendarType_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTCalendarType_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTChar" => {
            let model = xml_codecs::read_Omml_Models_CTChar_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTChar_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTColor" => {
            let model = xml_codecs::read_Omml_Models_CTColor_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTColor_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTControl" => {
            let model = xml_codecs::read_Omml_Models_CTControl_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTControl_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTCtrlPr" => {
            let model = xml_codecs::read_Omml_Models_CTCtrlPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTCtrlPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTCustomXmlPr" => {
            let model = xml_codecs::read_Omml_Models_CTCustomXmlPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTCustomXmlPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTCustomXmlRun" => {
            let model = xml_codecs::read_Omml_Models_CTCustomXmlRun_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTCustomXmlRun_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTD" => {
            let model = xml_codecs::read_Omml_Models_CTD_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTD_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTDPr" => {
            let model = xml_codecs::read_Omml_Models_CTDPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTDPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTDataBinding" => {
            let model = xml_codecs::read_Omml_Models_CTDataBinding_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTDataBinding_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTDecimalNumber" => {
            let model = xml_codecs::read_Omml_Models_CTDecimalNumber_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTDecimalNumber_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTDrawing" => {
            let model = xml_codecs::read_Omml_Models_CTDrawing_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTDrawing_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTEastAsianLayout" => {
            let model = xml_codecs::read_Omml_Models_CTEastAsianLayout_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTEastAsianLayout_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTEffectExtent" => {
            let model = xml_codecs::read_Omml_Models_CTEffectExtent_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTEffectExtent_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTEm" => {
            let model = xml_codecs::read_Omml_Models_CTEm_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTEm_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTEmpty" => {
            let model = xml_codecs::read_Omml_Models_CTEmpty_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTEmpty_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTEqArr" => {
            let model = xml_codecs::read_Omml_Models_CTEqArr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTEqArr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTEqArrPr" => {
            let model = xml_codecs::read_Omml_Models_CTEqArrPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTEqArrPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTF" => {
            let model = xml_codecs::read_Omml_Models_CTF_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTF_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTFFCheckBox" => {
            let model = xml_codecs::read_Omml_Models_CTFFCheckBox_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTFFCheckBox_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTFFDDList" => {
            let model = xml_codecs::read_Omml_Models_CTFFDDList_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTFFDDList_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTFFData" => {
            let model = xml_codecs::read_Omml_Models_CTFFData_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTFFData_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTFFHelpText" => {
            let model = xml_codecs::read_Omml_Models_CTFFHelpText_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTFFHelpText_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTFFName" => {
            let model = xml_codecs::read_Omml_Models_CTFFName_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTFFName_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTFFStatusText" => {
            let model = xml_codecs::read_Omml_Models_CTFFStatusText_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTFFStatusText_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTFFTextInput" => {
            let model = xml_codecs::read_Omml_Models_CTFFTextInput_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTFFTextInput_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTFFTextType" => {
            let model = xml_codecs::read_Omml_Models_CTFFTextType_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTFFTextType_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTFPr" => {
            let model = xml_codecs::read_Omml_Models_CTFPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTFPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTFType" => {
            let model = xml_codecs::read_Omml_Models_CTFType_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTFType_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTFitText" => {
            let model = xml_codecs::read_Omml_Models_CTFitText_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTFitText_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTFldChar" => {
            let model = xml_codecs::read_Omml_Models_CTFldChar_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTFldChar_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTFonts" => {
            let model = xml_codecs::read_Omml_Models_CTFonts_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTFonts_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTFtnEdnRef" => {
            let model = xml_codecs::read_Omml_Models_CTFtnEdnRef_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTFtnEdnRef_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTFunc" => {
            let model = xml_codecs::read_Omml_Models_CTFunc_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTFunc_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTFuncPr" => {
            let model = xml_codecs::read_Omml_Models_CTFuncPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTFuncPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTGraphicalObject" => {
            let model = xml_codecs::read_Omml_Models_CTGraphicalObject_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTGraphicalObject_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTGraphicalObjectData" => {
            let model = xml_codecs::read_Omml_Models_CTGraphicalObjectData_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTGraphicalObjectData_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTGraphicalObjectFrameLocking" => {
            let model = xml_codecs::read_Omml_Models_CTGraphicalObjectFrameLocking_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTGraphicalObjectFrameLocking_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTGroupChr" => {
            let model = xml_codecs::read_Omml_Models_CTGroupChr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTGroupChr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTGroupChrPr" => {
            let model = xml_codecs::read_Omml_Models_CTGroupChrPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTGroupChrPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTHighlight" => {
            let model = xml_codecs::read_Omml_Models_CTHighlight_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTHighlight_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTHpsMeasure" => {
            let model = xml_codecs::read_Omml_Models_CTHpsMeasure_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTHpsMeasure_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTHyperlink" => {
            let model = xml_codecs::read_Omml_Models_CTHyperlink_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTHyperlink_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTInline" => {
            let model = xml_codecs::read_Omml_Models_CTInline_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTInline_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTInteger255" => {
            let model = xml_codecs::read_Omml_Models_CTInteger255_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTInteger255_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTInteger2" => {
            let model = xml_codecs::read_Omml_Models_CTInteger2_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTInteger2_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTLang" => {
            let model = xml_codecs::read_Omml_Models_CTLang_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTLang_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTLanguage" => {
            let model = xml_codecs::read_Omml_Models_CTLanguage_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTLanguage_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTLimLoc" => {
            let model = xml_codecs::read_Omml_Models_CTLimLoc_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTLimLoc_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTLimLow" => {
            let model = xml_codecs::read_Omml_Models_CTLimLow_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTLimLow_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTLimLowPr" => {
            let model = xml_codecs::read_Omml_Models_CTLimLowPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTLimLowPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTLimUpp" => {
            let model = xml_codecs::read_Omml_Models_CTLimUpp_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTLimUpp_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTLimUppPr" => {
            let model = xml_codecs::read_Omml_Models_CTLimUppPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTLimUppPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTLock" => {
            let model = xml_codecs::read_Omml_Models_CTLock_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTLock_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTM" => {
            let model = xml_codecs::read_Omml_Models_CTM_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTM_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTMC" => {
            let model = xml_codecs::read_Omml_Models_CTMC_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTMC_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTMCPr" => {
            let model = xml_codecs::read_Omml_Models_CTMCPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTMCPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTMCS" => {
            let model = xml_codecs::read_Omml_Models_CTMCS_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTMCS_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTMPr" => {
            let model = xml_codecs::read_Omml_Models_CTMPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTMPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTMR" => {
            let model = xml_codecs::read_Omml_Models_CTMR_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTMR_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTMacroName" => {
            let model = xml_codecs::read_Omml_Models_CTMacroName_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTMacroName_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTManualBreak" => {
            let model = xml_codecs::read_Omml_Models_CTManualBreak_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTManualBreak_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTMarkup" => {
            let model = xml_codecs::read_Omml_Models_CTMarkup_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTMarkup_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTMarkupRange" => {
            let model = xml_codecs::read_Omml_Models_CTMarkupRange_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTMarkupRange_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTMathPr" => {
            let model = xml_codecs::read_Omml_Models_CTMathPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTMathPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTMoveBookmark" => {
            let model = xml_codecs::read_Omml_Models_CTMoveBookmark_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTMoveBookmark_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTNary" => {
            let model = xml_codecs::read_Omml_Models_CTNary_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTNary_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTNaryPr" => {
            let model = xml_codecs::read_Omml_Models_CTNaryPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTNaryPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTNonVisualDrawingProps" => {
            let model = xml_codecs::read_Omml_Models_CTNonVisualDrawingProps_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTNonVisualDrawingProps_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTNonVisualGraphicFrameProperties" => {
            let model = xml_codecs::read_Omml_Models_CTNonVisualGraphicFrameProperties_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTNonVisualGraphicFrameProperties_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTOMath" => {
            let model = xml_codecs::read_Omml_Models_CTOMath_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTOMath_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTOMathArg" => {
            let model = xml_codecs::read_Omml_Models_CTOMathArg_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTOMathArg_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTOMathArgPr" => {
            let model = xml_codecs::read_Omml_Models_CTOMathArgPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTOMathArgPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTOMathJc" => {
            let model = xml_codecs::read_Omml_Models_CTOMathJc_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTOMathJc_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTOMathPara" => {
            let model = xml_codecs::read_Omml_Models_CTOMathPara_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTOMathPara_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTOMathParaPr" => {
            let model = xml_codecs::read_Omml_Models_CTOMathParaPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTOMathParaPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTObject" => {
            let model = xml_codecs::read_Omml_Models_CTObject_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTObject_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTOfficeArtExtension" => {
            let model = xml_codecs::read_Omml_Models_CTOfficeArtExtension_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTOfficeArtExtension_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTOfficeArtExtensionList" => {
            let model = xml_codecs::read_Omml_Models_CTOfficeArtExtensionList_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTOfficeArtExtensionList_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTOnOff" => {
            let model = xml_codecs::read_Omml_Models_CTOnOff_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTOnOff_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTPTab" => {
            let model = xml_codecs::read_Omml_Models_CTPTab_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTPTab_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTPerm" => {
            let model = xml_codecs::read_Omml_Models_CTPerm_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTPerm_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTPermStart" => {
            let model = xml_codecs::read_Omml_Models_CTPermStart_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTPermStart_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTPhant" => {
            let model = xml_codecs::read_Omml_Models_CTPhant_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTPhant_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTPhantPr" => {
            let model = xml_codecs::read_Omml_Models_CTPhantPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTPhantPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTPicture" => {
            let model = xml_codecs::read_Omml_Models_CTPicture_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTPicture_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTPlaceholder" => {
            let model = xml_codecs::read_Omml_Models_CTPlaceholder_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTPlaceholder_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTPoint2D" => {
            let model = xml_codecs::read_Omml_Models_CTPoint2D_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTPoint2D_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTPosH" => {
            let model = xml_codecs::read_Omml_Models_CTPosH_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTPosH_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTPosV" => {
            let model = xml_codecs::read_Omml_Models_CTPosV_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTPosV_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTPositiveSize2D" => {
            let model = xml_codecs::read_Omml_Models_CTPositiveSize2D_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTPositiveSize2D_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTProofErr" => {
            let model = xml_codecs::read_Omml_Models_CTProofErr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTProofErr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTR" => {
            let model = xml_codecs::read_Omml_Models_CTR_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTR_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTRPR" => {
            let model = xml_codecs::read_Omml_Models_CTRPR_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTRPR_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTRPr" => {
            let model = xml_codecs::read_Omml_Models_CTRPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTRPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTRPrChange" => {
            let model = xml_codecs::read_Omml_Models_CTRPrChange_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTRPrChange_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTRPrOriginal" => {
            let model = xml_codecs::read_Omml_Models_CTRPrOriginal_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTRPrOriginal_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTRad" => {
            let model = xml_codecs::read_Omml_Models_CTRad_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTRad_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTRadPr" => {
            let model = xml_codecs::read_Omml_Models_CTRadPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTRadPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTRel" => {
            let model = xml_codecs::read_Omml_Models_CTRel_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTRel_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTRuby" => {
            let model = xml_codecs::read_Omml_Models_CTRuby_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTRuby_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTRubyAlign" => {
            let model = xml_codecs::read_Omml_Models_CTRubyAlign_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTRubyAlign_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTRubyContent" => {
            let model = xml_codecs::read_Omml_Models_CTRubyContent_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTRubyContent_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTRubyPr" => {
            let model = xml_codecs::read_Omml_Models_CTRubyPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTRubyPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTRunTrackChange" => {
            let model = xml_codecs::read_Omml_Models_CTRunTrackChange_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTRunTrackChange_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSPre" => {
            let model = xml_codecs::read_Omml_Models_CTSPre_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSPre_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSPrePr" => {
            let model = xml_codecs::read_Omml_Models_CTSPrePr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSPrePr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSSub" => {
            let model = xml_codecs::read_Omml_Models_CTSSub_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSSub_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSSubPr" => {
            let model = xml_codecs::read_Omml_Models_CTSSubPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSSubPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSSubSup" => {
            let model = xml_codecs::read_Omml_Models_CTSSubSup_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSSubSup_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSSubSupPr" => {
            let model = xml_codecs::read_Omml_Models_CTSSubSupPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSSubSupPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSSup" => {
            let model = xml_codecs::read_Omml_Models_CTSSup_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSSup_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSSupPr" => {
            let model = xml_codecs::read_Omml_Models_CTSSupPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSSupPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTScript" => {
            let model = xml_codecs::read_Omml_Models_CTScript_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTScript_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSdtComboBox" => {
            let model = xml_codecs::read_Omml_Models_CTSdtComboBox_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSdtComboBox_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSdtContentRun" => {
            let model = xml_codecs::read_Omml_Models_CTSdtContentRun_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSdtContentRun_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSdtDate" => {
            let model = xml_codecs::read_Omml_Models_CTSdtDate_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSdtDate_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSdtDateMappingType" => {
            let model = xml_codecs::read_Omml_Models_CTSdtDateMappingType_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSdtDateMappingType_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSdtDocPart" => {
            let model = xml_codecs::read_Omml_Models_CTSdtDocPart_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSdtDocPart_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSdtDropDownList" => {
            let model = xml_codecs::read_Omml_Models_CTSdtDropDownList_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSdtDropDownList_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSdtEndPr" => {
            let model = xml_codecs::read_Omml_Models_CTSdtEndPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSdtEndPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSdtListItem" => {
            let model = xml_codecs::read_Omml_Models_CTSdtListItem_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSdtListItem_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSdtPr" => {
            let model = xml_codecs::read_Omml_Models_CTSdtPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSdtPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSdtRun" => {
            let model = xml_codecs::read_Omml_Models_CTSdtRun_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSdtRun_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSdtText" => {
            let model = xml_codecs::read_Omml_Models_CTSdtText_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSdtText_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTShd" => {
            let model = xml_codecs::read_Omml_Models_CTShd_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTShd_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTShp" => {
            let model = xml_codecs::read_Omml_Models_CTShp_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTShp_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSignedHpsMeasure" => {
            let model = xml_codecs::read_Omml_Models_CTSignedHpsMeasure_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSignedHpsMeasure_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSignedTwipsMeasure" => {
            let model = xml_codecs::read_Omml_Models_CTSignedTwipsMeasure_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSignedTwipsMeasure_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSimpleField" => {
            let model = xml_codecs::read_Omml_Models_CTSimpleField_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSimpleField_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSmartTagPr" => {
            let model = xml_codecs::read_Omml_Models_CTSmartTagPr_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSmartTagPr_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSmartTagRun" => {
            let model = xml_codecs::read_Omml_Models_CTSmartTagRun_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSmartTagRun_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSpacingRule" => {
            let model = xml_codecs::read_Omml_Models_CTSpacingRule_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSpacingRule_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTString" => {
            let model = xml_codecs::read_Omml_Models_CTString_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTString_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTStyle" => {
            let model = xml_codecs::read_Omml_Models_CTStyle_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTStyle_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTSym" => {
            let model = xml_codecs::read_Omml_Models_CTSym_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTSym_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTText" => {
            let model = xml_codecs::read_Omml_Models_CTText_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTText_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTTextEffect" => {
            let model = xml_codecs::read_Omml_Models_CTTextEffect_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTTextEffect_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTTextScale" => {
            let model = xml_codecs::read_Omml_Models_CTTextScale_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTTextScale_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTTopBot" => {
            let model = xml_codecs::read_Omml_Models_CTTopBot_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTTopBot_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTTrackChange" => {
            let model = xml_codecs::read_Omml_Models_CTTrackChange_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTTrackChange_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTTrackChangeNumbering" => {
            let model = xml_codecs::read_Omml_Models_CTTrackChangeNumbering_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTTrackChangeNumbering_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTTwipsMeasure" => {
            let model = xml_codecs::read_Omml_Models_CTTwipsMeasure_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTTwipsMeasure_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTUnSignedInteger" => {
            let model = xml_codecs::read_Omml_Models_CTUnSignedInteger_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTUnSignedInteger_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTUnderline" => {
            let model = xml_codecs::read_Omml_Models_CTUnderline_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTUnderline_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTVerticalAlignRun" => {
            let model = xml_codecs::read_Omml_Models_CTVerticalAlignRun_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTVerticalAlignRun_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTWordprocessingEmpty" => {
            let model = xml_codecs::read_Omml_Models_CTWordprocessingEmpty_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTWordprocessingEmpty_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTWordprocessingOnOff" => {
            let model = xml_codecs::read_Omml_Models_CTWordprocessingOnOff_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTWordprocessingOnOff_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTWordprocessingText" => {
            let model = xml_codecs::read_Omml_Models_CTWordprocessingText_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTWordprocessingText_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTWrapNone" => {
            let model = xml_codecs::read_Omml_Models_CTWrapNone_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTWrapNone_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTWrapPath" => {
            let model = xml_codecs::read_Omml_Models_CTWrapPath_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTWrapPath_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTWrapSquare" => {
            let model = xml_codecs::read_Omml_Models_CTWrapSquare_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTWrapSquare_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTWrapThrough" => {
            let model = xml_codecs::read_Omml_Models_CTWrapThrough_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTWrapThrough_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTWrapTight" => {
            let model = xml_codecs::read_Omml_Models_CTWrapTight_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTWrapTight_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTWrapTopBottom" => {
            let model = xml_codecs::read_Omml_Models_CTWrapTopBottom_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTWrapTopBottom_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTXAlign" => {
            let model = xml_codecs::read_Omml_Models_CTXAlign_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTXAlign_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "CTYAlign" => {
            let model = xml_codecs::read_Omml_Models_CTYAlign_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_CTYAlign_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "EGContentRunContent" => {
            let model = xml_codecs::read_Omml_Models_EGContentRunContent_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_EGContentRunContent_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "EGMathContent" => {
            let model = xml_codecs::read_Omml_Models_EGMathContent_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_EGMathContent_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "EGOMathElements" => {
            let model = xml_codecs::read_Omml_Models_EGOMathElements_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_EGOMathElements_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "EGOMathMathElements" => {
            let model = xml_codecs::read_Omml_Models_EGOMathMathElements_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_EGOMathMathElements_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "EGPContent" => {
            let model = xml_codecs::read_Omml_Models_EGPContent_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_EGPContent_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "EGRPrMath" => {
            let model = xml_codecs::read_Omml_Models_EGRPrMath_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_EGRPrMath_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "EGRangeMarkupElements" => {
            let model = xml_codecs::read_Omml_Models_EGRangeMarkupElements_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_EGRangeMarkupElements_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "EGRubyContent" => {
            let model = xml_codecs::read_Omml_Models_EGRubyContent_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_EGRubyContent_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "EGRunInnerContent" => {
            let model = xml_codecs::read_Omml_Models_EGRunInnerContent_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_EGRunInnerContent_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "EGRunLevelElts" => {
            let model = xml_codecs::read_Omml_Models_EGRunLevelElts_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_EGRunLevelElts_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "EGWordRunInnerContent" => {
            let model = xml_codecs::read_Omml_Models_EGWordRunInnerContent_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_EGWordRunInnerContent_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "EGWrapType" => {
            let model = xml_codecs::read_Omml_Models_EGWrapType_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_EGWrapType_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "OMath" => {
            let model = xml_codecs::read_Omml_Models_OMath_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_OMath_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        "OMathPara" => {
            let model = xml_codecs::read_Omml_Models_OMathPara_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_Omml_Models_OMathPara_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({"category":"serialization","message":error.to_string()}))?;
            Ok(json!({"model":model,"xml":xml}))
        },
        _ => Err(json!({"category":"invalid_argument","message":"unknown OMML model type"})),
    }
}

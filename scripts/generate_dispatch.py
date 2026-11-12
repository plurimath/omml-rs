"""Generate only the root dispatch table over the upstream generated codecs."""
from pathlib import Path
import re

root = Path(__file__).resolve().parents[1]
source = (root / "src/generated/xml_codecs.rs").read_text()
names = re.findall(r"pub fn read_(\w+)_xml\(", source)
assert len(names) == 186
arms = []
for name in names:
    public_name = name.removeprefix("Omml_Models_")
    arms.append(f'''        "{public_name}" => {{
            let model = xml_codecs::read_{name}_xml(xml).map_err(contract)?;
            let xml = xml_codecs::write_{name}_xml(&model).map_err(contract)?;
            let model = serde_json::to_value(model).map_err(|error| json!({{"category":"serialization","message":error.to_string()}}))?;
            Ok(json!({{"model":model,"xml":xml}}))
        }},''')
output = '''// Generated dispatch over the SDK-emitted, statically typed OMML codecs.
use crate::xml_codecs;
use lutaml_rs::contracts::ContractError;
use serde_json::{json, Value};

fn contract(error: Box<ContractError>) -> Value {
    json!({"category":"contract","contract":error})
}

pub fn roundtrip(root: &str, xml: &[u8]) -> Result<Value, Value> {
    match root {
'''+ "\n".join(arms) + '''
        _ => Err(json!({"category":"invalid_argument","message":"unknown OMML model type"})),
    }
}
'''
(root / "src/dispatch.rs").write_text(output)

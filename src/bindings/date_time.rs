// ISO dateTime binding. Chrono validates the qualified lexical forms;
// output follows the source Ruby XML serializer's timezone and microsecond spelling.
use chrono::{DateTime, FixedOffset, NaiveDateTime, Timelike};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DateTimeValue(DateTime<FixedOffset>);

pub fn parse(raw: &str) -> Result<Option<DateTimeValue>, &'static str> {
    if let Some((_, fraction)) = raw.split_once('.') {
        if fraction.bytes().take_while(u8::is_ascii_digit).count() > 9 {
            return Err("dateTime fractional precision exceeds the native ISO profile");
        }
    }
    let value = DateTime::parse_from_rfc3339(raw)
        .or_else(|_| {
            let naive = NaiveDateTime::parse_from_str(raw, "%Y-%m-%dT%H:%M:%S%.f")?;
            Ok::<_, chrono::ParseError>(
                naive
                    .and_local_timezone(FixedOffset::east_opt(0).unwrap())
                    .unwrap(),
            )
        })
        .map_err(|_| "unsupported or invalid OMML ISO dateTime")?;
    if value.nanosecond() >= 1_000_000_000 {
        return Err("leap-second dateTime is outside the native ISO profile");
    }
    Ok(Some(DateTimeValue(value)))
}

pub fn format(value: &DateTimeValue) -> Result<String, std::convert::Infallible> {
    let datetime = &value.0;
    let mut output = datetime.format("%Y-%m-%dT%H:%M:%S").to_string();
    let nanos = datetime.nanosecond();
    if nanos != 0 {
        let micros = nanos / 1000;
        if micros % 1000 == 0 {
            output.push_str(&format!(".{:03}", micros / 1000));
        } else {
            output.push_str(&format!(".{micros:06}"));
        }
    }
    if datetime.offset().local_minus_utc() == 0 {
        output.push('Z');
    } else {
        output.push_str(&datetime.format("%:z").to_string());
    }
    Ok(output)
}

impl Serialize for DateTimeValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format(self).unwrap())
    }
}

impl<'de> Deserialize<'de> for DateTimeValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        parse(&String::deserialize(deserializer)?)
            .map_err(serde::de::Error::custom)?
            .ok_or_else(|| serde::de::Error::custom("missing dateTime"))
    }
}

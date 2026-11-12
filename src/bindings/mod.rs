pub mod coordinate;
pub mod date_time;
pub mod integer;
pub mod token;

pub mod token_codec {
    use super::token::Token;

    pub fn parse(raw: &str) -> Result<Option<Token>, &'static str> {
        Token::new(raw.to_string()).map(Some)
    }

    pub fn format(value: &Token) -> Result<String, serde_json::Error> {
        Ok(serde_json::to_value(value)?
            .as_str()
            .expect("Token serializes as a string")
            .to_string())
    }
}

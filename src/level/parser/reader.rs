use crate::level::{LevelStatements, LevelTokenParseError, LevelTokens};
use std::str::FromStr;

pub fn read_level(content: String) {
    for line in content.lines() {
        let token = parse_token(line.to_string());
    }
}

fn parse_token(line: String) -> Result<LevelTokens, LevelTokenParseError> {
    let Some((token, parameters)) = line.split_once(" ") else {
        return Err(LevelTokenParseError::InvalidLine);
    };
    let parameters = parameters.split(" ").collect::<Vec<_>>();

    match LevelStatements::from_str(token) {
        Ok(LevelStatements::Version) => {
            if parameters.len() != 1 {
                return Err(LevelTokenParseError::InvalidArgsSize(parameters.len(), 1));
            }

            Ok(LevelTokens::Version(parameters[0].to_string()))
        }
        _ => Err(LevelTokenParseError::UnknownToken(token.to_string())),
    }
}

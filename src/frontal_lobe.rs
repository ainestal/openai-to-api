use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;

// Case insensitive enum that will be deserialized from a string
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Read,
    Write,
    Archive,
    None,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Intent {
    pub action: Action,
    pub content: Option<String>,
    pub comment: Option<String>,
}

pub fn get_json_object(input: String) -> Result<Intent> {
    let re = Regex::new(r#"\{.*\}"#)?;
    // Transform all text to lowercase
    let input = input.to_lowercase();
    let json_string = if let Some(m) = re.find(&input) {
        m.as_str()
    } else {
        return Err(anyhow::anyhow!("No JSON object found in the input string"));
    };
    let intent: Intent = serde_json::from_str(json_string)?;
    Ok(intent)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_receive_a_string_and_return_the_json_object_inside_or_none() {
        let input =
            r#"BlahBlahBlah. {"action": "REad", "content": "hello world"}. More blahblahblah"#
                .to_string();
        let expected = Intent {
            action: Action::Read,
            content: Some("hello world".to_string()),
            comment: None,
        };
        let actual = get_json_object(input);
        println!("{:?}", actual);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct Example {
    pub string: String,
    pub int: i32,
    pub float: f32,
}

#[cfg(test)]
mod tests {
    use super::Example;

    #[test]
    fn it_works() {
        let json = "{
            \"string\": \"hello world\",
            \"int\": 1234567890,
            \"float\": 12345.67890,
            \"localtime\": \"\"
        }";

        let deserialized: Example = serde_json::from_str(json).unwrap();

        assert_eq!(
            deserialized,
            Example {
                string: "hello world".to_string(),
                int: 1234567890,
                float: 12345.67890,
            }
        )
    }
}

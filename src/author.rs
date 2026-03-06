use crate::error::Error;

/// Implements an author
#[derive(Debug, Clone)]
pub struct Author {
    pub author_id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

impl Author {
    pub fn new_from_json(j: &serde_json::Value) -> Result<Author, Error> {
        if !j.is_object() {
            return Err(Error::InvalidJson(format!(
                "JSON for Author::new_from_json is not an object: {}",
                j
            )));
        }
        Ok(Author {
            author_id: j["authorId"].as_str().map(str::to_owned),
            name: j["name"].as_str().map(str::to_owned),
            url: j["url"].as_str().map(str::to_owned),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_author_from_valid_json() {
        let j = json!({
            "authorId": "123",
            "name": "Jane Doe",
            "url": "https://example.com/jane"
        });
        let author = Author::new_from_json(&j).unwrap();
        assert_eq!(author.author_id, Some("123".to_string()));
        assert_eq!(author.name, Some("Jane Doe".to_string()));
        assert_eq!(author.url, Some("https://example.com/jane".to_string()));
    }

    #[test]
    fn test_author_from_json_with_missing_fields() {
        let j = json!({});
        let author = Author::new_from_json(&j).unwrap();
        assert_eq!(author.author_id, None);
        assert_eq!(author.name, None);
        assert_eq!(author.url, None);
    }

    #[test]
    fn test_author_from_json_with_null_fields() {
        let j = json!({
            "authorId": null,
            "name": null,
            "url": null
        });
        let author = Author::new_from_json(&j).unwrap();
        assert_eq!(author.author_id, None);
        assert_eq!(author.name, None);
        assert_eq!(author.url, None);
    }

    #[test]
    fn test_author_from_non_object_json() {
        let j = json!("not an object");
        let err = Author::new_from_json(&j).unwrap_err();
        assert!(matches!(err, Error::InvalidJson(_)));
    }

    #[test]
    fn test_author_from_array_json() {
        let j = json!([1, 2, 3]);
        let err = Author::new_from_json(&j).unwrap_err();
        assert!(matches!(err, Error::InvalidJson(_)));
    }

    #[test]
    fn test_author_clone() {
        let j = json!({"authorId": "42", "name": "Test"});
        let author = Author::new_from_json(&j).unwrap();
        let cloned = author.clone();
        assert_eq!(author.author_id, cloned.author_id);
        assert_eq!(author.name, cloned.name);
        assert_eq!(author.url, cloned.url);
    }

    #[test]
    fn test_author_debug() {
        let j = json!({"authorId": "1", "name": "A"});
        let author = Author::new_from_json(&j).unwrap();
        let debug = format!("{:?}", author);
        assert!(debug.contains("Author"));
        assert!(debug.contains("\"1\""));
    }
}

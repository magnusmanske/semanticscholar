use crate::error::Error;

/// Implements a topic
#[derive(Debug, Clone, PartialEq)]
pub struct Topic {
    pub topic: Option<String>,
    pub topic_id: Option<String>,
    pub url: Option<String>,
}

impl Topic {
    pub fn new_from_json(j: &serde_json::Value) -> Result<Topic, Error> {
        if !j.is_object() {
            return Err(Error::InvalidJson(format!(
                "JSON for Topic::new_from_json is not an object: {}",
                j
            )));
        }
        Ok(Topic {
            topic_id: j["topicId"].as_str().map(str::to_owned),
            topic: j["topic"].as_str().map(str::to_owned),
            url: j["url"].as_str().map(str::to_owned),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_topic_from_valid_json() {
        let j = json!({
            "topicId": "T1",
            "topic": "Machine Learning",
            "url": "https://example.com/ml"
        });
        let topic = Topic::new_from_json(&j).unwrap();
        assert_eq!(topic.topic_id, Some("T1".to_string()));
        assert_eq!(topic.topic, Some("Machine Learning".to_string()));
        assert_eq!(topic.url, Some("https://example.com/ml".to_string()));
    }

    #[test]
    fn test_topic_from_json_with_missing_fields() {
        let j = json!({});
        let topic = Topic::new_from_json(&j).unwrap();
        assert_eq!(topic.topic_id, None);
        assert_eq!(topic.topic, None);
        assert_eq!(topic.url, None);
    }

    #[test]
    fn test_topic_from_json_with_null_fields() {
        let j = json!({
            "topicId": null,
            "topic": null,
            "url": null
        });
        let topic = Topic::new_from_json(&j).unwrap();
        assert_eq!(topic.topic_id, None);
        assert_eq!(topic.topic, None);
        assert_eq!(topic.url, None);
    }

    #[test]
    fn test_topic_from_non_object_json() {
        let j = json!("not an object");
        let err = Topic::new_from_json(&j).unwrap_err();
        assert!(matches!(err, Error::InvalidJson(_)));
    }

    #[test]
    fn test_topic_from_array_json() {
        let j = json!([1, 2, 3]);
        let err = Topic::new_from_json(&j).unwrap_err();
        assert!(matches!(err, Error::InvalidJson(_)));
    }

    #[test]
    fn test_topic_from_number_json() {
        let j = json!(42);
        let err = Topic::new_from_json(&j).unwrap_err();
        assert!(matches!(err, Error::InvalidJson(_)));
    }

    #[test]
    fn test_topic_clone() {
        let j = json!({"topicId": "T1", "topic": "AI"});
        let topic = Topic::new_from_json(&j).unwrap();
        let cloned = topic.clone();
        assert_eq!(topic.topic_id, cloned.topic_id);
        assert_eq!(topic.topic, cloned.topic);
        assert_eq!(topic.url, cloned.url);
    }

    #[test]
    fn test_topic_debug() {
        let j = json!({"topicId": "T1", "topic": "AI"});
        let topic = Topic::new_from_json(&j).unwrap();
        let debug = format!("{:?}", topic);
        assert!(debug.contains("Topic"));
        assert!(debug.contains("\"T1\""));
    }
}

mod author;
mod client;
mod error;
mod topic;
mod work;

pub use author::Author;
pub use client::Client;
pub use error::Error;
pub use topic::Topic;
pub use work::Work;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_full_work_roundtrip() {
        let j = json!({
            "arxivId": "1234.5678",
            "authors": [
                {"authorId": "1", "name": "Alice", "url": "https://example.com/alice"}
            ],
            "citationVelocity": 10,
            "citations": [
                {"paperId": "cite1", "title": "Citation 1"}
            ],
            "doi": "10.1234/test",
            "influentialCitationCount": 5,
            "paperId": "abc123",
            "references": [
                {"paperId": "ref1", "title": "Reference 1"}
            ],
            "title": "A Great Paper",
            "topics": [
                {"topicId": "T1", "topic": "AI", "url": "https://example.com/ai"}
            ],
            "url": "https://example.com/paper",
            "venue": "NeurIPS",
            "year": 2023
        });
        let work = Work::new_from_json(&j).unwrap();
        assert_eq!(work.paper_id, Some("abc123".to_string()));
        assert_eq!(work.authors.len(), 1);
        assert_eq!(work.authors[0].name, Some("Alice".to_string()));
        assert_eq!(work.topics.len(), 1);
        assert_eq!(work.topics[0].topic, Some("AI".to_string()));
        assert_eq!(work.citations.len(), 1);
        assert_eq!(work.references.len(), 1);
    }

    #[test]
    fn test_public_api_exports() {
        // Ensure all public types are accessible from the crate root
        let _client = Client::new();
        let _client_default = Client::default();

        let author_json = json!({"authorId": "1", "name": "Test"});
        let _author = Author::new_from_json(&author_json).unwrap();

        let topic_json = json!({"topicId": "T1", "topic": "Test"});
        let _topic = Topic::new_from_json(&topic_json).unwrap();

        let work_json = json!({"paperId": "P1"});
        let _work = Work::new_from_json(&work_json).unwrap();

        let _err = Error::InvalidJson("test".to_string());
        let _err2 = Error::Api("test".to_string());
    }
}

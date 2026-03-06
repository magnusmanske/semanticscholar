use crate::author::Author;
use crate::error::Error;
use crate::topic::Topic;

/// Implements a work (=paper)
#[derive(Debug, Clone, PartialEq)]
pub struct Work {
    pub arxiv_id: Option<String>,
    pub authors: Vec<Author>,
    pub citation_velocity: Option<u64>,
    pub citations: Vec<Work>,
    pub doi: Option<String>,
    pub influential_citation_count: Option<u64>,
    pub paper_id: Option<String>,
    pub references: Vec<Work>,
    pub title: Option<String>,
    pub topics: Vec<Topic>,
    pub url: Option<String>,
    pub venue: Option<String>,
    pub year: Option<u64>,
}

const EMPTY_ARRAY: &[serde_json::Value] = &[];

impl Work {
    /// Creates a new `Work` from a JSON value.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidJson` if the provided JSON value is not an object,
    /// or if any nested author, topic, or reference fails to parse.
    pub fn new_from_json(j: &serde_json::Value) -> Result<Work, Error> {
        if !j.is_object() {
            return Err(Error::InvalidJson(format!(
                "JSON for Work::new_from_json is not an object: {j}"
            )));
        }

        let authors = j["authors"]
            .as_array()
            .map_or(EMPTY_ARRAY, Vec::as_slice)
            .iter()
            .map(Author::new_from_json)
            .collect::<Result<Vec<_>, _>>()?;

        let topics = j["topics"]
            .as_array()
            .map_or(EMPTY_ARRAY, Vec::as_slice)
            .iter()
            .map(Topic::new_from_json)
            .collect::<Result<Vec<_>, _>>()?;

        let citations = match j["citations"].as_array() {
            Some(arr) => arr
                .iter()
                .filter_map(|paper| Work::new_from_json(paper).ok())
                .collect(),
            None => Vec::new(),
        };

        let references = match j["references"].as_array() {
            Some(arr) => arr
                .iter()
                .map(Work::new_from_json)
                .collect::<Result<Vec<_>, _>>()?,
            None => Vec::new(),
        };

        Ok(Work {
            arxiv_id: j["arxivId"].as_str().map(str::to_owned),
            authors,
            citation_velocity: j["citationVelocity"].as_u64(),
            citations,
            doi: j["doi"].as_str().map(str::to_owned),
            influential_citation_count: j["influentialCitationCount"].as_u64(),
            paper_id: j["paperId"].as_str().map(str::to_owned),
            references,
            title: j["title"].as_str().map(str::to_owned),
            topics,
            url: j["url"].as_str().map(str::to_owned),
            venue: j["venue"].as_str().map(str::to_owned),
            year: j["year"].as_u64(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample_work_json() -> serde_json::Value {
        json!({
            "arxivId": "1234.5678",
            "authors": [
                {"authorId": "1", "name": "Alice", "url": "https://example.com/alice"},
                {"authorId": "2", "name": "Bob", "url": null}
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
        })
    }

    #[test]
    fn test_work_from_valid_json() {
        let j = sample_work_json();
        let work = Work::new_from_json(&j).unwrap();
        assert_eq!(work.arxiv_id, Some("1234.5678".to_string()));
        assert_eq!(work.authors.len(), 2);
        assert_eq!(work.citation_velocity, Some(10));
        assert_eq!(work.citations.len(), 1);
        assert_eq!(work.doi, Some("10.1234/test".to_string()));
        assert_eq!(work.influential_citation_count, Some(5));
        assert_eq!(work.paper_id, Some("abc123".to_string()));
        assert_eq!(work.references.len(), 1);
        assert_eq!(work.title, Some("A Great Paper".to_string()));
        assert_eq!(work.topics.len(), 1);
        assert_eq!(work.url, Some("https://example.com/paper".to_string()));
        assert_eq!(work.venue, Some("NeurIPS".to_string()));
        assert_eq!(work.year, Some(2023));
    }

    #[test]
    fn test_work_from_minimal_json() {
        let j = json!({});
        let work = Work::new_from_json(&j).unwrap();
        assert_eq!(work.arxiv_id, None);
        assert!(work.authors.is_empty());
        assert_eq!(work.citation_velocity, None);
        assert!(work.citations.is_empty());
        assert_eq!(work.doi, None);
        assert_eq!(work.influential_citation_count, None);
        assert_eq!(work.paper_id, None);
        assert!(work.references.is_empty());
        assert_eq!(work.title, None);
        assert!(work.topics.is_empty());
        assert_eq!(work.url, None);
        assert_eq!(work.venue, None);
        assert_eq!(work.year, None);
    }

    #[test]
    fn test_work_from_non_object_json() {
        let j = json!("not an object");
        let err = Work::new_from_json(&j).unwrap_err();
        assert!(matches!(err, Error::InvalidJson(_)));
    }

    #[test]
    fn test_work_from_array_json() {
        let j = json!([1, 2, 3]);
        let err = Work::new_from_json(&j).unwrap_err();
        assert!(matches!(err, Error::InvalidJson(_)));
    }

    #[test]
    fn test_work_citations_skip_invalid() {
        // Citations silently skip entries that fail to parse
        let j = json!({
            "citations": [
                {"paperId": "cite1", "title": "Valid"},
                "invalid_citation",
                {"paperId": "cite2", "title": "Also Valid"}
            ]
        });
        let work = Work::new_from_json(&j).unwrap();
        assert_eq!(work.citations.len(), 2);
    }

    #[test]
    fn test_work_references_propagate_errors() {
        // References propagate parse errors
        let j = json!({
            "references": [
                {"paperId": "ref1", "title": "Valid"},
                "invalid_reference"
            ]
        });
        let err = Work::new_from_json(&j).unwrap_err();
        assert!(matches!(err, Error::InvalidJson(_)));
    }

    #[test]
    fn test_work_authors_propagate_errors() {
        // Authors propagate parse errors
        let j = json!({
            "authors": [
                {"authorId": "1", "name": "Valid"},
                "invalid_author"
            ]
        });
        let err = Work::new_from_json(&j).unwrap_err();
        assert!(matches!(err, Error::InvalidJson(_)));
    }

    #[test]
    fn test_work_topics_propagate_errors() {
        // Topics propagate parse errors
        let j = json!({
            "topics": [
                {"topicId": "T1", "topic": "Valid"},
                "invalid_topic"
            ]
        });
        let err = Work::new_from_json(&j).unwrap_err();
        assert!(matches!(err, Error::InvalidJson(_)));
    }

    #[test]
    fn test_work_nested_citations() {
        let j = json!({
            "paperId": "outer",
            "citations": [
                {
                    "paperId": "inner",
                    "citations": [
                        {"paperId": "deep"}
                    ]
                }
            ]
        });
        let work = Work::new_from_json(&j).unwrap();
        assert_eq!(work.citations.len(), 1);
        assert_eq!(work.citations[0].citations.len(), 1);
        assert_eq!(
            work.citations[0].citations[0].paper_id,
            Some("deep".to_string())
        );
    }

    #[test]
    fn test_work_null_arrays() {
        let j = json!({
            "authors": null,
            "topics": null,
            "citations": null,
            "references": null
        });
        let work = Work::new_from_json(&j).unwrap();
        assert!(work.authors.is_empty());
        assert!(work.topics.is_empty());
        assert!(work.citations.is_empty());
        assert!(work.references.is_empty());
    }

    #[test]
    fn test_work_clone() {
        let j = sample_work_json();
        let work = Work::new_from_json(&j).unwrap();
        let cloned = work.clone();
        assert_eq!(work.paper_id, cloned.paper_id);
        assert_eq!(work.title, cloned.title);
        assert_eq!(work.authors.len(), cloned.authors.len());
    }

    #[test]
    fn test_work_debug() {
        let j = json!({"paperId": "test123"});
        let work = Work::new_from_json(&j).unwrap();
        let debug = format!("{:?}", work);
        assert!(debug.contains("Work"));
        assert!(debug.contains("test123"));
    }

    #[test]
    fn test_work_empty_arrays() {
        let j = json!({
            "authors": [],
            "topics": [],
            "citations": [],
            "references": []
        });
        let work = Work::new_from_json(&j).unwrap();
        assert!(work.authors.is_empty());
        assert!(work.topics.is_empty());
        assert!(work.citations.is_empty());
        assert!(work.references.is_empty());
    }

    #[test]
    fn test_work_year_as_non_u64() {
        // If year is a string instead of a number, it should be None
        let j = json!({"year": "2023"});
        let work = Work::new_from_json(&j).unwrap();
        assert_eq!(work.year, None);
    }
}

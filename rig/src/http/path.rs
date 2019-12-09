use std::collections::HashMap;

pub type PathParser = PathSegments;

pub struct PathSegments {
    segments: Vec<Segment>,
    pub pattern: String,
}

impl PathSegments {
    pub fn new(pattern: &str) -> Self {
        let spited: Vec<&str> = pattern[1..].split("/").collect();
        let segments = spited.iter()
            .map(|s| {
                if s.starts_with("{") && s.ends_with("}") {
                    if s.starts_with("{*") {
                        Segment::Wildcard(s[2..s.len() - 1].to_string())
                    } else {
                        Segment::Variable(s[1..s.len() - 1].to_string())
                    }
                } else {
                    Segment::Constant(s.to_string())
                }
            }).collect();

        PathSegments {
            segments,
            pattern: pattern[1..].to_string(),
        }
    }

    pub fn parse(&self, path: &str) -> (bool, HashMap<String, String>) {
        let spited: Vec<&str> = path[1..].split("/").collect();
        let mut values: HashMap<String, String> = HashMap::default();
        let need_segments = spited.len();
        if self.segments.len() > need_segments {
            return (false, values);
        }

        for i in 0..spited.len() {
            if i > self.segments.len() {
                return (false, HashMap::default());
            }
            let segment: &Segment = &self.segments[i];
            match segment {
                Segment::Constant(cons) => {
                    if cons != spited[i] {
                        return (false, HashMap::default());
                    }
                }
                Segment::Variable(key) => {
                    values.insert(key.clone(), spited[i].to_string());
                }
                Segment::Wildcard(key) => {
                    values.insert(key.clone(), spited[i..].join("/"));
                    break;
                }
            }
        }

        (true, values)
    }
}

#[derive(Debug, PartialEq)]
enum Segment {
    Constant(String),
    Variable(String),
    Wildcard(String),
}

#[cfg(test)]
mod tests {
    use super::{PathSegments, Segment};

    #[test]
    pub fn test_path_segments() {
        let segments = PathSegments::new("/test/{version}");
        assert_eq!(vec![Segment::Constant("test".to_string()), Segment::Variable("version".to_string())], segments.segments);

        let segments = PathSegments::new("/test/{version}/{*tails}");
        assert_eq!(vec![Segment::Constant("test".to_string()), Segment::Variable("version".to_string()), Segment::Wildcard("tails".to_string())], segments.segments);
    }

    #[test]
    pub fn test_parse_one() {
        let map = PathSegments::new("/test/{version}").parse("/test/123");
        assert!(true, map.0);
        assert_eq!("123", map.1.get("version").unwrap());

        let map = PathSegments::new("/test/{book}/{author}").parse("/test/make-a-doctor/jackson");
        assert!(true, map.0);
        assert_eq!("make-a-doctor", map.1.get("book").unwrap());
        assert_eq!("jackson", map.1.get("author").unwrap());

        let map = PathSegments::new("/test/{*args}").parse("/test/123");
        assert!(true, map.0);
        assert_eq!("123", map.1.get("args").unwrap());

        let map = PathSegments::new("/test/{*args}").parse("/test/123/456");
        assert!(true, map.0);
        assert_eq!("123/456", map.1.get("args").unwrap());

        let map = PathSegments::new("/test/{version}/{other}").parse("/test/123");
        assert!(!map.0);
        assert!(map.1.is_empty());
    }
}
mod segment;

use std::path::Path;
use std::path::PathBuf;
use std::collections::HashSet;


pub struct Index {
    index_path: PathBuf,
    segments: Vec<segment::Segment>,
}


impl Index {
    pub fn new(base_dir: &Path) -> Index {
        let mut index_path = PathBuf::from(base_dir);
        index_path.push("indices");
        index_path.push("indexname");

        return Index {
            index_path: index_path,
            segments: Vec::new(),
        }
    }

    pub fn create(&mut self, id: &str, text: &str) {
        let segment = self.select_segment();
        segment.index_text(id, text);
    }

    pub fn search(&self, term: &str) -> HashSet<String> {

        let mut doc_matches: HashSet<String> = HashSet::new();
        for segment in self.segments.iter() {
            let segment_matches = segment.search_term(term);
            for doc_id in segment_matches.iter() {
                doc_matches.insert(doc_id.to_owned());
            }
        };

        return doc_matches;
    }

    fn select_segment(&mut self) -> &segment::Segment {
        if self.segments.is_empty() {
            let fresh_segment = segment::Segment::new(&self.index_path);
            self.segments.push(fresh_segment);
        }
        return self.segments.last().unwrap();
    }
}

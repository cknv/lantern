extern crate sha1;

use std::error::Error;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::path::Path;
use std::collections::HashSet;


pub struct Segment {
    segment_path: PathBuf,
}

impl Segment {
    pub fn new(base_dir: &Path) -> Segment {
        let mut segment_path = PathBuf::from(base_dir);
        segment_path.push("segments");
        segment_path.push("somename");
        let s = Segment {
            segment_path: segment_path,
        };

        match fs::create_dir_all(s.segment_path.as_path()) {
            Err(why) => panic!("could not create {}: {}", s.segment_path.display(), why.description()),
            Ok(__) => __,
        };

        return s;
    }

    pub fn index_text(&self, id: &str, text: &str) {
        let vec: Vec<&str> = text.split_whitespace().collect();

        for (position, term) in vec.iter().enumerate() {

            let term_hash = self.hash_token(term);
            println!("{} at {} is {}", term, position, term_hash);

            let mut token_file = PathBuf::new();
            token_file.push(self.segment_path.as_path());
            token_file.push(term_hash);

            println!("{}", token_file.display());

            let mut file = match OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(token_file) {
                 Err(why) => panic!("couldn't open token file: {}", why.description()),
                Ok(file) => file,
            };

            let row = format!("{}\t{}\n", id, position);
            println!("{}", row);
        }
    }

    pub fn search_term(&self, term: &str) -> HashSet<String> {
        let token_hash = self.hash_token(term);
        let mut token_file = PathBuf::new();
        token_file.push(self.segment_path.as_path());
        token_file.push(token_hash);

        let file = match File::open(token_file) {
            Err(why) => panic!("couldn't open token file: {}", why.description()),
            Ok(file) => file,
        };

        let mut id_matches: HashSet<String> = HashSet::new();

        let reader = BufReader::new(file);
        for row in reader.lines() {
            let row_str = row.unwrap();
            let id: Vec<&str> = row_str.split("\t").collect();
            id_matches.insert(id[0].to_string());
        }
        return id_matches;
    }

    fn hash_token(&self, token: &str) -> String {
        let mut shaer = sha1::Sha1::new();
        shaer.update(token.as_bytes());
        let term_hash = shaer.digest().to_string();
        shaer.reset();
        return term_hash;
    }
}

extern crate lantern;
extern crate tempdir;

use tempdir::TempDir;


#[test]
fn basic_indexing_and_search() {
    let base_path = TempDir::new("index-path").unwrap();
    let mut index = lantern::Index::new(base_path.path());
    index.create("id_1", "this is some text");
    assert!(index.search("some").contains("id_1"));
}

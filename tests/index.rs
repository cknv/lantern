extern crate lucerne;
extern crate tempdir;

use tempdir::TempDir;


#[test]
fn it_works() {
    let base_path = TempDir::new("index-path").unwrap();
    let mut index = lucerne::Index::new(base_path.path());
    index.create("id_1", "this is some text");
    assert!(index.search("some").contains("id_1"));
}

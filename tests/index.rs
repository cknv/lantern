extern crate lucerne;

use std::path::Path;


#[test]
fn it_works() {
    let base_path = Path::new("./");
    let mut index = lucerne::Index::new(base_path);
    index.create("id_1", "this is some text");
    assert!(index.search("some").contains("id_1"));
}

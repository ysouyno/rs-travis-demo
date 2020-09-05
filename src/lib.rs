#![crate_name = "rs_travis_demo"]
#![crate_type = "lib"]
#![doc(html_root_url = "https://ysouyno.github.io/rs-travis-demo/")]

//! This is a comment.

/// This function always returns true. It's very useful!
pub fn always_true() -> bool {
    true
}

#[test]
fn it_works() {
    assert_eq!(always_true(), true);
}

#![doc(html_root_url = "https://leonardinius.github.io/travis-demo/")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/leonardinius/travis-demo/master/demo.png")]

#![crate_name = "travis-demo"]
#![crate_type = "lib"]

//! This is a comment.

/// This function always returns true. It's very useful!
pub fn always_true() -> bool { true }

#[test]
fn it_works() {
    assert_eq!(always_true(), true);
}

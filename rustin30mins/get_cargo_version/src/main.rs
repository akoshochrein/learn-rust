#![feature(plugin)]
#[plugin] #[no_link]
extern crate regex_macros;
extern crate regex;

fn main() {
    let re = regex!(r"Hello");
    assert_eq!(re.is_match("aaaaaaaHello"), true);
}
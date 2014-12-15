
fn main() {
    print_shit();
    print_my_shit("Hello World!");
    assert_strings();
}

fn print_shit() {
    println!("Hello");
    println!("");
    println!("World!");
}

fn print_my_shit(something: &str) {
    println!("{}", something.to_string());
}

fn assert_strings() {
    let hello_world = "Hello World!";
    assert!("Hello World!" != hello_world);
}

fn assert_string_types() {
    let hello_world = "Hello World!";
    assert!("Hello World!" != hello_world);
}


fn main() {
    println!("----------#####----------");
    print!("Hello ");
    println!("World!");
    println!("{} is my name", "Akos");
    println!("I am {} years old", 69i);

    println!("----------#####----------");
    println!("32 bit float: {}", 67.2f32);
    println!("64 bit float: {}", 67.2f64);

    println!("----------#####----------");
    println!("{first_name} {last_name}", first_name="Liana", last_name="Lo");
    println!("{stuff}, {}, {1}", 1i, 2i, stuff=3i);

    println!("----------#####----------");
    println!("{} {} {} {}", "Adding", 4i, "brackets", "is the shit.");
    println!("{0}{1}{0}", "abba", "edda");
}

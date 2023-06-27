fn main() {
    println!("Hello, world!");

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // error: value borrowed here after move
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn test_mutable_variables() {
    let mut m = 5;
    println!("The value of m is: {m}");
    m = 6;
    println!("The value of m is: {m}");
}

fn test_constants() {
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
}

fn test_shadowing() {
    let x = 5;
    println!("1. The value of x is: {x}");
    let x = x + 1;
    println!("2. The value of x is: {x}");

    {
        let x = x * 2;
        println!("3. The value of x is: {x}");
    }
    println!("4. The value of x is: {x}");
}

fn main() {
    test_mutable_variables();
    println!("------------------");
    test_constants();
    println!("------------------");
    test_shadowing();
}

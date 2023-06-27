fn main() {
    // Function
    another_function();
    another_function_with_param(5);
    another_function_with_param_and_return(5);
    another_function_with_param_and_return2(5);
    another_function_with_param_and_return3(5);
    another_function_with_param_and_return4(5);

    // Statements and Expressions
    let y = {
        let x = 3;
        x + 1
    };
    println!("y: {}", y);

    // Function with return value
    let x = five();
    println!("The value of x is: {}", x);

    // Function with return value
    let x = plus_one(5);
    println!("The value of x is: {}", x);

    // Function with return value
    let x = plus_one(plus_one(5));
    println!("The value of x is: {}", x);
}


fn another_function() {
    println!("Another function.");
}

fn another_function_with_param(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function_with_param_and_return(x: i32) -> i32 {
    x + 1
}

fn another_function_with_param_and_return2(x: i32) -> i32 {
    return x + 1;
}

fn another_function_with_param_and_return3(x: i32) -> i32 {
    x + 1
}

fn another_function_with_param_and_return4(x: i32) -> i32 {
    return x + 1;
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}



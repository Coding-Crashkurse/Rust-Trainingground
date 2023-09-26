fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let f = false;
    let mut x = 4;

    // let z = x / f; // does not work
    // println!({z})

    let number = 3;
    if number > 5 {
        println!("larger");
    } else {
        println!("not larger");
    }

    another_function(5);
    another_function(5);
    another_function(5);
    let abc = five(6);
    println!("{abc}");

}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five(x: i32) -> i32 {
    return x;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn test {
    let user = User {
        active: true,
        username: String::from("someuser"),
        email: String::from("someemail"),
        sign_in_count: 1
    }
}
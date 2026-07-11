fn main() {
    println!("Hello world!");
    another_function(50);

    println!("The value of five is {}", five());

    let y = {
        // not an expression
        let x = 2;
        // not an expression: ends with ;
        x + 1;
        // expression:
        x + 1
    };
    println!("y: {y}");

    let mut x = 1;
    x = plus_one(x);

    println!("Value of x: {x}");

    plus_one_in_place_wrong(x);
    println!("x after wrong in place: {x}");

    plus_one_in_place(&mut x);
    println!("x after right in place: {x}");

}

fn another_function(x: i32) {
    println!("Value of x is {x}");
}

// functions return the last expression in its body.
// if a function returns something, its return type shall be anotated.
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32{
    return x + 1
}

// this creates a mutable copy of the argument
fn plus_one_in_place_wrong(mut x: i32) {
    x += 1;
}

// this receives a mutable reference to the argument
fn plus_one_in_place(x: &mut i32) {
    // need to dereference because we're dealing with a reference
    *x += 1;
}
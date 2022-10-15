fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(5, 'h');

    // Statements and expressions
    let y = {
        let x = 3;
        x + 1 // there is no semicolon, so it returns the value
    };

    println!("The value of y is: {y}");

    // Functions with Return Values
    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5 // no semicolon, so it returns 5 implicitly
}

fn plus_one(x: i32) -> i32 {
    x + 1 // no semicolon, so it returns x + 1
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

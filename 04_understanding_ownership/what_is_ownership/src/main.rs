fn main() {
    // The String type
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    // Ways Variables nad Data Interact: Move

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); // compile error  ^^ value borrowed here after move

    // Ways Variables nad Data Interact: Clone

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // Ownership and Functions
    let s = String::from("hello");

    takes_ownership(s);

    // println!("{s}"); compile error: value borrowed here after move

    let x = 5;
    makes_copy(x);

    // Return Values and Scope
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    // tuple

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len)
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and 'drop' is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

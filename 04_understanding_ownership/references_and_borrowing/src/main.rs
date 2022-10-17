fn main() {
    // References and Borrowing
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // Mutable References

    let mut s = String::from("hello");

    change(&mut s);

    // cannot borrow `s` as mutable more than once at a time
    let mut s = String::from("hello");

    let r1 = &mut s;
    //let r2 = &mut s; // compile error cannot borrow `s` as mutable more than once at a time
    //
    //println!("{}, {}", r1, r2)

    // multiple mutable references
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    // Combining mutable and immutable references
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    // compile error: cannot borrow `s` as mutable because it is also borrowed as immutable
    //let r3 = &mut s;     // BIG PROBLEM

    //println!("{}, {}, and {}", r1, r2, r3);

    // mutable reference after println!
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Dangling References
    //  let reference_to_nothing = dange();
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

// compile error: return type contains a borrowed value, but there is no value for it to be borrowed from
//fn dangle() -> &String {
//    let s = String::from("hello");
//
//    &s
//}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn change_1(some_string: &String) {
    // compile error cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    // some_string.push_str(", world");
}

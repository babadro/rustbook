fn main() {
    // example 1
    let mut a = Box::new(5);
    let mut b = &mut a;
    let mut c = &mut b;

    // example 2
    let a = String::new();
    //a.push_str("asd") // compile error

    // example 3
    let mut a = Box::new(1);

    *a = 2;

    println!("{:?}", *a);

    // example 4
    let a = Box::new(2);

    // *a = 4; // compile error

    let mut b = &a;

    println!("{:?}", a);

    // example 5
    let mut a = Box::new(10);

    let b = &mut a;

    **b = 3;

    println!("{:?}", a);

    play_with_clone()
}

fn play_with_clone() {
    let a = Some(Box::new(10));

    let as_ref = a.as_ref();

    let cloned = as_ref.cloned();

    let mut unwrapped = cloned.unwrap();

    *unwrapped = 1;

    println!("{:?}", a);
    println!("{:?}", unwrapped);
}

fn my_func(a: &mut i32) {
    let mut b = a;
}

fn func2(a: String) {
    //a.push_str("asd") // compile error. Need to use mut a as arg
}

fn mut_mut_mut() {
    // 1: let not_mut = &mut
    let mut a = 1;

    let b = &mut a;

    *b = 3;

    // 2: let not_mut = mut, copy
    let mut a = 1;

    let b = a; // b was copied here

    a = 1; // no problem, a was not moved

    // b = 3; // compile error, b is immutable

    // 3: let not_mut = mut, no copy
    let a = Box::new(1);

    let b = a;

    // println!("{:?}", a); // compile error, a is moved

    // *b = 1; // compile error, b is immutable

    // 4: not move because copy
    let a = 1;
    accept_by_value(a);

    println!("{:?}", a); // no problem, a was copied inside accept_by_value

    // 5: move because cant copy
    let a = Box::new(1);

    accept_by_pointer(a);

    // println!("{:?}", a); // compile error, a was moved inside accept_by_pointer

    let c = Box::new(1);

    let d = c; // c moved to a because can't be copied

    // println!("{:?}", c); // compile error, c was moved to d
}

fn accept_by_value(a: i32) {
    println!("{:?}", a);
}

fn accept_by_pointer(a: Box<i32>) {
    println!("{:?}", a);
}

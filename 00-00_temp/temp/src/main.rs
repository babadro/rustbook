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

    println!("{:?}", a)
}

fn my_func(a: &mut i32) {
    let mut b = a;
}

fn func2(a: String) {
    //a.push_str("asd") // compile error. Need to use mut a as arg
}

fn main() {
    // Creating a New Vector
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    // Updating a Vector
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading Elements of Vectors
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // compile errors
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    // println!("The first element is: {}", first);  cannot borrow `v` as mutable because it is also borrowed as immutable

    // Iterating over the Values in a Vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Using an Enum to Store Multiple Types
    enum SpreadscheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadscheetCell::Int(3),
        SpreadscheetCell::Text(String::from("blue")),
        SpreadscheetCell::Float(10.12),
    ];

    // Dropping a Vector Drops Its Elements
    {
        let v = vec![1, 2, 3, 4];
    } // <- v goes out of scope and is freed here
}

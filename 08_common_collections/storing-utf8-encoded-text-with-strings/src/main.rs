fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");

    // Updating a String
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    // Concatenation with the + Operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    // Indexing into Strings
    let s1 = String::from("hello");
    // let h = s1[0] the type `String` cannot be indexed by `{integer}`

    // Internal Representation
    let hello = String::from("Hola");
    let hello = String::from("Здравствуйте");

    let hello = "Здравствуйте";
    // let answer = &hello[0]; the type `String` cannot be indexed by `{integer}`

    // Bytes and Scalar Values and Grapheme Clusters! Oh My!
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // ['न', 'म', 'स', '्', 'त', 'े']
    // ["न", "म", "स्", "ते"]

    // Slicing Strings
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    // Method for Iterating Over Strings
    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b)
    }
}

use std::fmt::Display;

fn main() {
    // The Borrow Checker
    let x = 5;

    let r = &x;

    println!("r: {}", r);

    // Lifetime Annotation Syntax
    /*
    &i32        // a reference
    &'a i32     // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime
    */

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // examples how the lifetime annotations restrict the longest function

    // valid example
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // invalid example, compile error: borrowed value does not live long enough
    /*
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string  is {}", result);
     */

    // Lifetime Annotations in Struct Definitions
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // Lifetime Annotations in Method Definitions
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }

        // example where the third lifetime elision rule applies:
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    // The Static Lifetime
    let s: &'static str = "I have a static lifetime.";

    // Generic Type Parameters, Trait Bounds, and Lifetimes Together
    use std::fmt::Display;
}

fn longest_with_an_annoumcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime Annotations in Function Signatures
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
// compile error: ^ expected named lifetime parameter
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

// compile error: ^^ borrowed value does not live long enough
/*
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r)
}
*/

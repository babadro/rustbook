use using_trait_objects_that_allow_for_values_of_different_types::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // do something
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Ok"),
            }),
        ],
    };

    screen.run();

    // compile error: the trait `Draw` is not implemented for `String`
    /*
    let screen = Screen {
        components: vec![Box::new(String::from("Hi"))],
    };

    screen.run();
     */
}

use std::fmt;

fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // another example
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long(f: Thunk) {
        // do something
    }

    fn returns_long_type() -> Thunk {
        Box::new(|| println!("hi"))
    }

    // Type aliases are also commonly used with the Result<T, E> type for reducing repetition.
    use std::fmt;
    use std::io::Result;

    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
    }

    // The Never Type that Never Returns
    //fn bar() -> ! {
    //
    //}
}

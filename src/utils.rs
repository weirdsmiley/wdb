//! Utils section provide macro to print the instructions instead of the
//! usual one address in each line.
//!

#[macro_export]
// TODO: Print such that each 'instruction' is on new line.
macro_rules! instprint {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{}};
}

#[macro_use]
macro_rules! say_hello {
    () => {
        // The macro will expand into the contents of this
        block.
            println!("Hello!");
    };
}

#[macro_use]
macro_rules! dump {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{}};
}

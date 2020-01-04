// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

// I AM _NOT DONE

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!(); // this is a declarative macro - they are called with the '!' character added to the end of the function/macro name. 
}

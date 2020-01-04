// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)

// I AM _NOT DONE

// macros must be defined before they are called. Code in rust is read top-down, so it makes sense in most scenarios to define them at the top of a module or file.
 
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
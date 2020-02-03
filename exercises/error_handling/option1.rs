// option1.rs
// This example panics because the second time it calls `pop`, the `vec`
// is empty, so `pop` returns `None`, and `unwrap` panics if it's called
// on `None`. Handle this in a more graceful way than calling `unwrap`!
// Execute `rustlings hint option1` for hints :)

// I AM _NOT DONE

pub fn pop_too_much() -> bool { 
    let mut list = vec![3];

    for _ in 0..2 {
        match list.pop() {
                Some(list) =>
                    println!("Latest item in list was: {:?}\n", list),
                None =>
                    println!("Hit end of list")
            }
    }
    /*
    let last = list.pop().unwrap();
    println!("The last item in the list is {:?}", last);

    let second_to_last = list.pop().unwrap();
    println!(
        "The second-to-last item in the list is {:?}",
        second_to_last
    );
    true
    */
    true
}

pub fn pop_wrap() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_panic() {
        assert!(pop_too_much());
    }
}

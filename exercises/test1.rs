// test1.rs
// This is a test for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 dollars, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!

// I AM _NOT DONE

// Put your function here!
fn calculate_apple_price(n: i32) {
    let price: i32 = if n > 40 {
            1 * n 
        } 
        else {
            2 * n
        };
}


//Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(calculate_apple_price(35), price1); // assert_eq! is not implemented for integers. This was trying to compare a literal integer to an expression/function. wtf guys. 
    assert_eq!(calculate_apple_price(65), price2); //
}
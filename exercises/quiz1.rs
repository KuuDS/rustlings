// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!


// Put your function here!
// fn ..... {
const APPLE_PRIZE : i32 = 2;
const CONDITION_SIZE : i32 = 40;
const OFF_PRICE : i32 = 1;
fn calculate_apple_price(size: i32) -> i32 {
    if size > CONDITION_SIZE {
        size
    } else {
        size * 2
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}

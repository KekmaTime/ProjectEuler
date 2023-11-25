use num_bigint::{BigUint, ToBigUint};
use num_traits::Zero;

fn main() {
    let limit = 100u32;

    let mut sum_of_squares = BigUint::zero();
    let mut sum = BigUint::zero();

    for i in 1u32..=limit {
        let number = i.to_biguint().unwrap();
        sum_of_squares += &number * &number;
        sum += number;
    }

    let square_of_sum = &sum * &sum;
    let difference = if sum_of_squares > square_of_sum {
        sum_of_squares - square_of_sum
    } else {
        square_of_sum - sum_of_squares
    };

    println!("The difference is: {}", difference);
}

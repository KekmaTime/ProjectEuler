fn palindrome(a: i64) -> bool {
    let mut temp = 0;
    let mut nums = a;
    while nums > 0 {
        temp = (temp * 10) + (nums % 10);
        nums = nums / 10;
    }
    a == temp
}

fn main() {
    let mut largest_palindrome = 0;
    let mut factors: (i64, i64) = (0, 0);

    for i in (100..=999).rev() {
        for j in (100..=999).rev() {
            let product = i * j;
            if palindrome(product) && product > largest_palindrome {
                largest_palindrome = product;
                factors = (i, j);
            }
        }
    }

    println!(
        "The largest palindrome made from the product of two three-digit numbers is {} ({} * {})",
        largest_palindrome, factors.0, factors.1
    );
}

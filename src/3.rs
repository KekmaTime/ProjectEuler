fn main() {
    let mut num: i64 = 600851475143;
    let mut largest: i64 = 1;
    let mut factor: i64 = 2;

    while factor * factor <= num {
        if num % factor == 0 {
            largest = factor;
            while num % factor == 0 {
                num /= factor;
            }
        }
        factor += 1;
    }

    if num > largest {
        largest = num;
    }

    println!("The Largest Prime Factor is: {}", largest);
}

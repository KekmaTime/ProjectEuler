fn main() {
    let mut sum = 0;
    let mut a = 1;
    let mut b = 2;

    while a <= 4_000_000 {
        if a % 2 == 0 {
            sum += a;
        }

        let temp = a + b;
        a = b;
        b = temp;
    }

    println!("The sum is : {}", sum);
}

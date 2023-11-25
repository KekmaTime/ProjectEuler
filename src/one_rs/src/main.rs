fn main() {
    let mut sum = 0;
    let mut temp = 0;
    while temp != 1000 {
        if temp % 3 == 0 || temp % 5 == 0 {
            sum = sum + temp;
        }
        temp += 1;
    }
    println!("sum is : {}", sum);
}

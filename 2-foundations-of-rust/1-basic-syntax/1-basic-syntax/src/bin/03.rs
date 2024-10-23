fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];
    let mut min = input[0];
    let mut max = input[0];
    for i in input {
        if i > max {
            max = i;
        }
        if i < min {
            min = i;
        }
    }

    println!("{max} is largest and {min} is smallest");
}

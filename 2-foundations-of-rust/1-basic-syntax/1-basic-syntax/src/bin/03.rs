fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let mut max = input[0];
    let mut min = input[0];
    // TODO
    for i in 1..7 {
        if max < input[i] {
            max = input[i];
        }
        if min > input[i] {
            min = input[i];
        }
    }
    println!("{} is largest and {} is smallest", max, min);
}

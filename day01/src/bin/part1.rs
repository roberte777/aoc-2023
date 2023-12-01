fn main() {
    let test = include_str!("../input/input1.txt").to_string();
    run(test)
}
fn run(input: String) {
    let output: i32 = input
        .lines()
        .map(|s| {
            let mut temp = s.chars();
            let first = temp.find(|d| d.is_digit(10)).unwrap();
            let last = match temp.rev().find(|d| d.is_digit(10)) {
                Some(val) => val,
                None => first,
            };
            println!("{}{}", first, last);
            format!("{}{}", first, last).parse::<i32>().unwrap()
        })
        .sum();
    println!("{}", output);
}

fn main() {
    let test = include_str!("../input/input2.txt").to_string();
    run(test)
}
fn run(input: String) {
    let input = replace_nums(&input);
    let output: i32 = input
        .lines()
        .map(|s| {
            let digits: Vec<char> = s.chars().filter(|c| c.is_ascii_digit()).collect();

            let first = digits.first().unwrap();
            let last = digits.last().unwrap_or(first);

            format!("{}{}", first, last).parse::<i32>().unwrap()
        })
        .sum();
    println!("{}", output);
}

fn replace_nums(s: &str) -> String {
    let nums = [
        ("eightwo", "82"),
        ("eighthree", "83"),
        ("sevenine", "79"),
        ("fiveight", "58"),
        ("threeight", "38"),
        ("twone", "21"),
        ("nineight", "98"),
        ("oneight", "18"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let mut new_string = String::from(s);
    nums.iter().for_each(|(string, num)| {
        new_string = new_string.replace(string, num);
    });
    new_string
}

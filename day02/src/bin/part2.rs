use std::collections::HashMap;

fn main() {
    let test = include_str!("../input/input2.txt").to_string();
    run(test)
}
fn run(input: String) {
    let a: usize = input
        .lines()
        .map(|s| {
            let mut totals: HashMap<&str, usize> = HashMap::new();
            s.split(':')
                .last()
                .unwrap()
                .trim()
                .split(';')
                .flat_map(|s| {
                    s.split(", ")
                        .map(|s| {
                            let pair = s.trim().split(' ').collect::<Vec<&str>>();
                            let num = pair.first().unwrap().parse::<usize>().unwrap();
                            let color = pair.last().unwrap().to_owned();
                            (num, color)
                        })
                        .collect::<Vec<(usize, &str)>>()
                })
                .for_each(|pair| {
                    if let Some(totals_hash) = totals.get_mut(&pair.1 as &str) {
                        if pair.0 > *totals_hash {
                            *totals_hash = pair.0;
                        }
                    } else {
                        totals.insert(pair.1, pair.0);
                    }
                });
            let final_val = totals
                .iter()
                .map(|v| *v.1)
                .reduce(|acc, v| acc * v)
                .unwrap()
                .to_owned();
            final_val
        })
        .sum();

    dbg!(a);
}

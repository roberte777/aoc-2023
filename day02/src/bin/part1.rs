use std::collections::HashMap;

fn main() {
    let test = include_str!("../input/input1.txt").to_string();
    run(test)
}
fn run(input: String) {
    let mut vals = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let a: usize = input
        .lines()
        .map(|s| {
            let mut rounds = s.split(':');
            let id = rounds
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let truth_vals = rounds
                .last()
                .unwrap()
                .trim()
                .to_string()
                .split(';')
                .flat_map(|s| {
                    s.split(',')
                        .map(|s| {
                            let pair = s
                                .split(' ')
                                .filter(|s| !s.is_empty())
                                .map(|s| s.trim().to_string())
                                .collect::<Vec<String>>();
                            let num = pair.first().unwrap().parse::<usize>().unwrap();
                            let color = pair.last().unwrap().clone();
                            (num, color)
                        })
                        .collect::<Vec<(usize, String)>>()
                })
                .map(|pair| {
                    let mut totals: HashMap<String, usize> = HashMap::new();
                    match vals.get_mut(&pair.1 as &str) {
                        Some(hash) => {
                            if let Some(totals_hash) = totals.get_mut(&pair.1 as &str) {
                                *totals_hash += pair.0;
                                if totals_hash <= hash {
                                    return true;
                                }
                            } else {
                                totals.insert(pair.1.clone(), pair.0);
                                if pair.0 <= *hash {
                                    return true;
                                }
                                return false;
                            }
                            false
                        }
                        _ => false,
                    }
                })
                .collect::<Vec<bool>>();
            let final_val = match truth_vals
                .iter()
                .find(|val| val.to_owned().to_owned() == false)
            {
                Some(_) => false,
                None => true,
            };
            (id, final_val)
        })
        .filter(|val| val.1)
        .map(|val| dbg!(val.0))
        .sum();

    dbg!(a);
}

use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace1},
    sequence::delimited,
    IResult,
};

fn card_label(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, number) = digit1(input)?;
    let (input, _) = tag(":")(input)?;
    Ok((input, number))
}

fn number_list(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, numbers) = nom::multi::separated_list0(multispace1, digit1)(input)?;
    Ok((input, numbers))
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, card_num) = card_label(input)?;
    let (input, first_group) = delimited(multispace1, number_list, multispace1)(input)?;
    let (input, _) = char('|')(input)?;
    let (input, _) = multispace1(input)?;
    let (input, second_group) = number_list(input)?;
    let card = Card {
        name: card_num.parse::<u32>().unwrap(),
        winning_nums: first_group
            .iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect(),
        card_nums: second_group
            .iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect(),
    };
    Ok((input, card))
}

#[derive(Debug)]
struct Card {
    name: u32,
    winning_nums: Vec<u32>,
    card_nums: Vec<u32>,
}

fn main() {
    let input = include_str!("../input/input1.txt");
    let mut total = 0;
    let mut temp: HashMap<u32, u32> = HashMap::new();
    for line in input.lines() {
        let mut wins = 0;
        if let Ok((_, card)) = parse_card(line) {
            for num in card.winning_nums.iter() {
                if card.card_nums.contains(num) {
                    wins += 1;
                }
            }
            let entry = temp.entry(card.name).or_insert(0);
            *entry += 1;
            let multiplier = *entry;
            for i in 1..=wins {
                *temp.entry(card.name + i).or_insert(0) += multiplier;
            }
        } else if let Err(e) = parse_card(line) {
            println!("Failed to parse line: {}, {}", line, e);
        }
    }
    // sum up the values in the map
    for (_, v) in temp.iter() {
        total += v;
    }
    println!("Total: {}", total);
}

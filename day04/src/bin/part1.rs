use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace1},
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
    let (input, _) = multispace1(input)?;
    let (input, first_group) = number_list(input)?;
    let (input, _) = multispace1(input)?;
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
    for line in input.lines() {
        let mut wins = 0;
        let mut card_total = 0;
        if let Ok((_, card)) = parse_card(line) {
            for num in card.winning_nums.iter() {
                if card.card_nums.contains(num) {
                    wins += 1;
                    card_total = 2_u32.pow(wins - 1);
                }
            }
        } else if let Err(e) = parse_card(line) {
            println!("Failed to parse line: {}, {}", line, e);
        }
        total += card_total;
    }
    println!("Total: {}", total);
}

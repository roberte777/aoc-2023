use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, newline, space1},
    multi::separated_list1,
    sequence::{pair, terminated},
    IResult,
};

fn main() {
    let input = include_str!("../input/input1.txt");
    process(input);
}
fn process(input: &str) {
    let (input, seeds) = parse_seeds(input).unwrap();
}
fn parse_input(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, seed_to_soil_map) = parse_seed_to_soil_map(input)?;
    let (input, soil_to_fertilizer_map) = parse_soil_to_fertilizer_map(input)?;
    let (input, fertilizer_to_water_map) = parse_fertilizer_to_water_map(input)?;
    let (input, water_to_light_map) = parse_water_to_light_map(input)?;
    let (input, light_to_temperature_map) = parse_light_to_temperature_map(input)?;
    let (input, temperature_to_humidity_map) = parse_temperature_to_humidity_map(input)?;
    let (input, humidity_to_location_map) = parse_humidity_to_location_map(input)?;
    dbg!(&seeds);
    dbg!(seed_to_soil_map);
    Ok((input, seeds))
}
fn parse_seeds(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(pair(tag(" "), tag(" ")), tag(" "))(input)?;
    let (input, _) = newline(input)?;
    Ok((input, seeds))
}
fn parse_seed_to_soil_map(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, _) = tag("seed-to-soil map:")(input)?;
    let (input, seed_to_soil_map) = separated_list1(space1, digit1)(input)?;
    let (input, _) = newline(input)?;
    Ok((input, seed_to_soil_map))
}
fn parse_soil_to_fertilizer_map(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("soil-to-fertilizer map:")(input)?;
    let (input, soil_to_fertilizer_map) = separated_list1(space1, complete::u32)(input)?;
    let (input, _) = newline(input)?;
    Ok((input, soil_to_fertilizer_map))
}
fn parse_fertilizer_to_water_map(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("fertilizer-to-water map:")(input)?;
    let (input, fertilizer_to_water_map) = separated_list1(space1, complete::u32)(input)?;
    let (input, _) = newline(input)?;
    Ok((input, fertilizer_to_water_map))
}
fn parse_water_to_light_map(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("water-to-light map:")(input)?;
    let (input, water_to_light_map) = separated_list1(space1, complete::u32)(input)?;
    let (input, _) = newline(input)?;
    Ok((input, water_to_light_map))
}
fn parse_light_to_temperature_map(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("light-to-temperature map:")(input)?;
    let (input, light_to_temperature_map) = separated_list1(space1, complete::u32)(input)?;
    let (input, _) = newline(input)?;
    Ok((input, light_to_temperature_map))
}
fn parse_temperature_to_humidity_map(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("temperature-to-humidity map:")(input)?;
    let (input, temperature_to_humidity_map) = separated_list1(space1, complete::u32)(input)?;
    let (input, _) = newline(input)?;
    Ok((input, temperature_to_humidity_map))
}
fn parse_humidity_to_location_map(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, humidity_to_location_map) = tag("humidity-to-location map:")(input)?;
    let (input, humidity_to_location_map) =
        terminated(separated_list1(space1, complete::u32), newline)(input)?;
    let (input, _) = newline(input)?;
    Ok((input, humidity_to_location_map))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        process(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
    }
}

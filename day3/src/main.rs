use std::fs::File;
use std::io::read_to_string;
use std::io::Result;

fn read_input_file() -> Result<String> {
    let mut file = File::open("input.txt")?;
    Ok(read_to_string(&mut file)?)
}

fn get_rucksacks_from(input: &mut String) -> Vec<&str> {
    input.trim().split("\n").collect::<Vec<_>>()
}

fn get_compartments_from(rucksacks: Vec<&str>) -> Vec<(&str, &str)> {
    rucksacks
        .iter()
        .map(|x| x.trim().split_at(x.len() / 2))
        .collect::<Vec<_>>()
}

fn compare_compartments_part1(compartments: Vec<(&str, &str)>) -> usize {
    let mut sum = 0;

    compartments.iter().for_each(|(x, y)| {
        let common = x.chars().filter(|i| y.contains(*i)).next().unwrap();

        if common.is_lowercase() {
            sum += common as usize - 97 + 1;
        } else {
            sum += common as usize - 65 + 26 + 1;
        }
    });

    sum
}

fn compare_compartments_part2(rucksacks: Vec<&str>) -> usize {
    let mut sum = 0;

    rucksacks.chunks(3).for_each(|rucksack| {
        let common = rucksack[0]
            .chars()
            .filter(|x| rucksack[1].contains(*x))
            .filter(|x| rucksack[2].contains(*x))
            .next()
            .unwrap();

        if common.is_lowercase() {
            sum += common as usize - 97 + 1;
        } else {
            sum += common as usize - 65 + 26 + 1;
        }
    });

    sum
}

fn main() -> Result<()> {
    let mut input = read_input_file()?;

    let rucksacks = get_rucksacks_from(&mut input);
    let compartments = get_compartments_from(rucksacks.clone());
    println!("{}", compare_compartments_part1(compartments));
    println!("{}", compare_compartments_part2(rucksacks.clone()));

    Ok(())
}

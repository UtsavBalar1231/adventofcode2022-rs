use std::fs::File;
use std::io::read_to_string;
use std::io::Result;

fn read_input_file() -> Result<String> {
    let mut file = File::open("input.txt")?;
    Ok(read_to_string(&mut file)?)
}

fn get_rucksacks_from(input: &mut String) -> Vec<&str> {
    input.trim().split("\n").collect()
}

fn get_compartments_from(rucksacks: Vec<&str>) -> Vec<(&str, &str)> {
    rucksacks
        .iter()
        .map(|x| x.trim().split_at(x.len() / 2))
        .collect::<Vec<_>>()
}

fn compare_compartments(compartments: Vec<(&str, &str)>) -> usize {
    let mut sum = 0;

    for (x, y) in compartments {
        for i in x.chars() {
            if y.contains(i) {
                if i.is_lowercase() {
                    sum += i as usize - 97 + 1;
                } else {
                    sum += i as usize - 65 + 26 + 1;
                }
                break;
            }
        }
    }
    sum
}

fn main() -> Result<()> {
    let mut input = read_input_file()?;

    let rucksacks = get_rucksacks_from(&mut input);
    let compartments = get_compartments_from(rucksacks);
    println!("{}", compare_compartments(compartments));

    Ok(())
}

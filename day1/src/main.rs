use std::fs::File;
use std::io::{read_to_string, Result};

fn read_input_file() -> Result<String> {
    let mut file = File::open("input.txt")?;
    let content = read_to_string(&mut file)?;

    Ok(content)
}

fn parse_string_usize(input: &mut String) -> Result<Vec<usize>> {
    let mut list = input
        .split("\n\n")
        .collect::<Vec<_>>()
        .iter_mut()
        .map(|s| s.split("\n").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // remove the last element, which is an empty String
    if list.last().unwrap().last().unwrap().is_empty() {
        list.pop();
    }

    // convrt vec<vec<&str>> to vec<vec<usize>>
    let list = list
        .iter()
        .map(|s| {
            s.iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // convert vec<vec<usize>> to <vec<usize>> with sum of each element
    let mut list = list
        .iter()
        .map(|s| s.iter().sum::<usize>())
        .collect::<Vec<_>>();

    // sort the list
    list.sort_unstable();

    Ok(list)
}

fn main() -> Result<()> {
    let mut input = read_input_file()?;

    let calories_list = parse_string_usize(&mut input).unwrap();
    // part one
    println!("{:?}", calories_list.iter().last().unwrap());

    // part two
    println!(
        "{:?}",
        &calories_list[(calories_list.len() - 3)..]
            .iter()
            .sum::<usize>()
    );

    Ok(())
}

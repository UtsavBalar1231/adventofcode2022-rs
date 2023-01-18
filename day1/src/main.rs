use std::collections::BTreeMap;
use std::fs::File;
#[allow(unused_imports)]
use std::io::{read_to_string, Read, Result};
use std::usize;

#[allow(dead_code)]
fn read_input_file() -> Result<String> {
    let mut file = File::open("input.txt")?;
    let content = read_to_string(&mut file)?;

    Ok(content)
}

#[allow(dead_code)]
fn create_input_hashmap(input: &mut String) -> Result<BTreeMap<usize, Vec<usize>>> {
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

    let map = list
        .iter()
        .enumerate()
        .map(|l| {
            let key = l.0;
            let value =
                l.1.iter()
                    .map(|s| s.split(" ").collect::<Vec<_>>())
                    .flatten()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
            (key, value)
        })
        .collect::<BTreeMap<usize, Vec<_>>>();

    Ok(map)
}

#[allow(dead_code)]
fn create_hashmap_sum(map: BTreeMap<usize, Vec<usize>>) -> BTreeMap<usize, usize> {
    let mut sum_map = BTreeMap::new();

    for (key, value) in map {
        let sum = value.iter().sum();
        sum_map.insert(key, sum);
    }

    sum_map
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
    // let map = create_input_hashmap(&mut input)?;
    // let mut map = create_hashmap_sum(map);

    println!(
        "{:?}",
        parse_string_usize(&mut input)
            .unwrap()
            .iter()
            .last()
            .unwrap()
    );

    Ok(())
}

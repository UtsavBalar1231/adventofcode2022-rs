use std::fs::File;
use std::io::read_to_string;
use std::io::Result;

fn read_input_file() -> Result<String> {
    let mut file = File::open("input.txt")?;
    Ok(read_to_string(&mut file)?)
}

fn get_assignment_pairs(input: &mut String) -> Vec<Vec<Vec<usize>>> {
    let pairs = input
        .trim()
        .split("\n")
        .map(|x| {
            x.split(",")
                .map(|y| {
                    y.split("-")
                        .map(|z| z.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    pairs
}

fn get_overlaping_pairs(pairs: Vec<Vec<Vec<usize>>>, part: usize) -> usize {
    let mut count = 0;
    pairs.iter().for_each(|p| {
        let a_min = p[0].iter().min().unwrap();
        let b_min = p[1].iter().min().unwrap();
        let a_max = p[0].iter().max().unwrap();
        let b_max = p[1].iter().max().unwrap();

        if part == 1 {
            // part one
            if a_min <= b_min && a_max >= b_max
                || a_min >= b_min && a_max <= b_max
                || a_min == b_min && a_max == b_max
            {
                count += 1;
            }
        } else {
            // part two
            if a_min <= b_min && a_max >= b_max
                || a_min >= b_min && a_max <= b_max
                || a_min == b_min && a_max == b_max
                || a_min <= b_min && a_max >= b_min
                || a_min <= b_max && a_max >= b_max
            {
                count += 1;
            }
        }
    });
    count
}

fn main() -> Result<()> {
    let mut input = read_input_file()?;
    let pairs = get_assignment_pairs(&mut input);

    let count = get_overlaping_pairs(pairs.clone(), 1);
    println!("{count:?}");
    let count = get_overlaping_pairs(pairs, 2);
    println!("{count:?}");

    Ok(())
}

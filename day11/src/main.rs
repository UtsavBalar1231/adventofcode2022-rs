#![feature(int_roundings)]

use day11::monkey::Monkey;
use day11::operation::*;
use std::collections::VecDeque;

fn parse_input() -> Vec<Monkey> {
    let input = include_str!("../input.txt");
    let mut monkeys: Vec<Monkey> = vec![];
    let lines = input
        .split("\n\n")
        .map(|input| input.lines().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for ele in lines.iter() {
        let id = ele[0]
            .split("Monkey ")
            .skip(1)
            .map(|id| id.replace(':', "").parse::<usize>().unwrap())
            .next()
            .unwrap();

        let starting_items = ele[1]
            .split(": ")
            .skip(1)
            .flat_map(|item| {
                item.split(',')
                    .map(|v| v.trim().parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<VecDeque<_>>();
        // println!("Items: {starting_items:?}");

        let operation = Some(get_operation(
            &ele[2]
                .split("= ")
                .skip(1)
                .flat_map(|operation| operation.split_whitespace().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        ));
        // println!("operation: {operation:?}");

        let divisible = ele[3]
            .split("by ")
            .skip(1)
            .map(|num| num.parse::<usize>().unwrap())
            .next()
            .unwrap();
        // println!("divisible: {divisible:?}");

        let monkey_test = ele[4]
            .split("monkey ")
            .skip(1)
            .map(|num| num.parse::<usize>().unwrap())
            .zip(
                ele[5]
                    .split("monkey ")
                    .skip(1)
                    .map(|num| num.parse::<usize>().unwrap()),
            )
            .next()
            .unwrap();
        // println!("{monkey_test:?}");

        monkeys.push(Monkey {
            id,
            starting_items,
            operation,
            divisible,
            monkey_test,
            times_inspected: 0,
        });
    }

    monkeys
}

static mut MONKEYS: Vec<Monkey> = vec![];

#[allow(dead_code)]
enum Part {
    One,
    Two,
}

fn answer(part: Part, prime_product: usize) -> usize {
    let rounds;
    let divided_by_three;

    match part {
        Part::One => {
            rounds = 20;
            divided_by_three = true;
        }
        Part::Two => {
            rounds = 10000;
            divided_by_three = false;
        }
    }

    unsafe {
        for _round in 0..rounds {
            MONKEYS.iter_mut().for_each(|monkey| {
                // println!("Monkey: {}", monkey.id);

                for _ in 0..monkey.starting_items.len() {
                    let item = monkey.starting_items.pop_front().unwrap();
                    // println!("  Monkey inspects an item with a worry level of {item}");

                    let mut worrylevel =
                        monkey.operation.as_ref().unwrap().get_worry_level(item) % prime_product;

                    if divided_by_three {
                        worrylevel /= 3;

                        // println!(
                        // "    Monkey gets bored with item. Worry level is divided by 3 to {worrylevel}"
                        // );
                    }

                    monkey.times_inspected += 1;

                    if worrylevel % monkey.divisible == 0 {
                        // println!(
                        //     "    Current worry level is divisible by {}",
                        //     monkey.divisible
                        // );
                        let to_monkey = monkey.monkey_test.0;
                        MONKEYS[to_monkey].starting_items.push_back(worrylevel);

                        // println!(
                        //     "    Item with worry level {worrylevel} is thrown to monkey {to_monkey}"
                        // );
                    } else {
                        // println!(
                        //     "    Current worry level is not divisible by {}",
                        //     monkey.divisible
                        // );
                        let to_monkey = monkey.monkey_test.1;
                        MONKEYS[to_monkey].starting_items.push_back(worrylevel);
                        // println!(
                        //     "    Item with worry level {worrylevel} is thrown to monkey {to_monkey}"
                        // );
                    }
                }
                // println!();
            });
            // println!(
            //     "After round {_round}, the monkeys are holding items with these worry levels:"
            // );
            // MONKEYS.iter().for_each(|monkey| {
            //     println!("Monkey {}: {:?}", monkey.id, monkey.times_inspected);
            // });
        }
        let mut monkey_business: Vec<usize> = vec![];
        MONKEYS.iter().for_each(|monkey| {
            monkey_business.push(monkey.times_inspected);
            // println!(
            //     "Monkey {} inspected items {} times",
            //     monkey.id, monkey.times_inspected
            // );
        });
        monkey_business.sort_by(|a, b| b.cmp(a));
        monkey_business.iter().take(2).product()
    }
}

fn main() {
    unsafe {
        MONKEYS = parse_input();
        let monkeys = MONKEYS.clone();
        let prime_product = MONKEYS.iter().map(|m| m.divisible).product::<usize>();
        // MONKEYS.iter().for_each(|b| {
        //     println!("{b}");
        // });

        {
            let part_one = answer(Part::One, prime_product);
            println!("Answer part one: {part_one}");
        }
        {
            MONKEYS = monkeys;
            let part_two = answer(Part::Two, prime_product);
            println!("Answer part two: {part_two}");
        }
    }
}

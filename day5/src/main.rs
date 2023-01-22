use std::io::Result;

const STACK_SIZE: usize = 9;

fn get_stack_crates(input: &mut Vec<u8>) -> ([Vec<u8>; STACK_SIZE], Vec<usize>) {
    let mut stack_list: [Vec<u8>; STACK_SIZE] = Default::default();
    let (stack, crates) = input.split_at(input.windows(2).position(|w| w == b"\n\n").unwrap() + 2);
    let stack =
        &stack[..stack.len() - stack.windows(2).rev().position(|w| w == b" 1").unwrap() - 1];

    stack.split(|&c| c == b'\n').rev().for_each(|n| {
        n.iter().skip(1).step_by(4).enumerate().for_each(|(i, c)| {
            if *c != b' ' {
                stack_list[i].push(*c);
            }
        });
    });

    // Split crates into a Vec<&str>
    let crates = crates
        .split(|&c| c == b'\n')
        .map(|n| n.split(|&c| c == b' ').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // filter out "move" , "from" and "to" from crates
    let crates = crates
        .iter()
        .flat_map(|n| n.iter().skip(1).step_by(2))
        .map(|n| unsafe { std::str::from_utf8_unchecked(*n).parse::<usize>().unwrap() })
        .collect::<Vec<_>>();

    (stack_list, crates)
}

fn move_crates_in_stack(moves: &([Vec<u8>; STACK_SIZE], Vec<usize>), cratemover: usize) {
    // clone it why not
    let mut moves = moves.clone();
    // Iter over the moves 3 at time [size, from, to]
    moves.1.chunks(3).for_each(|n| {
        let size = n[0];
        // let from = n[1] - 1;
        // let to = n[2] - 1;
        // println!("{}, {}, {}", n[0], n[1], n[2]);

        // moves.0.iter().for_each(|n| {
        //     println!("{:?}", { n.iter().map(|c| *c as char).collect::<String>() });
        // });

        match cratemover {
            9000 => {
                for _ in 0..size {
                    if let Some(from_stack) = moves.0[n[1] - 1].pop() {
                        moves.0[n[2] - 1].push(from_stack);
                    }
                }
            }
            9001 => {
                if size == 1 {
                    if let Some(from_stack) = moves.0[n[1] - 1].pop() {
                        moves.0[n[2] - 1].push(from_stack);
                    }
                } else {
                    moves.0[n[2] - 1]
                        .extend(moves.0[n[1] - 1][moves.0[n[1] - 1].len() - size..].to_vec());
                    moves.0[n[1] - 1].truncate(moves.0[n[1] - 1].len() - size);
                }
            }
            _ => {
                println!("cratemover is invalid! :(");
            }
        }
    });
    print_answer(&moves.0);
}

fn print_answer(moves: &[Vec<u8>; STACK_SIZE]) {
    moves
        .iter()
        .for_each(|x| print!("{}", *x.iter().last().unwrap() as char));
    println!();
}

fn main() -> Result<()> {
    let mut input = include_bytes!("../input.txt").to_vec();
    let pairs = get_stack_crates(&mut input);

    // print crates
    // pairs.0.iter().for_each(|n| {
    //     println!("{:?}", { n.iter().map(|c| *c as char).collect::<String>() });
    // });

    move_crates_in_stack(&pairs, 9000);
    move_crates_in_stack(&pairs, 9001);

    Ok(())
}

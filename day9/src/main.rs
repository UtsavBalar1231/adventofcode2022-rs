#![allow(clippy::needless_return)]
const GRID_SIZE: usize = 2000;
static mut GRID: Vec<usize> = Vec::new();

fn parse_moves(input: &str) -> Vec<Vec<&str>> {
    let mut moves = input
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Pop last element if its empty
    if moves.last().unwrap().is_empty() {
        moves.pop();
    }

    moves
}

#[allow(dead_code)]
fn print_move(head: &(isize, isize), tail: &(isize, isize)) {
    for j in 0..GRID_SIZE as isize {
        for i in 0..GRID_SIZE as isize {
            if head.0 == j && head.1 == i && tail.0 == j && tail.1 == i {
                print!("H");
                continue;
            }
            if head.0 == j && head.1 == i {
                print!("H");
                continue;
            }
            if tail.0 == j && tail.1 == i {
                print!("T");
                continue;
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn move_rope(
    _move: (&str, isize),
    head: &mut (isize, isize),
    tail: &mut (isize, isize),
    size: isize,
) {
    for _ in 0..size {
        match _move.0 {
            "U" => {
                head.1 -= 1;
            }
            "D" => {
                head.1 += 1;
            }
            "L" => {
                head.0 -= 1;
            }
            "R" => {
                head.0 += 1;
            }
            _ => {}
        }

        for _ in 0.._move.1 {
            if (head.0 - tail.0).abs() > 1 {
                if _move.0.eq("L") {
                    tail.0 -= 1;
                } else {
                    tail.0 += 1;
                }
                tail.1 = head.1;
            }
            if (head.1 - tail.1).abs() > 1 {
                if _move.0.eq("U") {
                    tail.1 -= 1;
                } else {
                    tail.1 += 1;
                }
                tail.0 = head.0;
            }

            unsafe {
                GRID[tail.1 as usize * GRID_SIZE + tail.0 as usize] += 1;
            }
        }
    }
}

fn move_rope_2(_move: (&str, isize), rope: &mut Vec<(isize, isize)>, size: isize) {
    for _ in 0..size as usize {
        match _move.0 {
            "U" => {
                rope[0].1 -= 1;
            }
            "D" => {
                rope[0].1 += 1;
            }
            "L" => {
                rope[0].0 -= 1;
            }
            "R" => {
                rope[0].0 += 1;
            }
            _ => {}
        }

        for i in 0..rope.len() - 1 {
            let move_x = (rope[i].0 - rope[i + 1].0).abs();
            let move_y = (rope[i].1 - rope[i + 1].1).abs();
            let move_xy = move_x + move_y;

            if move_xy > 2 {
                if rope[i + 1].0 > rope[i].0 {
                    rope[i + 1].0 -= 1;
                } else {
                    rope[i + 1].0 += 1;
                }
                if rope[i + 1].1 > rope[i].1 {
                    rope[i + 1].1 -= 1;
                } else {
                    rope[i + 1].1 += 1;
                }
            } else if move_x > 1 {
                if rope[i + 1].0 > rope[i].0 {
                    rope[i + 1].0 -= 1;
                } else {
                    rope[i + 1].0 += 1;
                }
            } else if move_y > 1 {
                if rope[i + 1].1 > rope[i].1 {
                    rope[i + 1].1 -= 1;
                } else {
                    rope[i + 1].1 += 1;
                }
            }
        }

        unsafe {
            GRID[rope[9].1 as usize * GRID_SIZE + rope[9].0 as usize] += 1;
        }
        // print_grid();
    }
}

#[allow(dead_code)]
fn print_grid() {
    (0..GRID_SIZE).for_each(|x| {
        (0..GRID_SIZE).for_each(|y| unsafe {
            if GRID[x * GRID_SIZE + y] > 0 {
                print!(".");
            } else {
                print!("{}", GRID[x * GRID_SIZE + y]);
            }
        });
        println!()
    });
    println!();
}

fn start_movement(
    moves_list: &Vec<(&str, isize)>,
    head: &mut (isize, isize),
    tail: &mut (isize, isize),
) {
    for &_move in moves_list {
        match _move.0 {
            "R" => move_rope(_move, head, tail, _move.1),
            "L" => move_rope(_move, head, tail, _move.1),
            "U" => move_rope(_move, head, tail, _move.1),
            "D" => move_rope(_move, head, tail, _move.1),
            _ => {}
        }
    }
}

fn start_movement_2(moves_list: &Vec<(&str, isize)>, rope: &mut Vec<(isize, isize)>) {
    for &_move in moves_list {
        match _move.0 {
            "R" => move_rope_2(_move, rope, _move.1),
            "L" => move_rope_2(_move, rope, _move.1),
            "U" => move_rope_2(_move, rope, _move.1),
            "D" => move_rope_2(_move, rope, _move.1),
            _ => {}
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let moves = parse_moves(&input[..input.len() - 1]);
    unsafe {
        GRID.resize(GRID_SIZE * GRID_SIZE, 0);
    }

    let moves_list = moves
        .iter()
        .map(|m| {
            match m[..] {
                ["R", size] => return ("R", size.parse::<isize>().unwrap()),
                ["L", size] => return ("L", size.parse::<isize>().unwrap()),
                ["U", size] => return ("U", size.parse::<isize>().unwrap()),
                ["D", size] => return ("D", size.parse::<isize>().unwrap()),
                _ => return ("", 0),
            };
        })
        .collect::<Vec<_>>();
    // println!("{moves_list:?}");
    {
        let mut head: (isize, isize) = ((GRID_SIZE / 2) as isize, (GRID_SIZE / 2) as isize);
        let mut tail: (isize, isize) = ((GRID_SIZE / 2) as isize, (GRID_SIZE / 2) as isize);

        start_movement(&moves_list, &mut head, &mut tail);
        unsafe {
            println!("{}", GRID.iter().filter(|v| **v > 0).count());
        }
    }

    {
        unsafe {
            GRID.clear();
            GRID.resize(GRID_SIZE * GRID_SIZE, 0);
        }
        let mut rope: Vec<(isize, isize)> = Vec::from_iter(
            (0..=9)
                .map(|_| ((GRID_SIZE / 2) as isize, (GRID_SIZE / 2) as isize))
                .collect::<Vec<_>>(),
        );

        start_movement_2(&moves_list, &mut rope);
        unsafe {
            println!("{}", GRID.iter().filter(|v| **v > 0).count());
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let instructions = parse_input(input);

    // println!("{instructions:?}");

    part_one(&instructions);
    part_two(&instructions);
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|v| v.split_whitespace().collect())
        .collect()
}

enum Instruction {
    Add(isize),
    Noop,
}

fn parse_instruction(instruction: &[&str]) -> Option<Instruction> {
    match instruction[0] {
        "addx" => Some(Instruction::Add(instruction[1].parse::<isize>().unwrap())),
        "noop" => Some(Instruction::Noop),
        _ => None,
    }
}

impl Instruction {
    fn get_cycles(&self) -> isize {
        match self {
            Instruction::Noop => 1,
            Instruction::Add(_) => 2,
        }
    }
}

fn part_two(instructions: &Vec<Vec<&str>>) {
    let mut cycles = 0;
    let mut reg_x = 1;
    let mut crt: String = String::from("");

    for ins in instructions {
        let instruction = parse_instruction(ins).unwrap();

        // regx contains some instruction
        let sprite = reg_x - 1..=reg_x + 1;
        for x in 0..instruction.get_cycles() {
            let pixel_pos = (cycles + x) % 40;
            // println!(
            //     "cycle: {cycles} x_reg pos: {reg_x}, pixel pos: {pixel_pos}, spite: {sprite:?}"
            // );
            if sprite.contains(&pixel_pos) {
                crt.push('#');
            } else {
                crt.push('.');
            }
        }

        match instruction {
            Instruction::Add(val) => {
                cycles += instruction.get_cycles();
                reg_x += val;
            }

            Instruction::Noop => {
                cycles += instruction.get_cycles();
            }
        }
    }

    crt.chars().enumerate().for_each(|(i, v)| {
        if (i + 1) % 40 == 0 {
            println!();
        } else {
            print!("{v}");
        }
    });
    println!();
}

fn part_one(instructions: &Vec<Vec<&str>>) {
    let mut cycles = 1;
    let mut reg_x = 1;
    let mut signal_strength = Vec::new();
    for ins in instructions {
        let instruction = parse_instruction(ins).unwrap();
        // println!("{instruction:?} regx on {cycles}: {reg_x:?}");
        match instruction {
            Instruction::Add(value) => {
                reg_x += value;
                cycles += instruction.get_cycles();
            }
            Instruction::Noop => {
                cycles += instruction.get_cycles();
            }
        }

        if cycles == 20 || (cycles - 20) % 40 == 0 {
            // println!("regx on {cycles}: {reg_x:?}");

            signal_strength.push(reg_x * cycles);
        }
    }

    // println!("{reg_x}, {cycles}, {signal_strength:?}");
    println!("{}", signal_strength.iter().sum::<isize>());
}

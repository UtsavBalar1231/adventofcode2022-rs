fn main() {
    let input = include_bytes!("../input.txt").to_vec();
    solution(input.clone(), 4);
    solution(input, 14);
}

fn solution(input: Vec<u8>, size: usize) {
    input.split(|x| *x == b'\n').into_iter().for_each(|buffer| {
        if buffer.len() > 0 {
            let pos = buffer
                .windows(size)
                .position(|x| {
                    let mut temp: bool = true;
                    for i in 0..size {
                        for j in i + 1..size {
                            temp = temp && (x[i] != x[j]);
                        }
                    }
                    temp
                })
                .unwrap();
            print!("{:?} ", pos + size);
        }
    });
    println!();
}

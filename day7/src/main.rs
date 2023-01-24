use std::{collections::HashMap, path::PathBuf};

fn parse_files_list(input: &str) -> HashMap<PathBuf, usize> {
    // store the directory entries (filename, filesize)
    let mut files_sizes = HashMap::new();

    // keep the track of file in and file out (cd somedir and cd ..)
    let mut files_traverse = vec![];

    // iterate over each line in input
    for line in input.lines() {
        // exclude line with ls and dir prefixe
        if !line.starts_with("$ ls") && !line.starts_with("dir") {
            // else collect line as cmds seperated by space
            let cmds = line.split_whitespace().collect::<Vec<_>>();

            // three possible matches for the cmds
            match cmds[..] {
                // traverse file(dir) up
                ["$", "cd", ".."] => {
                    files_traverse.pop();
                }
                // traverse file(dir) down
                ["$", "cd", dir_name] => {
                    files_traverse.push(dir_name);
                }
                // iterate over the file(dir) traverse list (example: a/e)
                [size, _file_name] => {
                    if let Ok(size) = size.parse::<usize>() {
                        for i in 0..files_traverse.len() {
                            let path = PathBuf::from_iter(&files_traverse[..i + 1]);
                            // add the path(directory) as well as its constituient files size
                            *files_sizes.entry(path).or_insert(0) += size;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    files_sizes
}

fn part_two(file_sizes: &HashMap<PathBuf, usize>) -> usize {
    let used_space = *file_sizes.get(&PathBuf::from(r"/")).unwrap();
    let total_space_available = 70000000;
    let required_space = 30000000;
    let unused_space = total_space_available - used_space;

    let mut dir_to_remove_size = used_space;

    file_sizes.iter().for_each(|(_k, v)| {
        // check if the value is big enough to accomodate update
        // and smaller then the previous dir size (as we require the smallest dir for update)
        if *v >= (required_space - unused_space) && *v <= dir_to_remove_size {
            // println!(
            //     "used_space: {}, unused_space: {}, need: {}, dir_to_remove: {}",
            //     used_space,
            //     unused_space,
            //     (required_space - unused_space),
            //     dir_to_remove
            // );
            dir_to_remove_size = *v;
        }
    });

    dir_to_remove_size
}

fn part_one(file_sizes: &HashMap<PathBuf, usize>) -> usize {
    file_sizes.values().filter(|s| **s <= 100000).sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let file_sizes = parse_files_list(&input);

    let ans_part_one = part_one(&file_sizes);
    println!("{}", ans_part_one);

    let ans_part_two = part_two(&file_sizes);
    println!("{}", ans_part_two);
}

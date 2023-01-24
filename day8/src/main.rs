fn parse(input: &str) -> Vec<Vec<&str>> {
    let mut trees = input.lines().collect::<Vec<&str>>();

    let trees = trees
        .iter_mut()
        .map(|t| t.splitn(t.len() + 1, "").skip(1).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    trees
}

fn count_visible_trees_part_one(trees: &[Vec<&str>]) -> usize {
    // perimeter of trees patch - 4 (corner trees)
    let mut count = (2 * (trees.len() + trees.iter().last().unwrap().len())) - 4;
    trees.iter().enumerate().for_each(|(i, tree)| {
        // skip first and last rows of trees (to be used by inner trees)
        if i != 0 && i != trees.len() - 1 {
            tree.iter().enumerate().for_each(|(j, t)| {
                // skip first and last tree of tree column (to be used by inner trees column)
                if j != 0 && j != tree.len() - 1 {
                    // lets make the logic a bit sane
                    let bottom = trees.len() - i - 1;
                    let right = tree.len() - j - 1;

                    // i is number of trees available on top of current tree
                    // and j is number of trees available on the left side of current tree

                    // check if all the left most or all the rightmost trees are smaller than the current tree
                    if tree[0..j].iter().max().unwrap() < t
                        || tree[j + 1..right + j + 1].iter().max().unwrap() < t
                    {
                        count += 1;
                    // if tree is not visible from left or right then only try top or bottom
                    } else {
                        // first check if the tree is visible from the top
                        let mut check_top = vec![];
                        (0..i).for_each(|x| check_top.push(trees[i - x - 1][j]));
                        if check_top.iter().max().unwrap() < t {
                            count += 1;
                        // if not visible from top check if visible from the bottom
                        } else {
                            let mut check_bottom = vec![];

                            (0..bottom).for_each(|x| check_bottom.push(trees[i + 1 + x][j]));
                            if check_bottom.iter().max().unwrap() < t {
                                count += 1;
                            }
                        }
                    }
                }
            });
        }
    });

    count
}

fn count_visible_trees_part_two(trees: &[Vec<&str>]) -> usize {
    let mut max_scenic_score = 0;

    trees.iter().enumerate().for_each(|(i, tree)| {
        // skip first and last rows of trees (to be used by inner trees)
        if i != 0 && i != trees.len() - 1 {
            tree.iter().enumerate().for_each(|(j, t)| {
                // skip first and last tree of tree column (to be used by inner trees column)
                if j != 0 && j != tree.len() - 1 {
                    let mut left_cnt = 0;
                    let mut right_cnt = 0;
                    let mut top_cnt = 0;
                    let mut bottom_cnt = 0;

                    // lets make the logic a bit sane
                    let bottom = trees.len() - i - 1;
                    let right = tree.len() - j - 1;

                    // i is number of trees available on top of current tree
                    // and j is number of trees available on the left side of current tree

                    // check in reverse order of available left trees to find the tree
                    // that is more larger than the current tree
                    for &x in tree[0..j].iter().rev() {
                        left_cnt += 1;
                        if x >= t {
                            break;
                        }
                    }

                    // We do not need to reverse for right side available trees
                    // Iterate over available right trees and find number of trees that are visible
                    for x in &tree[j + 1..right + j + 1] {
                        right_cnt += 1;
                        if x >= t {
                            break;
                        }
                    }

                    // Same logic as past one to find top avaiable trees
                    let mut check_top = vec![];
                    (0..i).for_each(|x| check_top.push(trees[i - x - 1][j]));
                    // as this is in reverse order we will always get the proper number of visible top trees
                    for x in check_top {
                        top_cnt += 1;
                        if x >= t {
                            break;
                        }
                    }

                    // Same logic as past one to find bottom avaiable trees
                    let mut check_bottom = vec![];
                    (0..bottom).for_each(|x| check_bottom.push(trees[i + 1 + x][j]));
                    for x in check_bottom {
                        bottom_cnt += 1;
                        if x >= t {
                            break;
                        }
                    }

                    // println!(
                    //     "up: {top_cnt} left: {left_cnt} right: {right_cnt} down: {bottom_cnt}"
                    // );

                    let score = top_cnt * left_cnt * right_cnt * bottom_cnt;
                    if max_scenic_score < score {
                        max_scenic_score = score;
                    }
                }
            });
        }
    });

    max_scenic_score
}

fn main() {
    let input = include_str!("../input.txt");
    let parsed_trees = parse(input);

    let visible_trees = count_visible_trees_part_one(&parsed_trees);
    println!("visible_trees: {visible_trees}");

    let scenic_score = count_visible_trees_part_two(&parsed_trees);
    println!("scenic score: {scenic_score}");
}

use std::fs::File;
use std::io::read_to_string;
use std::io::Result;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum GameMove {
    Rock,
    Paper,
    Scissor,
    Nil,
}

#[derive(Debug)]
enum Game {
    Win,
    Lose,
    Draw,
}

#[derive(Debug, Clone, Copy)]
struct Player {
    game_move: GameMove,
}

impl Player {
    fn new() -> Self {
        Self {
            game_move: GameMove::Nil,
        }
    }

    fn player_move(&mut self, v: &str) {
        match v {
            "A" | "X" => self.game_move = GameMove::Rock,
            "B" | "Y" => self.game_move = GameMove::Paper,
            "C" | "Z" => self.game_move = GameMove::Scissor,
            _ => self.game_move = GameMove::Nil,
        }
    }

    fn check_move(&self, p2: &Player) -> Option<Game> {
        if self.game_move == GameMove::Nil || p2.game_move == GameMove::Nil {
            return None;
        }

        if self.game_move == p2.game_move {
            return Some(Game::Draw);
        } else if self.game_move == GameMove::Rock && p2.game_move == GameMove::Scissor {
            return Some(Game::Win);
        } else if self.game_move == GameMove::Rock && p2.game_move == GameMove::Paper {
            return Some(Game::Lose);
        } else if self.game_move == GameMove::Paper && p2.game_move == GameMove::Scissor {
            return Some(Game::Lose);
        } else if self.game_move == GameMove::Paper && p2.game_move == GameMove::Rock {
            return Some(Game::Win);
        } else if self.game_move == GameMove::Scissor && p2.game_move == GameMove::Rock {
            return Some(Game::Lose);
        } else if self.game_move == GameMove::Scissor && p2.game_move == GameMove::Paper {
            return Some(Game::Win);
        } else {
            return None;
        }
    }
}

fn get_moves_from_file() -> Result<String> {
    let mut file = File::open("input.txt")?;
    let content = read_to_string(&mut file)?.trim().to_string();

    Ok(content)
}

fn get_vec_moves_list(list: &mut String) -> Vec<Vec<&str>> {
    let moves = list
        .split("\n")
        .collect::<Vec<_>>()
        .iter_mut()
        .map(|x| x.split(" ").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    moves
}

fn calculate_score(game: &(Game, GameMove)) -> usize {
    let mut score = 0;
    match game {
        (Game::Win, GameMove::Rock) => score += 6 + 1,
        (Game::Win, GameMove::Paper) => score += 6 + 2,
        (Game::Win, GameMove::Scissor) => score += 6 + 3,
        (Game::Draw, GameMove::Rock) => score += 3 + 1,
        (Game::Draw, GameMove::Paper) => score += 3 + 2,
        (Game::Draw, GameMove::Scissor) => score += 3 + 3,
        (Game::Lose, GameMove::Rock) => score += 1,
        (Game::Lose, GameMove::Paper) => score += 2,
        (Game::Lose, GameMove::Scissor) => score += 3,
        _ => score += 0,
    };
    score
}

fn calculate_total_score(games: Vec<(Game, GameMove)>) -> usize {
    let mut score = 0;
    for game in games {
        match game {
            (Game::Win, GameMove::Rock) => score += 6 + 1,
            (Game::Win, GameMove::Paper) => score += 6 + 2,
            (Game::Win, GameMove::Scissor) => score += 6 + 3,
            (Game::Draw, GameMove::Rock) => score += 3 + 1,
            (Game::Draw, GameMove::Paper) => score += 3 + 2,
            (Game::Draw, GameMove::Scissor) => score += 3 + 3,
            (Game::Lose, GameMove::Rock) => score += 1,
            (Game::Lose, GameMove::Paper) => score += 2,
            (Game::Lose, GameMove::Scissor) => score += 3,
            _ => score += 0,
        }
    }
    score
}

fn main() -> Result<()> {
    let mut moves_list = get_moves_from_file()?;
    let moves_list = get_vec_moves_list(&mut moves_list);
    let mut games: Vec<(Game, GameMove)> = vec![];

    for game in moves_list {
        println!("{game:?}");
        let mut p1 = Player::new();
        let mut p2 = Player::new();
        p1.player_move(game[1]);
        p2.player_move(game[0]);

        println!("p1: {:?}, p2: {:?}", p1.game_move, p2.game_move);

        println!("game: {:?}", p1.check_move(&p2));

        println!(
            "score: {}",
            calculate_score(&(p1.check_move(&p2).unwrap(), p1.game_move))
        );

        games.push((p1.check_move(&p2).unwrap(), p1.game_move));
    }

    let score = calculate_total_score(games);

    println!("score: {score}");
    Ok(())
}

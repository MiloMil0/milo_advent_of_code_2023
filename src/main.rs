use std::fs::File;
use std::io::prelude::*;

const RED_MAX: i32 = 12;
const GREEN_MAX: i32 = 13;
const BLUE_MAX: i32 = 14;

struct TotalGames {
    games: Vec<SingleGame>,
}

impl TotalGames {
    fn new(games: Vec<SingleGame>) -> Self {
        TotalGames { games }
    }
}

#[derive(Debug)]
struct SingleGame {
    turns: Vec<Turn>,
}

impl SingleGame {
    fn new(turns: Vec<Turn>) -> Self {
        SingleGame { turns }
    }
}

#[derive(Debug)]
struct Turn {
    red_block: i32,
    green_block: i32,
    blue_block: i32,
}

impl Turn {
    fn new(r: i32, g: i32, b: i32) -> Self {
        Turn {
            red_block: r,
            green_block: g,
            blue_block: b,
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut content = File::open("assets/puzzle_input.txt").expect("couldn't find file");
    let mut buffer = String::new();

    content.read_to_string(&mut buffer)?;

    let strings = buffer.trim().split("\n").collect::<Vec<_>>();

    let split_game = strings
        .iter()
        .map(|game| game.split(':').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut games_input = Vec::new();

    for (i, _) in split_game.iter().enumerate() {
        games_input.push(split_game[i][1]);
    }

    let total_games: TotalGames = {
        let games: Vec<SingleGame> = games_input
            .iter()
            .map(|game_str| {
                let turns: Vec<Turn> = game_str
                    .trim()
                    .split("; ")
                    .map(|turn_str| {
                        let counts: Vec<&str> = turn_str.split(", ").collect();
                        let mut green = 0;
                        let mut blue = 0;
                        let mut red = 0;

                        for count_str in counts {
                            let parts: Vec<&str> = count_str.split(' ').collect();
                            let num = parts[0].parse::<i32>().unwrap();
                            match parts[1] {
                                "red" => red = num,
                                "green" => green = num,
                                "blue" => blue = num,
                                _ => {}
                            }
                        }

                        Turn::new(red, green, blue)
                    })
                    .collect();

                SingleGame::new(turns)
            })
            .collect();

        TotalGames::new(games)
    };
    let mut valid_games = 0;
    let mut block_power = 0;

    for (i, game) in total_games.games.iter().enumerate() {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        let mut valid = true;

        for turn in game.turns.iter() {
            if min_red <= turn.red_block {
                min_red = turn.red_block;
            }
            if min_green <= turn.green_block {
                min_green = turn.green_block;
            }
            if min_blue <= turn.blue_block {
                min_blue = turn.blue_block;
            }
            if turn.red_block > RED_MAX
                || turn.green_block > GREEN_MAX
                || turn.blue_block > BLUE_MAX
            {
                valid = false;
            }
        }
        block_power += min_red * min_green * min_blue;
        if valid {
            valid_games += i + 1;
        };
    }

    println!("block power = {block_power}");
    println!("number of valid games = {valid_games}");

    Ok(())
}

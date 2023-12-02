struct Game {
    id: u32,
    red_max: u32,
    green_max: u32,
    blue_max: u32,
}

pub fn day2_part1(input_data: &str) -> u64 {
    let input_vec: Vec<&str> = input_data.split('\n').collect();
    let mut id_sum: u64 = 0;
    let max_cubes = (12, 13, 14);
    let mut games: Vec<Game> = Vec::new();
    for input_line in input_vec {
        let game_id_str = input_line.split(':').next().unwrap();
        let game_str = input_line.split(':').last().unwrap().trim();
        let gameid: u32 = match game_id_str.split(' ').nth(1).unwrap().trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Keine GameID gefunden."),
        };
        let mut game = Game {
            id: gameid,
            red_max: 0,
            green_max: 0,
            blue_max: 0,
        };
        for roll in game_str.split(';') {
            for entry in roll.trim().split(',') {
                let number_entries :Vec<&str>= entry.trim().split(' ').collect();
                let roll = number_entries.first().unwrap().parse::<u32>().expect("Zahl erwartet");
                let color = *number_entries.last().unwrap();
                match color {
                    "red" => {
                        if roll > game.red_max {
                            game.red_max = roll
                        }
                    }
                    "green" => {
                        if roll > game.green_max {
                            game.green_max = roll
                        }
                    }
                    "blue" => {
                        if roll > game.blue_max {
                            game.blue_max = roll
                        }
                    }
                    _ => panic!("Keine Farbe gefunden"),
                }
            }
        }
        games.push(game);
    }
    let filtered_ids = games.iter().filter(|x| {
        x.red_max <= max_cubes.0 && x.green_max <= max_cubes.1 && x.blue_max <= max_cubes.2
    });
    for games in filtered_ids {
        id_sum += games.id as u64;
    }
    id_sum
}

pub fn day2_part2(input_data: &str) -> u64 {
    let input_vec: Vec<&str> = input_data.split('\n').collect();
    let mut id_sum: u64 = 0;
    let mut games: Vec<Game> = Vec::new();
    for input_line in input_vec {
        let game_id_str = input_line.split(':').next().unwrap();
        let game_str = input_line.split(':').last().unwrap().trim();
        let gameid: u32 = match game_id_str.split(' ').nth(1).unwrap().trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Keine GameID gefunden."),
        };
        let mut game = Game {
            id: gameid,
            red_max: 0,
            green_max: 0,
            blue_max: 0,
        };
        for roll in game_str.split(';') {
            for entry in roll.trim().split(',') {
                let number_entries :Vec<&str>= entry.trim().split(' ').collect();
                let roll = number_entries.first().unwrap().parse::<u32>().expect("Zahl erwartet");
                let color = *number_entries.last().unwrap();
                match color {
                    "red" => {
                        if roll > game.red_max {
                            game.red_max = roll
                        }
                    }
                    "green" => {
                        if roll > game.green_max {
                            game.green_max = roll
                        }
                    }
                    "blue" => {
                        if roll > game.blue_max {
                            game.blue_max = roll
                        }
                    }
                    _ => panic!("Keine Farbe gefunden"),
                }
            }
        }
        games.push(game);
    }
    for game in games {
        //println!("{}",games.id);
        id_sum += (game.blue_max*game.red_max*game.green_max) as u64;
    }
    id_sum
}

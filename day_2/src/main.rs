use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let mut bag: HashMap<&str, i32> = HashMap::from([("red",12), ("green", 13), ("blue", 14)]);

    let contents = fs::read_to_string("data.txt").expect("Error");
    let games = contents.split("\n").map(|it| it.trim()).collect::<Vec<&str>>();

    let mut result: i32 = 0;

    for game in games {
        let mut is_possible: bool = true;
        let mut game_id: i32 = 0;

        if let Some(captures) = Regex::new(r"\d\d?\d?:").unwrap().captures(&game) {
            if let Some(id) = captures.get(0) {
                game_id = id.as_str().replace(":", "").parse().unwrap();
            }
        }
        let draws: Vec<&str> = Regex::new(r"\d\d? \w*").unwrap().find_iter(&game).map(|m| m.as_str()).collect();

        for draw in draws {
            let split: Vec<&str> = draw.split_whitespace().collect();
            if split.len() == 2 {
                let num: i32 = split[0].parse().unwrap();
                let color: &str = split[1];
                let mut cubes = HashMap::new();
                cubes.insert(color, num);

                for (cube_color, cube_num) in &cubes {
                    if let Some(&bag_num) = bag.get(cube_color) {
                        if *cube_num > bag_num {
                            is_possible = false;
                        }
                    }
                }
            }
        }
        if is_possible {
            result += game_id;
        }
    }
    println!("{}", result);
}

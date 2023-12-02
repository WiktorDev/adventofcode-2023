use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("Error");
    let games = contents.split("\n").map(|it| it.trim()).collect::<Vec<&str>>();

    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;

    let mut counter = 0;
    for game in games {
        let formatted = game.replace("Game ", "");
        let data = formatted.split(":").map(|it| it.trim()).collect::<Vec<&str>>();
        if data.len() != 2 { continue }
        let game_id = data.first().unwrap().parse::<i32>().unwrap();
        let sets = data.get(1).unwrap().split(";").collect::<Vec<&str>>();
        let mut is_possible = true;
        for set in sets {
            let items = set.trim().split(", ").collect::<Vec<&str>>();
            for item in items {
                let d = item.split(" ").collect::<Vec<&str>>();
                let amount = d.first().unwrap().parse::<i32>().unwrap();
                let name = d.last().unwrap();

                match name {
                    &"red" => {
                        is_possible = amount < red_cubes;
                    }
                    &"green" => {
                        is_possible = amount < green_cubes;
                    }
                    &"blue" => {
                        is_possible = amount < blue_cubes;
                    }
                    _ => {
                        break
                    }
                }
            }
            if !is_possible { break }
        }
        if is_possible {
            counter += game_id;
        }
    }
    println!("{}", counter);
}

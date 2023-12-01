use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("Error");
    let strings = contents.split("\n").map(|it| it.trim()).collect::<Vec<&str>>();
    println!("{:?}", calculate(strings))
}

fn calculate(strings: Vec<&str>) -> i32 {
    let mut to_calculate = Vec::new();

    for text in strings {
        let mut numbers = Vec::new();
        let num1;
        let num2;
        for c in text.chars() {
            if c.is_numeric() {
                numbers.push(c.to_digit(10))
            }
        }
        match numbers.len() {
            0 => { continue }
            1 => {
                num1 = numbers.get(0).unwrap().unwrap();
                num2 = num1;
            }
            2 => {
                num1 = numbers.get(0).unwrap().unwrap();
                num2 = numbers.get(1).unwrap().unwrap();
            }
            _ => {
                num1 = numbers.get(0).unwrap().unwrap();
                num2 = numbers.get(numbers.len() - 1).unwrap().unwrap();
            }
        }
        to_calculate.push(format!("{num1}{num2}").parse::<i32>().unwrap())
    }
    return to_calculate.iter().sum::<i32>();
}
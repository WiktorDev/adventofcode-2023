// use std::fs;
//
// fn main() {
//     let contents = fs::read_to_string("data.txt").expect("Error");
//     let strings = contents.split("\n").map(|it| it.trim()).collect::<Vec<&str>>();
//     println!("{:?}", calculate(strings))
// }
//
// fn calculate(strings: Vec<&str>) -> i32 {
//     let mut to_calculate = Vec::new();
//
//     for text in strings {
//         let output = replace_numbers(text);
//         let mut numbers = Vec::new();
//         let num1;
//         let num2;
//         for c in output.chars() {
//             if c.is_numeric() {
//                 numbers.push(c.to_digit(10))
//             }
//         }
//         match numbers.len() {
//             0 => { continue }
//             1 => {
//                 num1 = numbers.get(0).unwrap().unwrap();
//                 num2 = num1;
//             }
//             2 => {
//                 num1 = numbers.get(0).unwrap().unwrap();
//                 num2 = numbers.get(1).unwrap().unwrap();
//             }
//             _ => {
//                 num1 = numbers.get(0).unwrap().unwrap();
//                 num2 = numbers.get(numbers.len() - 1).unwrap().unwrap();
//             }
//         }
//         to_calculate.push(format!("{num1}{num2}").parse::<i32>().unwrap())
//     }
//     return to_calculate.iter().sum::<i32>();
// }
// fn replace_numbers(string: &str) -> String {
//     let number_names = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
//     let mut output = string.to_string();
//
//     let mut i = 0;
//     while i < output.len() {
//         let mut found = false;
//         for name in &number_names {
//             if output[i..].starts_with(name) {
//                 let index = number_names.iter().position(|x| x == name).unwrap().to_string();
//                 output = output.replacen(name, &index, 1);
//                 i += index.len();
//                 found = true;
//                 break;
//             }
//         }
//         if !found {
//             i += 1;
//         }
//     }
//     return output;
// }
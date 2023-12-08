use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Part {
    number: u64,
    column: i64,
    length: u64,
    row: i64,
    marked: bool,
}

#[derive(Copy, Clone)]
struct Symbol {
    symbol: char,
    column: i64,
    row: i64,
}

pub fn day3_part1(input_data: &str) -> u64 {
    let mut result = 0_u64;
    let input_vec: Vec<&str> = input_data.split('\n').collect();
    let mut parts: HashSet<Part> = HashSet::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    for (linecounter, line) in input_vec.into_iter().enumerate() {
        let mut number_str = String::from("");
        for (lettercounter, letter) in line.chars().enumerate() {
            if letter.is_ascii_digit() {
                // println!("Zahl gefunden:{}", letter);
                number_str.push(letter);
            } else if letter == '.' && !number_str.is_empty() {
                // println!("Punkt gefunden");
                parts.insert(Part {
                    number: number_str.trim().parse().unwrap(),
                    column: (lettercounter + 1 - number_str.len()) as i64,
                    length: number_str.len() as u64,
                    row: linecounter as i64,
                    marked: false,
                });
                number_str = String::from("");
            } else if letter.is_ascii()&& letter != '.' {
                // println!("Symbol gefunden:{}", letter);
                symbols.push(Symbol {
                    symbol: letter,
                    column: (lettercounter + 1) as i64,
                    row: linecounter as i64,
                });
                if !number_str.is_empty() {
                    parts.insert(Part {
                        number: number_str.trim().parse().unwrap(),
                        column: lettercounter as i64,
                        length: number_str.len() as u64,
                        row: linecounter as i64,
                        marked: false,
                    });
                    number_str = String::from("");
                }
            }
        }
    }
    for mut part in parts {
        for symbol in &symbols {
            if (symbol.column >= part.column - 1
                && symbol.column <= part.column + part.length as i64)
                && (symbol.row >= part.row - 1 && symbol.row <= part.row + 1_i64)
                && !part.marked
                && !(symbol.column == part.column && symbol.row == part.row)
            {
                println!(
                    "Teil {} bei {}|{} mit Symbol: {}|{}",
                    part.number, part.row, part.column, symbol.row, symbol.column
                );
                result += part.number;
                part.marked = true;
            }
        }
    }
    result
}

pub fn day3_part2(input_data: &str) -> u64 {
    0
}

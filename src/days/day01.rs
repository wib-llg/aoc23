use substring::Substring;


pub fn day1_part1(input_data: &str) -> u64 {
    let input_vec: Vec<&str> = input_data.split("\n").collect();
    let mut result = 0_u64;
    for input_line in input_vec {
        let mut number_str :String = input_line.chars().filter(|x| x.is_numeric()).collect();
        let number_count = number_str.len();
        
        let number:u64 = match number_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        result+= number;

    }
    result
}


pub fn day1_part2(input_data: &str) -> u64 {
    0
}
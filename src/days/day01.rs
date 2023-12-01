use substring::Substring;

pub fn day1_part1(input_data: &str) -> u64 {
    let input_vec: Vec<&str> = input_data.split('\n').collect();
    let mut result = 0_u64;
    for input_line in input_vec {
        let number_intput: String = input_line.chars().filter(|x| x.is_numeric()).collect();
        let number_str = number_intput.substring(0, 1).to_string()
            + number_intput.substring(number_intput.len() - 1, number_intput.len());
        let number: u64 = match number_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        result += number;
    }
    result
}

pub fn day1_part2(input_data: &str) -> u64 {
    let input_vec: Vec<&str> = input_data.split('\n').collect();
    let mut result = 0_u64;
    let valid_numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    for input_line in input_vec {
        let mut first_pos = input_line.len();
        let mut first_number: u64 = 0;
        let mut last_number: u64 = 0;
        let mut last_pos = 0;
        for vnumber in valid_numbers {
            first_number = match input_line.find(vnumber) {
                Some(_) => {
                    let v_first_pos = input_line.find(vnumber).unwrap();
                    if v_first_pos < first_pos {
                        first_pos = v_first_pos;
                        str_to_num(vnumber)
                    } else {
                        continue
                    }
                }
                None => continue,
            };            
        }
        for vnumber in valid_numbers{
            last_number = match input_line.rfind(vnumber) {
                Some(_) => {
                    let v_last_pos = input_line.rfind(vnumber).unwrap();
                    if v_last_pos > last_pos {
                        last_pos = v_last_pos;
                        str_to_num(vnumber)
                    } else {
                        continue
                    }
                }
                None => continue,
            };
        }
        if last_number==0 {last_number=first_number};
        result += 10 * first_number + last_number;
    }
    result
}

fn str_to_num(input: &str) -> u64 {
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => 0,
    }
}

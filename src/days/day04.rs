struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    users_numbers: Vec<u32>,
    score: u64,
}

pub fn day4_part1(input_data: &str) -> u64 {
    let input_vec: Vec<&str> = input_data.split('\n').collect();
    let mut score: u64 = 0;
    let mut cards: Vec<Card> = Vec::new();
    for (linecounter, input_line) in input_vec.iter().enumerate() {
        let winning_cards_str = input_line
            .split(':')
            .last()
            .unwrap()
            .trim()
            .split('|')
            .next()
            .unwrap()
            .trim()
            .replace("  ", " ");
        let user_cards_str = input_line
            .split(':')
            .last()
            .unwrap()
            .trim()
            .split('|')
            .last()
            .unwrap()
            .trim()
            .replace("  ", " ");
        let winning_numbers: Vec<u32> = winning_cards_str
            .split(' ')
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect();
        let users_numbers: Vec<u32> = user_cards_str
            .split(' ')
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect();
        let mut counter = 0_u64;
        for number in &winning_numbers {
            if users_numbers.contains(number) {
                counter += 1;
            }
        }
        cards.push(Card {
            id: linecounter as u32+1,
            winning_numbers,
            users_numbers,
            score: counter*2,
        });
    }

    for card in &cards {
        score += card.score;
    }
    score
}

pub fn day2_part2(input_data: &str) -> u64 {
    0
}

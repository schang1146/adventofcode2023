fn main() {
    let input = include_str!("../input").split("\n");
    let mut running_sum = 0;

    for data in input {
        let mut first_num: Option<u32> = None;
        let mut last_num: Option<u32> = None;
        // let mut word = "";

        for (i, character) in data.chars().enumerate() {
            if first_num.is_none() {
                if character.is_digit(10) {
                    first_num = character.to_digit(10);
                } else {
                    let substring = match data.char_indices().nth(i as usize) {
                        // If a position with the given index was found in the string, create a substring
                        Some((pos, _)) => (&data[pos..]).to_string(),
                        // Else, create an empty string
                        None => "".to_string(),
                    };
                    if substring.starts_with("one") {
                        first_num = Some(1)
                    }
                    if substring.starts_with("two") {
                        first_num = Some(2)
                    }
                    if substring.starts_with("three") {
                        first_num = Some(3)
                    }
                    if substring.starts_with("four") {
                        first_num = Some(4)
                    }
                    if substring.starts_with("five") {
                        first_num = Some(5)
                    }
                    if substring.starts_with("six") {
                        first_num = Some(6)
                    }
                    if substring.starts_with("seven") {
                        first_num = Some(7)
                    }
                    if substring.starts_with("eight") {
                        first_num = Some(8)
                    }
                    if substring.starts_with("nine") {
                        first_num = Some(9)
                    }
                }
            }

            if character.is_digit(10) {
                last_num = character.to_digit(10);
            } else {
                let substring = match data.char_indices().nth(i as usize) {
                    // If a position with the given index was found in the string, create a substring
                    Some((pos, _)) => (&data[pos..]).to_string(),
                    // Else, create an empty string
                    None => "".to_string(),
                };
                if substring.starts_with("one") {
                    last_num = Some(1)
                }
                if substring.starts_with("two") {
                    last_num = Some(2)
                }
                if substring.starts_with("three") {
                    last_num = Some(3)
                }
                if substring.starts_with("four") {
                    last_num = Some(4)
                }
                if substring.starts_with("five") {
                    last_num = Some(5)
                }
                if substring.starts_with("six") {
                    last_num = Some(6)
                }
                if substring.starts_with("seven") {
                    last_num = Some(7)
                }
                if substring.starts_with("eight") {
                    last_num = Some(8)
                }
                if substring.starts_with("nine") {
                    last_num = Some(9)
                }
            }
        }

        running_sum += first_num.unwrap() * 10;
        running_sum += last_num.unwrap();
    }

    println!("{running_sum}")
}

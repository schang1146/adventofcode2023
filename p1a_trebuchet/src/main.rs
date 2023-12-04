fn main() {
    let input = include_str!("../sample_input").split("\n");
    let mut running_sum = 0;

    for data in input {
        for character in data.chars() {
            if character.is_digit(10) {
                running_sum = running_sum + character.to_digit(10).unwrap() * 10;
                break;
            }
        }

        for character in data.chars().rev() {
            if character.is_digit(10) {
                running_sum = running_sum + character.to_digit(10).unwrap();
                break;
            }
        }
    }

    println!("{running_sum}")
}

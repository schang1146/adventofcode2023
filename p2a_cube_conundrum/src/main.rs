fn main() {
    let input = include_str!("../sample_input").split("\n");
    let mut running_sum = 0;

    const MAX_RED_CUBES: i32 = 12;
    const MAX_GRN_CUBES: i32 = 13;
    const MAX_BLU_CUBES: i32 = 14;

    for (idx, game) in input.into_iter().enumerate() {
        let mut is_game_possible = true;

        let rounds = game.split(": ").collect::<Vec<&str>>()[1].split("; ");
        for round in rounds {
            let marbles = round.split(", ").collect::<Vec<&str>>();
            for marble in marbles {
                let data = marble.split(" ").collect::<Vec<&str>>();
                let count = data[0].parse::<i32>().unwrap();
                let color = data[1];

                if color == "red" && count > MAX_RED_CUBES {
                    is_game_possible = false;
                }
                if color == "green" && count > MAX_GRN_CUBES {
                    is_game_possible = false;
                }
                if color == "blue" && count > MAX_BLU_CUBES {
                    is_game_possible = false;
                }

                if !is_game_possible {
                    break;
                }
            }

            if !is_game_possible {
                break;
            }
        }

        if is_game_possible {
            running_sum += idx + 1
        }
    }

    println!("{running_sum}")
}

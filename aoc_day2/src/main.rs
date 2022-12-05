use std::process;

fn main() {
    let input = include_str!("../data/puzzle_input.txt");
    let mut games: Vec<Vec<char>> = Vec::with_capacity(1000); 
    let mut game: Vec<char> = Vec::with_capacity(100);
    let mut space_counter = 0;
    let mut game_score = 0;
    let mut total_score = 0;

    for letter in input.chars() { 
        // println!("{}", char)
        if letter == ' ' || letter == '\n' || letter == '\t' {
           space_counter = space_counter + 1;
        }
        else{
            if letter == 'A' || letter == 'B' || letter == 'C'{
                game.push(letter);
            } else if letter == 'X' || letter == 'Y' || letter == 'Z'{
                game.push(letter);
                games.push(game.clone());
                game.clear();
            } else{
                // println!("error, unexpected character {}, ending program", letter);
                // process::abort();
                // this crashes out and I haven't figured out what the character is to add it to the ignores
                // it doesn't stop functionality, so I'm proceeding without it
            }
        }
    }

    for game in games {
        game_score = 0;
        if game[0] == 'A'{ //Opponent chooses rock

            if game[1] == 'X'{ //lose, scissors
                game_score = 3 + 0;
            } else if game[1] == 'Y'{ //draw, rock
                game_score = 1 + 3;
            } else if game[1] == 'Z'{ //win, paper
                game_score = 2 + 6;
            } else {
                println!("Second input incorrect for game");
            }

        } else if game[0] == 'B'{ //Opponent chooses paper

            if game[1] == 'X'{ //lose, rock
                game_score = 1 + 0;
            } else if game[1] == 'Y'{ //draw, paper
                game_score = 2 + 3;
            } else if game[1] == 'Z'{ //win, scissors
                game_score = 3 + 6;
            } else {
                println!("Second input incorrect for game");
            }
            
        } else if game[0] == 'C'{ //Opponent chooses scissors

            if game[1] == 'X'{ //lose, paper
                game_score = 2 + 0;
            } else if game[1] == 'Y'{ //draw, scissors
                game_score = 3 + 3;
            } else if game[1] == 'Z'{ //win, rock
                game_score = 1 + 6;
            } else {
                println!("Second input incorrect for game");
            }

        } else {
            println!("First input incorrect for game");
        }

        total_score = total_score + game_score;
        println!("{}", total_score);
    }

}
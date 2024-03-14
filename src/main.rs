use std::io;
use std::convert::TryFrom;

fn main () {
    let mut playing = true;

    while playing {

        let mut round_won = false;

        //Uncomment once you make the starting board blank
        //let mut board = [" "; 9];

        let mut board = [" ", "X", "O", "O", "X", " ", "X", "O", " "];

        while !round_won {            

            println!("{}", render_screen(board));

            //Fix function to return Ok() and Err() instead of an int or 10 as an error
            let move_index = get_player_move(board);

            if move_index < 10 {

                //Better error catching here
                let board_index = usize::try_from(move_index - 1).unwrap();

                let cell_value = board[board_index];

                if cell_value != " " {
                    println!("{} is not a valid square. ({} is there).", move_index, cell_value);
                }
                else {

                    //Place symbol in array
                }
            }
            else {
                println!("Invalid input. Please enter a number from 1 to 9!");
            }
        }

        //Exit code
        let mut exit_answer = String::new();        

        while exit_answer.as_str().trim() != "y" && exit_answer.as_str().trim() != "n" {
            println!("Would you like to exit? (y/n): ");

            io::stdin()
                .read_line(&mut exit_answer)
                .expect("No input detected!");

            match exit_answer.as_str().trim() {
                "y" => { println!("Goodbye!"); playing = false; },
                "n" => { println!("Another round!"); },
                _ => { println!("Please enter a valid answer!") },
            }
        }
    }
    
}

fn render_screen(board: [&str;9]) -> String {

    let mut screen = String::new();

    for x in 0..3{
        screen += &String::from(format!("{} | {} | {}\n", board[x*3], board[(x*3) + 1], board[(x*3) + 2]));

        if x < 2 { screen += &String::from("---------\n");}
    }

    screen
}

fn get_bot_move(board: [&str;9]) -> u32 {
    let mut bot_move = 0;

    bot_move
}

fn get_player_move(board: [&str;9]) -> u32 {
    let mut player_move = String::new();
    println!("Enter your next square (1-9): ");

    io::stdin()
        .read_line(&mut player_move)
        .expect("No input detected!");

    //Returns 10 since that's out of range for our logic.
    player_move.trim().parse::<u32>() {
        Ok(i) => i,
        Err(_) => 10,
    }
}
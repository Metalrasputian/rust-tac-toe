use std::io;
use std::io::prelude::*;
use std::convert::TryFrom;
use rand::prelude::*;
use exitcode;

fn main () {
    let mut playing = true;

    while playing {
        clear_screen();
        let mut round_won = false;

        let mut board = [" "; 9];

        let coin_flip = rand::thread_rng().gen_range(0..=1);
        let mut player_turn = coin_flip == 0;

        //Set up logic to change this to a coin flip for the user to be X or O

        let player_token = if player_turn {"X"} else {"O"};
        let bot_token = if player_turn {"O"} else {"X"};

        while !round_won {
            //Clear Screen
            println!("{}", render_screen(board));

            if player_turn {
                let player_command = get_player_command();

                if player_command.as_str().trim() == "exit" {
                    println!("Press any key to continue...");
                    io::stdin().read(&mut [0u8]).unwrap();
                    std::process::exit(exitcode::OK);
                }
                else { 
                    //Fix function to return Ok() and Err() instead of an int or 10 as an error
                    let move_index = get_player_move(player_command);
                    
                    if move_index < 10 {                
                        //Better error catching here
                        let board_index = usize::try_from(move_index - 1).unwrap();

                        let cell_value = board[board_index];

                        if cell_value != " " {
                            clear_screen();
                            println!("{} is not a valid square. ({} is there).", move_index, cell_value);
                        }
                        else {
                            clear_screen();
                            board[board_index] = player_token;

                            if check_win(board, player_token){
                                clear_screen();
                                println!("Player wins!");
                                round_won = true;
                            }
                        }
                    }
                    else {
                        println!("Invalid input. Please enter a number from 1 to 9!");
                    }
                }
            }
            else {
                //Bot move currently ignores board state. Likely a scope/mutability issue.
                let bot_move = get_bot_move(&board);
            
                board[bot_move] = bot_token;

                if check_win(board, bot_token){
                    clear_screen();
                    println!("Bot wins!");
                    round_won = true;
                }
            }

            player_turn = !player_turn;
        }
        //Get player command
        //Print final board        
        println!("{}", render_screen(board));

        //Exit code
        let mut exit_answer = String::new();        

        while exit_answer.as_str().trim() != "y" && exit_answer.as_str().trim() != "n" {
            println!("Would you like to play again? (y/n): ");

            io::stdin()
                .read_line(&mut exit_answer)
                .expect("No input detected!");

            match exit_answer.as_str().trim() {
                "n" => { println!("Goodbye!"); playing = false; },
                "y" => { println!("Another round!"); },
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

fn get_bot_move(board: &[&str;9]) -> usize {
    let mut valid_cells = Vec::<usize>::new();

    for i in 0..board.len() {
        if board[usize::try_from(i).unwrap()] == " "{
            valid_cells.push(usize::try_from(i).unwrap());
        }
    }

    let r_valid_move_index = rand::thread_rng().gen_range(0..valid_cells.len());

    valid_cells[r_valid_move_index]
}

fn get_player_command() -> String{
    let mut player_command = String::new();
    println!("Enter your next square (1-9): ");

    io::stdin()
        .read_line(&mut player_command)
        .expect("No input detected!");

    player_command.to_lowercase()
}

fn get_player_move(player_move: String) -> u32 {

    //Returns 10 since that's out of range for our logic.
    match player_move.trim().parse::<u32>() {
        Ok(i) => i,
        Err(_) => 10,
    }
}

fn check_win(board: [&str;9], token: &str) -> bool {

    //Check Rows
    for y in 0..3 {
        //Better parsing here
        let check_index = usize::try_from(y).unwrap();

        if board[check_index * 3] == token && board[(check_index * 3) + 1] == token && board[(check_index * 3) + 2] == token { return true; }
    }

    //Check Columns
    for x in 0..3 {
        //Better parsing here
        let check_index = usize::try_from(x).unwrap();

        if board[check_index] == token && board[check_index + 3] == token && board[check_index + 6] == token { return true; }
    }

    //Check Diagonals
    if board[0] == token && board[4] == token && board[8] == token || board[2] == token && board[4] == token && board[6] == token { return true; }

    false
}

fn clear_screen(){
    let _result = std::process::Command::new("cmd").arg("/C").arg("cls").status().or_else(|_| std::process::Command::new("clear").status()).unwrap().success();
}
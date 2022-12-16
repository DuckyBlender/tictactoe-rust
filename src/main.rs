use std::io;

// Function for drawing the game board
// &[&str] = &Vec<&str>
fn draw(array: &[&str]) {
    //for i in 0..array.len() {
    //    println!("{}", array[i]);
    //}
    println!("┌───┬───┬───┐");
    println!("│ {} │ {} │ {} │", array[0], array[1], array[2]);
    println!("├───┼───┼───┤");
    println!("│ {} │ {} │ {} │", array[3], array[4], array[5]);
    println!("├───┼───┼───┤");
    println!("│ {} │ {} │ {} │", array[6], array[7], array[8]);
    println!("└───┴───┴───┘");
}

fn main() {
    let mut array: Vec<&str> = vec![" "; 9];
    let mut win = false;
    let mut turn: usize = 0;

    println!("Welcome to Tic Tac Toe!");
    println!("Player 1 is X and Player 2 is O");
    println!("Enter the number of the square you want to place your piece in (1-9)");

    draw(&array);
    // Game loop
    loop {
        // Number input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {}
            Err(_error) => println!("Please enter a number."),
        }

        // Convert to int
        let mut input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        // Convert from 0-8 to 1-9
        input -= 1;

        // Check if square is in range (not checking below zero because it's unsigned)
        if input > 8 {
            println!("Please enter a number between 1 and 9.");
            // Repeat the loop
            continue;
        }

        // Check if square is free
        if array[input] != " " {
            println!("That square is already taken.");
            // Repeat the loop
            continue;
        }

        // Check if it's X or O turn
        if turn % 2 == 0 {
            // Set square to X
            array[input] = "X";
        } else {
            // Set square to O
            array[input] = "O";
        }
        turn += 1;

        // Check for win
        if array[0] == array[1] && array[1] == array[2] && array[0] != " " {
            win = true;
        }
        if array[3] == array[4] && array[4] == array[5] && array[3] != " " {
            win = true;
        }
        if array[6] == array[7] && array[7] == array[8] && array[6] != " " {
            win = true;
        }
        if array[0] == array[3] && array[3] == array[6] && array[0] != " " {
            win = true;
        }
        if array[1] == array[4] && array[4] == array[7] && array[1] != " " {
            win = true;
        }
        if array[2] == array[5] && array[5] == array[8] && array[2] != " " {
            win = true;
        }
        if array[0] == array[4] && array[4] == array[8] && array[0] != " " {
            win = true;
        }
        if array[2] == array[4] && array[4] == array[6] && array[2] != " " {
            win = true;
        }

        // Draw board
        draw(&array);

        // Current player
        if turn % 2 == 0 {
            println!("X's turn");
        } else {
            println!("O's turn");
        }

        // Check if game is a tie
        if turn == 9 {
            println!("It's a tie!");
            println!("Press enter to exit");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
        }

        // End game if win
        if win {
            println!("{} wins!", array[input]);
            println!("Press enter to exit");

            // Wait for enter
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            // End the game
            break;
        }
    }
}
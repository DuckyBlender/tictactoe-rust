use std::io;

fn draw(array: Vec<&str>) {
    //for i in 0..array.len() {
    //    println!("{}", array[i]);
    //}
    println!("{} | {} | {}", array[0],array[1],array[2]);
    println!("- + - + -");
    println!("{} | {} | {}", array[3],array[4],array[5]);
    println!("- + - + -");
    println!("{} | {} | {}", array[6],array[7],array[8]);
}

fn main() {
    //let mut array = vec![vec![0; width]; height];

    let mut array: Vec<&str> = vec![" "; 9];

    draw(array.clone());
   
    let mut win = false;

    let mut turn = 0;

    while !win{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut input: u8 = input.trim().parse().expect("Please type a number!");
        input = input - 1;
        if array[input as usize] != "X" && array[input as usize] != "O" {
          if turn % 2 == 0 {
            array[input as usize] = "X";
        } else {
            array[input as usize] = "O";
        }
        turn += 1;
        }

        //check win
        if array[0] == array[1] && array[1] == array[2] && array[0] != " " {
            win = true;
        } else if array[3] == array[4] && array[4] == array[5] && array[3] != " " {
            win = true;
        } else if array[6] == array[7] && array[7] == array[8] && array[6] != " " {
            win = true;
        } else if array[0] == array[3] && array[3] == array[6] && array[0] != " " {
            win = true;
        } else if array[1] == array[4] && array[4] == array[7] && array[1] != " " {
            win = true;
        } else if array[2] == array[5] && array[5] == array[8] && array[2] != " " {
            win = true;
        } else if array[0] == array[4] && array[4] == array[8] && array[0] != " " {
            win = true;
        } else if array[2] == array[4] && array[4] == array[6] && array[2] != " " {
            win = true;
        }
        draw(array.clone());
        
        if win {
            println!("{} win!", array[input as usize]);
        }

    }

}

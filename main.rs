use std::io;
use std::collections::HashMap;

fn main() {

    println!("Chess :)");
    //println!("{}, {}, {}, {}, {}", get_x_index(0), get_x_index(1), get_x_index(2), get_x_index(3), get_x_index(4));
    //println!("{}, {}, {}, {}, {}", get_y_index(0), get_y_index(1), get_y_index(2), get_y_index(3), get_y_index(4));


    let top = "        A     B     C     D     E     F     G     H";
    let vert_line = "     -------------------------------------------------";
    let w_line = "     ██████      ██████      ██████      ██████       ";
    let b_line = "           ██████      ██████      ██████      ██████ ";

    //println!("\n{}\n", top);

    // Create empty array to store board state
    let mut array: [char; 64] = [' '; 64];

    // We will write these into the array to define the starting position of the board
    let black: [char; 16] = ['R', 'H', 'B', 'Q', 'K', 'B', 'H', 'R', 'P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'];
    let white: [char; 16] = ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p', 'r', 'h', 'b', 'q', 'k', 'b', 'h', 'r'];
    
    for i in 0..64 {
        if i < 16 {
            array[i] = black[i];
        } 
        if i >= 48 {
            array[i] = white[i - 48];
        }
    }



    let char_piece_art_assoc = Hashmap::from([
        ("p", "()"),
        ("r", "uu"),
        ("h", "|>"),
        ("b", "{}"),
        ("q", "vv"),
        ("k", "++"),
        ("P", "()"),
        ("R", "nn"),
        ("H", "<|"),
        ("B", "{}"),
        ("Q", "^^"),
        ("K", "++"),
    ]);


    // Part A: create array of type String.
    // ... Use to_string for elements.
    let mut board: Vec<String> = vec![String::new(); 27];

    board[0] = "".to_string();
    board[1] = top.to_string();
    board[2] = "".to_string();
    for i in 0..=7 {
        let mut active_line: String = "no".to_string(); 
        if i % 2 == 0 {
            active_line = w_line.to_string();
        } else {
            active_line = b_line.to_string();
        }
        let mut active_line_with_num = active_line.clone();
        let mut gooba = format!("{}", 8 - i);
        active_line_with_num.replace_range(2..3, gooba.as_str());
        println!("{}, ", i);
        board[i * 3 + 0 + 3] = active_line.clone();
        //board[i * 4 + 1 + 3] = active_line.clone();
        board[i * 3 + 1 + 3] = active_line_with_num.to_string();
        board[i * 3 + 2 + 3] = active_line.clone();
    }

    //board[27] = vert_line.to_string();
    
    // Part B: print elements.
    for value in &board {
        println!("{}", value);
    }

    loop {

        // Generate the pieces of the board
       /* 
        for i in 

        
        print!("  ");
        for i in 0..=8 {
            for j in 0..=7 {
                if i == 0 {
                    print!("{} ", (j as u8 + 65) as char);
                } else {
                    print!("{} ", array[(i-1) * 8 + j]);
                }
            }
            if i != 8 {
                print!("\n{} ", (8 - i as u8 + 48) as char);
            } else {
                print!("\n");
            }
        }
        */
        println!("Select piece: ");
        let mut u_in = String::new();
        match io::stdin().read_line(&mut u_in) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        println!("{:?}, {:?}", u_in.chars().nth(0), u_in.chars().nth(1));
        println!("{}", u_in);
        println!("");
    }
}

fn get_x_index(x: i32) -> i32 {
    x * 6 + 8
}

fn get_y_index(y: i32) -> i32 {
    y * 4 + 5
}

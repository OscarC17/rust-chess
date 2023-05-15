use std::io;
use std::collections::HashMap;

fn main() {

    //println!("{}, {}, {}, {}, {}", get_x_index(0), get_x_index(1), get_x_index(2), get_x_index(3), get_x_index(4));
    //println!("{}, {}, {}, {}, {}", get_y_index(0), get_y_index(1), get_y_index(2), get_y_index(3), get_y_index(4));


    let top = "        A     B     C     D     E     F     G     H";
    let vert_line = "     -------------------------------------------------";
    //let w_line = "     ██████      ██████      ██████      ██████       ";
    //let b_line = "           ██████      ██████      ██████      ██████ ";
    let w_line = "     ######      ######      ######      ######       ";
    let b_line = "           ######      ######      ######      ###### ";

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



    let char_piece_art_assoc = HashMap::from([
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
        (" ", ""),
    ]);


    // Part A: create array of type String.
    // ... Use to_string for elements.
    let mut board: Vec<String> = vec![String::new(); 27];

    loop {

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
            let mut active_line_top = active_line.clone();
            let mut active_line_bottom = active_line.clone();
            let mut gooba = format!("{}", 8 - i);
            active_line_with_num.replace_range(2..3, gooba.as_str());
            for j in 0..=7 {
               // println!("3");
               // println!("{}", array[i as usize * 8 + j as usize]);
               // println!("{}", char_piece_art_assoc[&array[i as usize * 8 + j as usize].to_string() as &str]);
               // println!("{}, {}", get_x_index(j), get_x_index(j) + 2);
                if (array[i as usize * 8 + j as usize] != ' ') {
                    active_line_with_num.replace_range(get_x_index(j)..(get_x_index(j) + 2), char_piece_art_assoc[&array[i as usize * 8 + j as usize].to_string() as &str]);
                }
                if array[i as usize * 8 + j as usize] > 96 as char {
                    active_line_bottom.replace_range(get_x_index(j)..(get_x_index(j) + 2), "][");
                } else if array[i as usize * 8 + j as usize] != 32 as char {
                    active_line_top.replace_range(get_x_index(j)..(get_x_index(j) + 2), "][");
                }
                //active_line_with_num.replace_range(7..8, &char_piece_art_assoc[&array[i as usize * 8 + j as usize].to_string() as &str]);
                //active_line_with_num.replace_range(3..5, "yy");
                
            }
            //println!("{}, ", i);
            board[i as usize * 3 + 0 + 3] = active_line_top.to_string();
            //board[i * 4 + 1 + 3] = active_line.clone();
            board[i as usize * 3 + 1 + 3] = active_line_with_num.to_string();
            board[i as usize * 3 + 2 + 3] = active_line_bottom.to_string();
        }

        //board[27] = vert_line.to_string();
        
        // Part B: print elements.
        for value in &board {
            println!("{}", value);
        }


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
        //println!("{:?}, {:?}", u_in.chars().nth(0), u_in.chars().nth(1));
        println!("{}, {}", letter_to_num(u_in.chars().nth(0).unwrap()), 8 - (u_in.chars().nth(1).unwrap() as usize - 48 as usize));
        let mut from_index = letter_to_num(u_in.chars().nth(0).unwrap()) + 8 * (8 - (u_in.chars().nth(1).unwrap() as usize - 48 as usize));
        println!("{}", array[from_index]);
        println!("");

        u_in = String::new();
        match io::stdin().read_line(&mut u_in) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        let mut to_index = letter_to_num(u_in.chars().nth(0).unwrap()) + 8 * (8 - (u_in.chars().nth(1).unwrap() as usize - 48 as usize));
        array[to_index] = array[from_index];
        array[from_index] = ' ';
    }
}

fn get_x_index(x: i32) -> usize {
    (x * 6 + 7) as usize
}

fn get_y_index(y: i32) -> usize {
    (y * 4 + 5) as usize
}

fn letter_to_num(alpha: char) -> usize {
    if alpha as usize >= 97 {
        return alpha as usize - 97;
    } else {
        return alpha as usize - 65;
    }
}

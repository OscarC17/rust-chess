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

    let ruleset = HashMap::new();

    ruleset.insert("p", b"%as0001%bs0002%cs0101%ds-101");
    ruleset.insert("r", b"%ar0100%ar-100%ar0001%ar00-1");
    ruleset.insert("h", b"%as0102%as0201%as02-1%as01-2%as-102%as-201%as-2-1%as-1-2");


    // Part A: create array of type String.
    // ... Use to_string for elements.
    let mut board: Vec<String> = vec![String::new(); 27];

    const WHITE: bool = false;
    const BLACK: bool = true;
    let mut turn: bool = WHITE;

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
        
        print!("Select piece ");

        if turn == WHITE {
            println!("(White's turn):")
        } else {
            println!("(Black's turn):")
        }

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
        if array[from_index] != ' ' && white_or_black(array[from_index]) == turn {
            array[to_index] = array[from_index];
            array[from_index] = ' ';
        } else {
            println!("You do not own that piece");
        }

        turn = !turn;
    }
}

fn white_or_black(x: char) -> bool {
    (x as u16) < 96
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



fn check_move_against_ruleset(dx: usize, dy: usize, rule: &[u8], argument: &[u8]) -> bool {
    for byte in argument.bytes() {
        if (byte as char) == '%' 
    }
       
}
    /*let ruleset = HashMap::from([
        ("p", "%as0001%bs0002%cs0101%ds-101"),
        ("r", "%ar0100%ar-100%ar0001%ar00-1"),
        ("h", "%as0102%as0201%as02-1%as01-2%as-102%as-201%as-2-1%as-1-2");
    ]);*/













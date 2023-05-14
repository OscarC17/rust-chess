use std::io;
fn main() {

    println!("Chess :)");


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



    let stdin = io::stdin();
    let mut iterator: u8 = 0;
    for piece in array { 
        if iterator % 8 == 0 {
            println!("----------------------------------------------");
        }
        iterator += 1;
    }

    loop {
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

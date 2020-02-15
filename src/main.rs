use std::io::{stdin, stdout, Write};
use std::collections::HashMap;

struct Game {
    amount_black_pieces: u8,
    amount_white_pieces: u8,
    dark_squares: HashMap<String, String>,
}

impl Game {
    fn new() -> Game {
        let mut dark_squares = HashMap::new();
        for i in 1..9 {
            for j in 0..4 {
                let mut x_position_num = 65u8;// as char;
                x_position_num += if (i % 2) == 0 {2*j+1} else {(2*j)};
                let x_position_char = x_position_num as char;

                if i <= 3 {
                    dark_squares.insert(format!("{}{}", x_position_char, i), "  ○  ".to_string());
                }
                else if i <= 5 {
                    dark_squares.insert(format!("{}{}", x_position_char, i), "     ".to_string());
                }
                else {
                    dark_squares.insert(format!("{}{}", x_position_char, i), "  ●  ".to_string());
                }
            }
        }

        Game {amount_black_pieces: 12, amount_white_pieces: 12, dark_squares: dark_squares}
    }
}

fn main() {
    let mut x = 42;
    let mut game = Game::new();
    x += 2;
    println!("Hello World!");
    println!();
    print!("{}[2J", 27 as char);
    print_screen(game);
    println!();
    println!("End");
}

fn print_screen(game: Game) {
    println!("    A    B    C    D    E    F    G    H");
    for i in (0..24).rev() {
        if (i % 3) != 1 {
            if ((i / 3) % 2) == 0 {
                println!("       ■■■■■     ■■■■■     ■■■■■     ■■■■■    ");
            }
            else {
                println!("  ■■■■■     ■■■■■     ■■■■■     ■■■■■       ");
            }
        }
        else {
            print!("{} ", (i / 3) + 1);
            for j in 0..8 {
                if ((i + j) % 2) == 0 {
                    print!("■■■■■");
                }
                else {
                    let mut x_position_num = 65u8;
                    x_position_num += j;
                    let x_position_char = x_position_num as char;
                    let y_position_char = ((i/3) + 1 + 48) as char;
                    print!("{}", game.dark_squares[&format!("{}{}", x_position_char, y_position_char)]);
                }
            }
            println!(" {}", (i / 3) + 1);
        }
    }
    println!("    A    B    C    D    E    F    G    H");
}

fn validate_input() {

}

fn process_input() {

}

fn check_mandatory_move() {

}
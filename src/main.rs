use std::io::{self, Write};
use std::collections::HashMap;
use std::collections::HashSet;
use std::process;
use std::cmp;

const EMPTY_CHECKER: &'static str = "     ";
const BLACK_CHECKER: &'static str = "  ○  ";
const WHITE_CHECKER: &'static str = "  ●  ";
const BLACK_KING: &'static str = "  ☖  ";
const WHITE_KING: &'static str = "  ☗  ";
// const BLACK_KING: &'static str = "  ♔  ";
// const WHITE_KING: &'static str = "  ♚  ";

#[derive(Clone)]
struct Game {
    amount_black_pieces: u8,
    amount_white_pieces: u8,
    dark_squares: HashMap<String, String>,
    mode: u8,
    black_turn: bool,
    player_1_black: bool,
}

impl Game {
    fn new(mode: u8, player_1_black: bool) -> Game {
        let mut dark_squares = HashMap::new();
        for i in 1..9 {
            for j in 0..4 {
                let mut x_position_num = 65u8;// as char;
                x_position_num += if (i % 2) == 0 {2*j+1} else {(2*j)};
                let x_position_char = x_position_num as char;

                if i <= 3 {
                    dark_squares.insert(format!("{}{}", x_position_char, i), BLACK_CHECKER.to_string());
                }
                else if i <= 5 {
                    //println!("Placing empty at: {}", format!("{}{}", x_position_char, i));
                    dark_squares.insert(format!("{}{}", x_position_char, i), EMPTY_CHECKER.to_string());
                }
                else {
                    dark_squares.insert(format!("{}{}", x_position_char, i), WHITE_CHECKER.to_string());
                }
            }
        }

        Game {amount_black_pieces: 12, amount_white_pieces: 12, dark_squares: dark_squares, mode: mode, black_turn: true, player_1_black: player_1_black}
    }
}

fn main() {
    let mut game;// = Game::new(1);

    print_menu();
    let choice_mode = get_choice();
    print!("{}[2J", 27 as char);
    if choice_mode == 1 {
        print_color_selection();
        let choice_color = get_choice();
        print!("{}[2J", 27 as char);
        game = Game::new(1, choice_color == 1);
    }
    else if choice_mode == 2 {
        game = Game::new(2, true);
        //get_choice();
    }
    else {
        process::exit(0);
    }

    let mut set: HashSet<String>;
    set = HashSet::new();
    //game.dark_squares.insert("D4".to_string(), WHITE_CHECKER.to_string());
    loop {
        if (game.amount_black_pieces == 0) || (game.amount_white_pieces == 0) {
            break;
        }

        print!("{}[2J", 27 as char);
        println!("White: {}\tBlack: {}", game.amount_white_pieces, game.amount_black_pieces);
        print_screen(game.clone());

        check_mandatory_move(game.clone(), game.black_turn, &mut set);
        
        if game.mode == 1 {
            if game.player_1_black == game.black_turn {
                if set.is_empty() {
                    println!("There are no movement requirements");
                }
                else {
                    print!("You need to move one of the following checkers: ");
                    for value in set.clone() {
                        print!("{} ", value);
                    }
                    println!("");
                }
                
                print!("Enter your movement order: ");
                io::stdout().flush().expect("Error flushing stdout");
                let mut movement = get_move();
        
                while !validate_input(game.clone(), game.black_turn, movement.clone(), &set) {
                    print!("{}[2J", 27 as char);
                    println!("White: {}\tBlack: {}", game.amount_white_pieces, game.amount_black_pieces);
                    print_screen(game.clone());
                    check_mandatory_move(game.clone(), game.black_turn, &mut set);
                    print!("Not a valid movement. You need to move one of the following checkers: ");
                    for value in set.clone() {
                        print!("{} ", value);
                    }
                    println!("");
                    print!("Enter your movement order: ");
                    io::stdout().flush().expect("Error flushing stdout");
                    movement = get_move();
                }
        
                game = process_input(game.clone(), movement.clone());
            }
            else {
                game = make_cpu_movement(game.clone(), game.black_turn, &mut set);
            }
        }

        if game.mode == 2 {
            if set.is_empty() {
                println!("There are no movement requirements");
            }
            else {
                print!("You need to move one of the following checkers: ");
                for value in set.clone() {
                    print!("{} ", value);
                }
                println!("");
            }

            if game.black_turn {
                print!("Player 1, enter your movement order: ");
            }
            else {
                print!("Player 2, enter your movement order: ");
            }
            io::stdout().flush().expect("Error flushing stdout");
            let mut movement = get_move();
    
            while !validate_input(game.clone(), game.black_turn, movement.clone(), &set) {
                print!("{}[2J", 27 as char);
                println!("White: {}\tBlack: {}", game.amount_white_pieces, game.amount_black_pieces);
                print_screen(game.clone());
                //set = check_mandatory_move(game.clone(), true, &mut set);
                check_mandatory_move(game.clone(), game.black_turn, &mut set);
                if set.is_empty() {
                    println!("Not a valid movement, please try again");
                }
                else {
                    print!("Not a valid movement. You need to move one of the following checkers: ");
                    for value in set.clone() {
                        print!("{} ", value);
                    }
                    println!("");
                }
                
                print!("Enter your movement order: ");
                io::stdout().flush().expect("Error flushing stdout");
                movement = get_move();
            }
    
            game = process_input(game.clone(), movement.clone());
        }

        game.black_turn = !game.black_turn;
    }
    
    
    if game.mode == 1 {
        if (game.amount_black_pieces == 0) == game.player_1_black {
            println!("The CPU wins!")
        }
        else {
            println!("You win!")
        }
    }
    else if game.mode == 2 {
        if game.amount_white_pieces == 0 {
            println!("Player 1 wins!");
        } 
        else {
            println!("Player 2 wins!");
        }
    }
}

fn print_screen(game: Game) {
    println!("    A    B    C    D    E    F    G    H");
    for i in (0..24).rev() {
        if (i % 3) != 1 {
            if ((i / 3) % 2) == 0 {
                println!("       █████     █████     █████     █████    ");
            }
            else {
                println!("  █████     █████     █████     █████       ");
            }
        }
        else {
            print!("{} ", (i / 3) + 1);
            for j in 0..8 {
                if ((i + j) % 2) == 0 {
                    print!("█████");
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

fn print_menu() {
    println!("Welcome to Checkers");
    println!("Choose your option:");
    println!("\t1 - Single player mode");
    println!("\t2 - Multiplayer mode");
    println!("\t3 - Exit");
}

fn print_color_selection() {
    println!("Which color do you want to play?");
    println!("Choose your option:");
    println!("\t1 - Black");
    println!("\t2 - White");
}

fn get_choice()  -> u8 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let choice: u8 = input.trim().parse::<u8>().unwrap();

    choice
}

fn get_move() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    input
}

fn validate_input(game: Game, black_turn: bool, mut input: String, requirements: &HashSet<String>) -> bool {
    input.pop();
    input = input.to_ascii_uppercase();
    let path: Vec<&str> = input.split(' ').collect();
    
    let initial_position :String = (*path.get(0).unwrap()).to_string();
    //Check if the initial position is valid and lays inside the board
    if !check_valid_position(initial_position.clone()) {
        return false;
    }

    if !requirements.is_empty() && !requirements.contains(&initial_position) {
        return false;
    }
    
    let game_clone = game.clone();
    let initial_checker = &game_clone.dark_squares[&initial_position];
    let mut current_position :String;// = &path.get(0).unwrap().to_string();
    let mut current_position_bytes :&[u8];// = current_position.as_bytes();
    let mut next_position :String;
    let mut next_position_bytes :&[u8];

    if initial_checker == BLACK_CHECKER {
        // Checking if it is allowed to move a black checker 
        if !black_turn {
            return false;
        }

        for i in 1..path.len() {
            current_position = (*path.get(i-1).unwrap()).to_string();
            current_position_bytes = current_position.as_bytes();
            //Check if the current position is valid and lays inside the board
            if !check_valid_position(current_position.clone()) {
                return false;
            }
            
            next_position = (*path.get(i).unwrap()).to_string();
            next_position_bytes = next_position.as_bytes();
            //Check if the next position is valid and lays inside the board 
            if !check_valid_position(next_position.clone()) {
                return false;
            }

            //Check it is moving forward
            if next_position_bytes[1] <= current_position_bytes[1] {
                return false;
            }

            let mut x_diff :u8;
            let mut y_diff :u8 = next_position_bytes[1] - current_position_bytes[1];
            // If moving right
            if next_position_bytes[0] > current_position_bytes[0] {
                x_diff = next_position_bytes[0] - current_position_bytes[0];

                // If not moving diagonally to the right
                if x_diff != y_diff {
                    return false;
                }

                // Moved 1 square, should be moving to an empty square
                if x_diff == 1 {
                    //println!("Checking empty at: |{}|", next_position);
                    if game_clone.dark_squares[&next_position] != EMPTY_CHECKER {
                        return false;
                    }
                }
                // Moved 2 squares, should be making a capture
                else if x_diff == 2 {
                    if game_clone.dark_squares[&format!("{}{}", (current_position_bytes[0]+1) as char, (current_position_bytes[1]+1) as char)] != WHITE_CHECKER 
                        && game_clone.dark_squares[&format!("{}{}", (current_position_bytes[0]+1) as char, (current_position_bytes[1]+1) as char)] != WHITE_KING {
                        return false;
                    }
                    if game_clone.dark_squares[&next_position] != EMPTY_CHECKER {
                        return false;
                    }
                }
                // Moved more than 2 squares, only allowed for kings
                else {
                    return false;
                }
            }
            // Else, not moving right
            else {
                x_diff = current_position_bytes[0] - next_position_bytes[0];

                //If not moving diagonally to the left
                if x_diff != y_diff {
                    return false;
                }

                // Moved 1 square, should be moving to an empty square
                if x_diff == 1 {
                    if game_clone.dark_squares[&next_position] != EMPTY_CHECKER {
                        return false;
                    }
                }
                // Moved 2 squares, should be making a capture
                else if x_diff == 2 {
                    if game_clone.dark_squares[&format!("{}{}", (current_position_bytes[0]-1) as char, (current_position_bytes[1]+1) as char)] != WHITE_CHECKER 
                        && game_clone.dark_squares[&format!("{}{}", (current_position_bytes[0]-1) as char, (current_position_bytes[1]+1) as char)] != WHITE_KING {
                        return false;
                    }
                    if game_clone.dark_squares[&next_position] != EMPTY_CHECKER {
                        return false;
                    }
                }
                // Moved more than 2 squares, only allowed for kings
                else {
                    return false;
                }
            }
        }

        return true;
    }
    if initial_checker == WHITE_CHECKER {
        // Checking if it is allowed to move a white checker 
        if black_turn {
            return false;
        }

        for i in 1..path.len() {
            current_position = (*path.get(i-1).unwrap()).to_string();
            current_position_bytes = current_position.as_bytes();
            //Check if the current position is valid and lays inside the board
            if !check_valid_position(current_position.clone()) {
                return false;
            }

            next_position = (*path.get(i).unwrap()).to_string();
            next_position_bytes = next_position.as_bytes(); 
            //Check if the next position is valid and lays inside the board 
            if !check_valid_position(next_position.clone()) {
                return false;
            }
            //Check it is moving forward
            if next_position_bytes[1] >= current_position_bytes[1] {
                return false;
            }

            let mut x_diff :u8;
            let mut y_diff :u8 = current_position_bytes[1] - next_position_bytes[1];
            // If moving right
            if next_position_bytes[0] > current_position_bytes[0] {
                x_diff = next_position_bytes[0] - current_position_bytes[0];

                // If not moving diagonally to the right
                if x_diff != y_diff {
                    return false;
                }

                // Moved 1 square, should be moving to an empty square
                if x_diff == 1 {
                    if game_clone.dark_squares[&next_position] != EMPTY_CHECKER {
                        return false;
                    }
                }
                // Moved 2 squares, should be making a capture
                else if x_diff == 2 {
                    if game_clone.dark_squares[&format!("{}{}", (current_position_bytes[0]+1) as char, (current_position_bytes[1]-1) as char)] != BLACK_CHECKER
                        && game_clone.dark_squares[&format!("{}{}", (current_position_bytes[0]+1) as char, (current_position_bytes[1]-1) as char)] != BLACK_KING {
                        return false;
                    }
                    if game_clone.dark_squares[&next_position] != EMPTY_CHECKER {
                        return false;
                    }
                }
                // Moved more than 2 squares, only allowed for kings
                else {
                    return false;
                }
            }
            // Else, not moving right
            else {
                x_diff = current_position_bytes[0] - next_position_bytes[0];

                //If not moving diagonally to the left
                if x_diff != y_diff {
                    return false;
                }

                // Moved 1 square, should be moving to an empty square
                if x_diff == 1 {
                    if game_clone.dark_squares[&next_position] != EMPTY_CHECKER {
                        return false;
                    }
                }
                // Moved 2 squares, should be making a capture
                else if x_diff == 2 {
                    if game_clone.dark_squares[&format!("{}{}", (current_position_bytes[0]-1) as char, (current_position_bytes[1]-1) as char)] != BLACK_CHECKER 
                        && game_clone.dark_squares[&format!("{}{}", (current_position_bytes[0]-1) as char, (current_position_bytes[1]-1) as char)] != BLACK_KING {
                        return false;
                    }
                    if game_clone.dark_squares[&next_position] != EMPTY_CHECKER {
                        return false;
                    }
                }
                // Moved more than 2 squares, only allowed for kings
                else {
                    return false;
                }
            }
        }

        return true;
    }
    if initial_checker == BLACK_KING {
        // Checking if it is allowed to move a black checker 
        if !black_turn {
            return false;
        }

        for i in 1..path.len() {
            current_position = (*path.get(i-1).unwrap()).to_string();
            current_position_bytes = current_position.as_bytes();
            //Check if the current position is valid and lays inside the board
            if !check_valid_position(current_position.clone()) {
                return false;
            }
            
            next_position = (*path.get(i).unwrap()).to_string();
            next_position_bytes = next_position.as_bytes();
            //Check if the next position is valid and lays inside the board 
            if !check_valid_position(next_position.clone()) {
                return false;
            }

            let x_diff = cmp::max(current_position_bytes[0], next_position_bytes[0]) - cmp::min(current_position_bytes[0], next_position_bytes[0]);
            let y_diff = cmp::max(current_position_bytes[1], next_position_bytes[1]) - cmp::min(current_position_bytes[1], next_position_bytes[1]);

            // Check if moving diagonally, and if actually moving
            if (x_diff != y_diff) || (x_diff == 0) {
                return false;
            }

            // Check if landing on a non-empty square
            if game_clone.dark_squares[&next_position] != EMPTY_CHECKER {
                return false;
            }

            let going_up :bool = next_position_bytes[1] > current_position_bytes[1];
            let going_left :bool = next_position_bytes[0] < current_position_bytes[0];

            let mut found_white :bool = false;
            let mut diagonal :&String;
            let mut new_x_position :u8 = current_position_bytes[0];
            let mut new_y_position :u8 = current_position_bytes[1];
            //Up and left
            for _i in 1..(x_diff+1) {
                if going_up {
                    new_y_position += 1;
                }
                else {
                    new_y_position -= 1;
                }

                if going_left {
                    new_x_position -= 1;
                }
                else {
                    new_x_position += 1;
                }

                diagonal = &game_clone.dark_squares[&format!("{}{}", (new_x_position) as char, (new_y_position) as char)];
                if (diagonal == BLACK_CHECKER) || (diagonal == BLACK_KING) {
                    return false;
                }
                if (diagonal == WHITE_CHECKER) || (diagonal == WHITE_KING) {
                    if found_white {
                        return false;
                    }
                    else {
                        found_white = true;
                    }
                }
            }
        }

        return true;
    }
    if initial_checker == WHITE_KING {
        // Checking if it is allowed to move a white checker 
        if black_turn {
            return false;
        }

        for i in 1..path.len() {
            current_position = (*path.get(i-1).unwrap()).to_string();
            current_position_bytes = current_position.as_bytes();
            //Check if the current position is valid and lays inside the board
            if !check_valid_position(current_position.clone()) {
                return false;
            }
            
            next_position = (*path.get(i).unwrap()).to_string();
            next_position_bytes = next_position.as_bytes();
            //Check if the next position is valid and lays inside the board 
            if !check_valid_position(next_position.clone()) {
                return false;
            }

            let x_diff = cmp::max(current_position_bytes[0], next_position_bytes[0]) - cmp::min(current_position_bytes[0], next_position_bytes[0]);
            let y_diff = cmp::max(current_position_bytes[1], next_position_bytes[1]) - cmp::min(current_position_bytes[1], next_position_bytes[1]);

            // Check if moving diagonally, and if actually moving
            if (x_diff != y_diff) || (x_diff == 0) {
                return false;
            }

            // Check if landing on a non-empty square
            if game_clone.dark_squares[&next_position] != EMPTY_CHECKER {
                return false;
            }

            let going_up :bool = next_position_bytes[1] > current_position_bytes[1];
            let going_left :bool = next_position_bytes[0] < current_position_bytes[0];

            let mut found_black :bool = false;
            let mut diagonal :&String;
            let mut new_x_position :u8 = current_position_bytes[0];
            let mut new_y_position :u8 = current_position_bytes[1];
            //Up and left
            for i in 1..(x_diff+1) {
                if going_up {
                    new_y_position += 1;
                }
                else {
                    new_y_position -= 1;
                }

                if going_left {
                    new_x_position -= 1;
                }
                else {
                    new_x_position += 1;
                }

                diagonal = &game_clone.dark_squares[&format!("{}{}", (new_x_position) as char, (new_y_position) as char)];
                if (diagonal == BLACK_CHECKER) || (diagonal == BLACK_KING) {
                    if found_black {
                        return false;
                    }
                    else {
                        found_black = true;
                    }
                }
                if (diagonal == WHITE_CHECKER) || (diagonal == WHITE_KING) {
                    return false;
                }
            }
        }

        return true;
    }
    false
}

fn process_input(mut game: Game, mut input: String) -> Game {
    input.pop();
    input = input.to_ascii_uppercase();
    let path: Vec<&str> = input.split(' ').collect();
    
    let initial_position :String = (*path.get(0).unwrap()).to_string();
    let game_clone = game.clone();
    let initial_checker = &game_clone.dark_squares[&initial_position];
    
    let mut current_position :String = path.get(0).unwrap().to_string();
    let mut current_position_bytes :&[u8] = current_position.as_bytes();
    let mut next_position :String = path.get(0).unwrap().to_string();
    let mut next_position_bytes :&[u8] = current_position.as_bytes();

    if initial_checker == BLACK_CHECKER {
        for i in 1..path.len() {
            current_position = (*path.get(i-1).unwrap()).to_string();
            current_position_bytes = current_position.as_bytes();
            next_position = (*path.get(i).unwrap()).to_string();
            next_position_bytes = next_position.as_bytes();

            game.dark_squares.insert(current_position.clone(), EMPTY_CHECKER.to_string());
            game.dark_squares.insert(next_position.clone(), BLACK_CHECKER.to_string());

            if (next_position_bytes[1] - current_position_bytes[1]) == 2 {
                if next_position_bytes[0] > current_position_bytes[0] {
                    game.dark_squares.insert(format!("{}{}", (current_position_bytes[0]+1) as char, (current_position_bytes[1]+1) as char), EMPTY_CHECKER.to_string());
                }
                else {
                    game.dark_squares.insert(format!("{}{}", (current_position_bytes[0]-1) as char, (current_position_bytes[1]+1) as char), EMPTY_CHECKER.to_string());
                }
                
                game.amount_white_pieces -= 1;
            }

        }

        //Check if turned into a king
        if next_position_bytes[1] == 56 {
            game.dark_squares.insert(next_position.clone(), BLACK_KING.to_string());
        }
    }

    if initial_checker == WHITE_CHECKER {
        for i in 1..path.len() {
            current_position = (*path.get(i-1).unwrap()).to_string();
            current_position_bytes = current_position.as_bytes();
            next_position = (*path.get(i).unwrap()).to_string();
            next_position_bytes = next_position.as_bytes();

            game.dark_squares.insert(current_position.clone(), EMPTY_CHECKER.to_string());
            game.dark_squares.insert(next_position.clone(), WHITE_CHECKER.to_string());

            if (current_position_bytes[1] - next_position_bytes[1]) == 2 {
                if next_position_bytes[0] > current_position_bytes[0] {
                    game.dark_squares.insert(format!("{}{}", (current_position_bytes[0]+1) as char, (current_position_bytes[1]-1) as char), EMPTY_CHECKER.to_string());
                }
                else {
                    game.dark_squares.insert(format!("{}{}", (current_position_bytes[0]-1) as char, (current_position_bytes[1]-1) as char), EMPTY_CHECKER.to_string());
                }
                
                game.amount_black_pieces -= 1;
            }

        }

        //Check if turned into a king
        if next_position_bytes[1] == 49 {
            game.dark_squares.insert(next_position.clone(), WHITE_KING.to_string());
        }
    }

    if initial_checker == BLACK_KING {
        for i in 1..path.len() {
            current_position = (*path.get(i-1).unwrap()).to_string();
            current_position_bytes = current_position.as_bytes();
            next_position = (*path.get(i).unwrap()).to_string();
            next_position_bytes = next_position.as_bytes();

            game.dark_squares.insert(current_position.clone(), EMPTY_CHECKER.to_string());
            game.dark_squares.insert(next_position.clone(), BLACK_KING.to_string());

            let x_diff = cmp::max(current_position_bytes[0], next_position_bytes[0]) - cmp::min(current_position_bytes[0], next_position_bytes[0]);
            let going_up :bool = next_position_bytes[1] > current_position_bytes[1];
            let going_left :bool = next_position_bytes[0] < current_position_bytes[0];

            let mut diagonal :&String;
            let mut new_x_position :u8 = current_position_bytes[0];
            let mut new_y_position :u8 = current_position_bytes[1];
            
            for _i in 1..(x_diff+1) {
                if going_up {
                    new_y_position += 1;
                }
                else {
                    new_y_position -= 1;
                }

                if going_left {
                    new_x_position -= 1;
                }
                else {
                    new_x_position += 1;
                }

                diagonal = &game_clone.dark_squares[&format!("{}{}", (new_x_position) as char, (new_y_position) as char)];
                
                if (diagonal == WHITE_CHECKER) || (diagonal == WHITE_KING) {
                    game.dark_squares.insert(format!("{}{}", new_x_position as char, new_y_position as char), EMPTY_CHECKER.to_string());
                    game.amount_white_pieces -= 1;
                    break;
                }
            }
        }
    }
    if initial_checker == WHITE_KING {
        for i in 1..path.len() {
            current_position = (*path.get(i-1).unwrap()).to_string();
            current_position_bytes = current_position.as_bytes();
            next_position = (*path.get(i).unwrap()).to_string();
            next_position_bytes = next_position.as_bytes();

            game.dark_squares.insert(current_position.clone(), EMPTY_CHECKER.to_string());
            game.dark_squares.insert(next_position.clone(), WHITE_KING.to_string());

            let x_diff = cmp::max(current_position_bytes[0], next_position_bytes[0]) - cmp::min(current_position_bytes[0], next_position_bytes[0]);
            let going_up :bool = next_position_bytes[1] > current_position_bytes[1];
            let going_left :bool = next_position_bytes[0] < current_position_bytes[0];

            let mut diagonal :&String;
            let mut new_x_position :u8 = current_position_bytes[0];
            let mut new_y_position :u8 = current_position_bytes[1];
            
            for _i in 1..(x_diff+1) {
                if going_up {
                    new_y_position += 1;
                }
                else {
                    new_y_position -= 1;
                }

                if going_left {
                    new_x_position -= 1;
                }
                else {
                    new_x_position += 1;
                }

                diagonal = &game_clone.dark_squares[&format!("{}{}", (new_x_position) as char, (new_y_position) as char)];
                
                if (diagonal == BLACK_CHECKER) || (diagonal == BLACK_KING) {
                    game.dark_squares.insert(format!("{}{}", new_x_position as char, new_y_position as char), EMPTY_CHECKER.to_string());
                    game.amount_black_pieces -= 1;
                    break;
                }
            }
        }
    }
    
    //print_screen(game.clone());
    //get_choice();
    game
}

fn check_mandatory_move(game: Game, black_turn: bool, checkers_capturing_position: &mut HashSet<String>) {//-> HashSet<String> {
    // let mut checkers_capturing_position: HashSet<String>;
    // checkers_capturing_position = HashSet::new();
    //let mut game_clone = game.clone();
    checkers_capturing_position.clear();

    for (position, value) in game.clone().dark_squares {
        if black_turn {
            if value == BLACK_CHECKER {
                if check_can_capture(game.clone(), position.clone(), true, false) { checkers_capturing_position.insert(position); }
            }
            else if value == BLACK_KING {
                if check_can_capture(game.clone(), position.clone(), true, true) { checkers_capturing_position.insert(position); }
            }
        }
        else {
            if value == WHITE_CHECKER {
                if check_can_capture(game.clone(), position.clone(), false, false) { checkers_capturing_position.insert(position); }
            }
            else if value == WHITE_KING {
                if check_can_capture(game.clone(), position.clone(), false, true) { checkers_capturing_position.insert(position); }
            }
        }
    }

    //checkers_capturing_position
}

fn check_can_capture(game: Game, initial_position: String, is_black: bool, is_king: bool) -> bool {
    // println!("{:#?}", initial_position.as_bytes());
    // println!("{}", initial_position);
    let initial_position_bytes = initial_position.as_bytes();

    let mut diagonal_right_1;
    let diagonal_right_2;
    let mut diagonal_left_1;
    let diagonal_left_2;

    if is_black {
        if !is_king {
            if initial_position_bytes[0] >= 67 && initial_position_bytes[1] <= 54 {
                diagonal_left_1 = &game.dark_squares[&format!("{}{}", (initial_position_bytes[0]-1) as char, (initial_position_bytes[1]+1) as char)];
                diagonal_left_2 = &game.dark_squares[&format!("{}{}", (initial_position_bytes[0]-2) as char, (initial_position_bytes[1]+2) as char)];
                if (diagonal_left_1 == WHITE_CHECKER || diagonal_left_1 == WHITE_KING) && diagonal_left_2 == EMPTY_CHECKER { return true; }
            } 
            if initial_position_bytes[0] <= 70 && initial_position_bytes[1] <= 54 {
                diagonal_right_1 = &game.dark_squares[&format!("{}{}", (initial_position_bytes[0]+1) as char, (initial_position_bytes[1]+1) as char)];
                diagonal_right_2 = &game.dark_squares[&format!("{}{}", (initial_position_bytes[0]+2) as char, (initial_position_bytes[1]+2) as char)];
                if (diagonal_right_1 == WHITE_CHECKER || diagonal_right_1 == WHITE_KING) && diagonal_right_2 == EMPTY_CHECKER { return true; }
            }
            else { return false;}
        }
        else {
            let mut found_white :bool = false;
            //Up and left
            for i in 1..(cmp::min(initial_position_bytes[0]-65, 56-initial_position_bytes[1])+1) {
                diagonal_left_1 = &game.dark_squares[&format!("{}{}", (initial_position_bytes[0]-i) as char, (initial_position_bytes[1]+i) as char)];
                if (diagonal_left_1 == BLACK_CHECKER) || (diagonal_left_1 == BLACK_KING) {
                    break;
                }
                if (diagonal_left_1 == WHITE_CHECKER) || (diagonal_left_1 == WHITE_KING) {
                    if found_white {
                        break;
                    }
                    else {
                        found_white = true;
                    }
                }
                else if diagonal_left_1 == EMPTY_CHECKER {
                    if found_white {
                        return true;
                    }
                }
            }
            found_white = false;
            //Up and right
            for i in 1..(cmp::min(72-initial_position_bytes[0], 56-initial_position_bytes[1])+1) {
                diagonal_right_1 = &game.dark_squares[&format!("{}{}", (initial_position_bytes[0]+i) as char, (initial_position_bytes[1]+i) as char)];
                if (diagonal_right_1 == BLACK_CHECKER) || (diagonal_right_1 == BLACK_KING) {
                    break;
                }
                if (diagonal_right_1 == WHITE_CHECKER) || (diagonal_right_1 == WHITE_KING) {
                    if found_white {
                        break;
                    }
                    else {
                        found_white = true;
                    }
                }
                else if diagonal_right_1 == EMPTY_CHECKER {
                    if found_white {
                        return true;
                    }
                }
            }
            found_white = false;
            //Down and left
            for i in 1..(cmp::min(initial_position_bytes[0]-65, initial_position_bytes[1]-49)+1) {
                diagonal_left_1 = &game.dark_squares[&format!("{}{}", (initial_position_bytes[0]-i) as char, (initial_position_bytes[1]-i) as char)];
                if (diagonal_left_1 == BLACK_CHECKER) || (diagonal_left_1 == BLACK_KING) {
                    break;
                }
                if (diagonal_left_1 == WHITE_CHECKER) || (diagonal_left_1 == WHITE_KING) {
                    if found_white {
                        break;
                    }
                    else {
                        found_white = true;
                    }
                }
                else if diagonal_left_1 == EMPTY_CHECKER {
                    if found_white {
                        return true;
                    }
                }
            }
            found_white = false;
            //Down and right
            for i in 1..(cmp::min(72-initial_position_bytes[0], initial_position_bytes[1]-49)+1) {
                diagonal_right_1 = &game.dark_squares[&format!("{}{}", (initial_position_bytes[0]+i) as char, (initial_position_bytes[1]-i) as char)];
                if (diagonal_right_1 == BLACK_CHECKER) || (diagonal_right_1 == BLACK_KING) {
                    break;
                }
                if (diagonal_right_1 == WHITE_CHECKER) || (diagonal_right_1 == WHITE_KING) {
                    if found_white {
                        break;
                    }
                    else {
                        found_white = true;
                    }
                }
                else if diagonal_right_1 == EMPTY_CHECKER {
                    if found_white {
                        return true;
                    }
                }
            }

            return false;
        }
    }
    else {
        if !is_king {
            if initial_position_bytes[0] >= 67 && initial_position_bytes[1] >= 51 {
                diagonal_left_1 = &game.dark_squares[&format!("{}{}", (initial_position_bytes[0]-1) as char, (initial_position_bytes[1]-1) as char)];
                diagonal_left_2 = &game.dark_squares[&format!("{}{}", (initial_position_bytes[0]-2) as char, (initial_position_bytes[1]-2) as char)];
                if (diagonal_left_1 == BLACK_CHECKER || diagonal_left_1 == BLACK_KING) && diagonal_left_2 == EMPTY_CHECKER { return true; }
            }
            if initial_position_bytes[0] <= 70 && initial_position_bytes[1] >= 51 {
                diagonal_right_1 = &game.dark_squares[&format!("{}{}", (initial_position_bytes[0]+1) as char, (initial_position_bytes[1]-1) as char)];
                diagonal_right_2 = &game.dark_squares[&format!("{}{}", (initial_position_bytes[0]+2) as char, (initial_position_bytes[1]-2) as char)];
                if (diagonal_right_1 == BLACK_CHECKER || diagonal_right_1 == BLACK_KING) && diagonal_right_2 == EMPTY_CHECKER { return true; }
            }
            else { return false;}
        }
        else {
            let mut found_black :bool = false;
            //Up and left
            for i in 1..(cmp::min(initial_position_bytes[0]-65, 56-initial_position_bytes[1])+1) {
                diagonal_left_1 = &game.dark_squares[&format!("{}{}", (initial_position_bytes[0]-i) as char, (initial_position_bytes[1]+i) as char)];
                if (diagonal_left_1 == BLACK_CHECKER) || (diagonal_left_1 == BLACK_KING) {
                    if found_black {
                        break;
                    }
                    else {
                        found_black = true;
                    }
                }
                if (diagonal_left_1 == WHITE_CHECKER) || (diagonal_left_1 == WHITE_KING) {
                    break;
                }
                else if diagonal_left_1 == EMPTY_CHECKER {
                    if found_black {
                        return true;
                    }
                }
            }
            found_black = false;
            //Up and right
            for i in 1..(cmp::min(72-initial_position_bytes[0], 56-initial_position_bytes[1])+1) {
                diagonal_right_1 = &game.dark_squares[&format!("{}{}", (initial_position_bytes[0]+i) as char, (initial_position_bytes[1]+i) as char)];
                if (diagonal_right_1 == BLACK_CHECKER) || (diagonal_right_1 == BLACK_KING) {
                    if found_black {
                        break;
                    }
                    else {
                        found_black = true;
                    }
                }
                if (diagonal_right_1 == WHITE_CHECKER) || (diagonal_right_1 == WHITE_KING) {
                    break;
                }
                else if diagonal_right_1 == EMPTY_CHECKER {
                    if found_black {
                        return true;
                    }
                }
            }
            found_black = false;
            //Down and left
            for i in 1..(cmp::min(initial_position_bytes[0]-65, initial_position_bytes[1]-49)+1) {
                diagonal_left_1 = &game.dark_squares[&format!("{}{}", (initial_position_bytes[0]-i) as char, (initial_position_bytes[1]-i) as char)];
                if (diagonal_left_1 == BLACK_CHECKER) || (diagonal_left_1 == BLACK_KING) {
                    if found_black {
                        break;
                    }
                    else {
                        found_black = true;
                    }
                }
                if (diagonal_left_1 == WHITE_CHECKER) || (diagonal_left_1 == WHITE_KING) {
                    break;
                }
                else if diagonal_left_1 == EMPTY_CHECKER {
                    if found_black {
                        return true;
                    }
                }
            }
            found_black = false;
            //Down and right
            for i in 1..(cmp::min(72-initial_position_bytes[0], initial_position_bytes[1]-49)+1) {
                diagonal_right_1 = &game.dark_squares[&format!("{}{}", (initial_position_bytes[0]+i) as char, (initial_position_bytes[1]-i) as char)];
                if (diagonal_right_1 == BLACK_CHECKER) || (diagonal_right_1 == BLACK_KING) {
                    if found_black {
                        break;
                    }
                    else {
                        found_black = true;
                    }
                }
                if (diagonal_right_1 == WHITE_CHECKER) || (diagonal_right_1 == WHITE_KING) {
                    break;
                }
                else if diagonal_right_1 == EMPTY_CHECKER {
                    if found_black {
                        return true;
                    }
                }
            }
            
            return false;
        }
    }

    false
}

//Check if the current position is valid and lays inside the board
fn check_valid_position (input: String) -> bool {
    let input_bytes = input.as_bytes();
    //Check if input has exactly 2 chars (x and y values)
    if input.len() != 2 {
        return false;
    }
    //Check if x value is outside A-H range
    if (input_bytes[0] < 65) || (input_bytes[0] > 72) {
        return false;
    }
    //Check if x value is outside 1-8 range
    if (input_bytes[1] < 49) || (input_bytes[1] > 56) {
        return false;
    }
    //Check if it is a dark square
    if (input_bytes[1] + input_bytes[0]) % 2 != 0 {
        return false;
    }
    true
}

fn make_cpu_movement(game: Game, black_turn: bool, checkers_capturing_position: &mut HashSet<String>) -> Game {

    game
}
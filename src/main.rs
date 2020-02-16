use std::io::{self, Write};
use std::collections::HashMap;
use std::collections::HashSet;
use std::process;
use std::cmp;

// Create constant values for the possible tiles
const EMPTY_CHECKER: &'static str = "     ";
const BLACK_CHECKER: &'static str = "  ○  ";
const WHITE_CHECKER: &'static str = "  ●  ";
const BLACK_KING: &'static str = "  ☖  ";
const WHITE_KING: &'static str = "  ☗  ";
// const BLACK_KING: &'static str = "  ♔  ";
// const WHITE_KING: &'static str = "  ♚  ";

// Declare Game structure to be used
#[derive(Clone)]
struct Game {
    amount_black_pieces: u8,
    amount_white_pieces: u8,
    dark_squares: HashMap<String, String>,
    mode: u8,
    black_turn: bool,
    player_1_black: bool,
}

// Implement a Game object and fill in the dark_squares with the checkers in their initial position
impl Game {
    fn new(mode: u8, player_1_black: bool) -> Game {
        let mut dark_squares = HashMap::new();
        for i in 1..9 {
            for j in 0..4 {
                let mut x_position_num = 65u8;
                x_position_num += if (i % 2) == 0 {2*j+1} else {(2*j)};
                let x_position_char = x_position_num as char;

                if i <= 3 {
                    dark_squares.insert(format!("{}{}", x_position_char, i), BLACK_CHECKER.to_string());
                }
                else if i <= 5 {
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
    let mut game;

    // Print menu for the game mode selection and get the user input
    print_menu();
    let choice_mode = get_choice();
    print!("{}[2J", 27 as char);

    // If applicable, print the menu for color selection and get the user input and then create the Game object
    if choice_mode == 1 {
        print_color_selection();
        let choice_color = get_choice();
        print!("{}[2J", 27 as char);
        game = Game::new(1, choice_color == 1);
    }
    else if choice_mode == 2 {
        game = Game::new(2, true);
    }
    else {
        process::exit(0);
    }

    // Create set to store checkers that can capture and enemy checker
    let mut checkers_capturing_position: HashSet<String>;
    checkers_capturing_position = HashSet::new();

    // Main game loop
    loop {
        // If one of the player is out of checkers, end the game
        if (game.amount_black_pieces == 0) || (game.amount_white_pieces == 0) {
            break;
        }

        // Clear the screem, print the amount of checkers for each side and print the board
        print!("{}[2J", 27 as char);
        println!("White: {}\tBlack: {}", game.amount_white_pieces, game.amount_black_pieces);
        print_screen(game.clone());

        // Check if there is any board able to capture an enemy
        check_mandatory_move(game.clone(), game.black_turn, &mut checkers_capturing_position);
        
        if game.mode == 1 {
            if game.player_1_black == game.black_turn {
                if checkers_capturing_position.is_empty() {
                    println!("There are no movement requirements");
                }
                else {
                    print!("You need to move one of the following checkers: ");
                    for value in checkers_capturing_position.clone() {
                        print!("{} ", value);
                    }
                    println!("");
                }
                
                print!("Enter your movement order: ");
                io::stdout().flush().expect("Error flushing stdout");
                let mut movement = get_move();
        
                while !validate_input(game.clone(), game.black_turn, movement.clone(), &checkers_capturing_position) {
                    print!("{}[2J", 27 as char);
                    println!("White: {}\tBlack: {}", game.amount_white_pieces, game.amount_black_pieces);
                    print_screen(game.clone());
                    check_mandatory_move(game.clone(), game.black_turn, &mut checkers_capturing_position);
                    print!("Not a valid movement. You need to move one of the following checkers: ");
                    for value in checkers_capturing_position.clone() {
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
                game = make_cpu_movement(game.clone(), game.black_turn, &mut checkers_capturing_position);
            }
        }

        if game.mode == 2 {
            if checkers_capturing_position.is_empty() {
                println!("There are no movement requirements");
            }
            else {
                print!("You need to move one of the following checkers: ");
                for value in checkers_capturing_position.clone() {
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
    
            while !validate_input(game.clone(), game.black_turn, movement.clone(), &checkers_capturing_position) {
                print!("{}[2J", 27 as char);
                println!("White: {}\tBlack: {}", game.amount_white_pieces, game.amount_black_pieces);
                print_screen(game.clone());
            
                check_mandatory_move(game.clone(), game.black_turn, &mut checkers_capturing_position);
                if checkers_capturing_position.is_empty() {
                    println!("Not a valid movement, please try again");
                }
                else {
                    print!("Not a valid movement. You need to move one of the following checkers: ");
                    for value in checkers_capturing_position.clone() {
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

// Print the board
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

// Print the mode selection menu
fn print_menu() {
    println!("Welcome to Checkers");
    println!("Choose your option:");
    println!("\t1 - Single player mode");
    println!("\t2 - Multiplayer mode");
    println!("\t3 - Exit");
}

// Print the color selection menu
fn print_color_selection() {
    println!("Which color do you want to play?");
    println!("Choose your option:");
    println!("\t1 - Black");
    println!("\t2 - White");
}

// Get user menu option selection input
fn get_choice()  -> u8 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let choice: u8 = input.trim().parse::<u8>().unwrap();

    choice
}

// Get user move input
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
    let mut current_position :String;
    let mut current_position_bytes :&[u8];
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

            let x_diff :u8;
            let y_diff :u8 = next_position_bytes[1] - current_position_bytes[1];
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

            let x_diff :u8;
            let y_diff :u8 = current_position_bytes[1] - next_position_bytes[1];
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

// Apply the move to the Game
fn process_input(game: Game, mut input: String) -> Game {
    // Remove the line break
    input.pop();
    // and make every letter uppercase
    input = input.to_ascii_uppercase();

    // Break the input into individual locations
    let path: Vec<&str> = input.split(' ').collect();
    
    // Check the type of checker in the starting position
    let initial_position :String = (*path.get(0).unwrap()).to_string();
    let game_clone = game.clone();
    let initial_checker = &game_clone.dark_squares[&initial_position];

    // And call the respective specialized input processor
    if initial_checker == BLACK_CHECKER { return process_checker_input(game, path, true); }

    if initial_checker == WHITE_CHECKER { return process_checker_input(game, path, false); }

    if initial_checker == BLACK_KING { return process_king_input(game, path, true); }

    if initial_checker == WHITE_KING { return process_king_input(game, path, false); }
    
    game
}

// Apply the move to the Game for a moving checker
fn process_king_input(mut game :Game, path: Vec<&str>, is_black: bool) -> Game {
    // Set allies and opponents based on own color
    let ally_king :&str = if is_black {BLACK_KING} else {WHITE_KING};
    let opponent_checker :&str = if is_black {WHITE_CHECKER} else {BLACK_CHECKER};
    let opponent_king :&str = if is_black {WHITE_KING} else {BLACK_KING};

    // Declare position variables to be used throughout the path
    let mut current_position :String;// = path.get(0).unwrap().to_string();
    let mut current_position_bytes :&[u8];// = current_position.as_bytes();
    let mut next_position :String;// = path.get(0).unwrap().to_string();
    let mut next_position_bytes :&[u8];// = current_position.as_bytes();

    let mut new_x :u8;
    let mut new_y :u8;

    // Create an initial game clone to allow reading the values before the path was applied
    let game_clone = game.clone();

    // Iterate over the path
    for i in 1..path.len() {
        // Update current and next position
        current_position = (*path.get(i-1).unwrap()).to_string();
        current_position_bytes = current_position.as_bytes();
        next_position = (*path.get(i).unwrap()).to_string();
        next_position_bytes = next_position.as_bytes();

        // Fill in the starting and ending point with the new values
        game.dark_squares.insert(current_position.clone(), EMPTY_CHECKER.to_string());
        game.dark_squares.insert(next_position.clone(), ally_king.to_string());

        // Calculate the distance to be traversed
        let x_diff = cmp::max(current_position_bytes[0], next_position_bytes[0]) - cmp::min(current_position_bytes[0], next_position_bytes[0]);
        // and the direction
        let going_up :bool = next_position_bytes[1] > current_position_bytes[1];
        let going_left :bool = next_position_bytes[0] < current_position_bytes[0];


        // Set the initial position of the sub-path
        new_x = current_position_bytes[0];
        new_y = current_position_bytes[1];
        // and declare a variable for its contents
        let mut diagonal :&String;
        
        // Iterate over the sub-path
        for _i in 1..(x_diff+1) {
            // Calculate the new position based on direction and color
            new_y = if going_up {new_y + 1} else {new_y -1};
            new_x = if going_left {new_x - 1} else {new_x + 1};
            // and its content
            diagonal = &game_clone.dark_squares[&format!("{}{}", new_x as char, new_y as char)];
            
            // If an opponent is found, it is captured
            if (diagonal == opponent_checker) || (diagonal == opponent_king) {
                // Empty the captured square
                game.dark_squares.insert(format!("{}{}", new_x as char, new_y as char), EMPTY_CHECKER.to_string());

                // Subtract amount of opponent pieces by one
                if is_black {game.amount_white_pieces -= 1;} else {game.amount_black_pieces -= 1;}
                break;
            }
        }
    }

    game
}

// Apply the move to the Game for a moving checker
fn process_checker_input(mut game :Game, path: Vec<&str>, is_black: bool) -> Game {    
    // Set allies and enemies based on own color
    let ally_checker :&str = if is_black {BLACK_CHECKER} else {WHITE_CHECKER};
    let ally_king :&str = if is_black {BLACK_KING} else {WHITE_KING};

    // Declare position variables to be used throughout the path
    let mut current_position :String = path.get(0).unwrap().to_string();
    let mut current_position_bytes :&[u8];// = current_position.as_bytes();
    let mut next_position :String = path.get(0).unwrap().to_string();
    let mut next_position_bytes :&[u8] = current_position.as_bytes();

    // Declare direction variable to be used throughout the path
    let mut going_left :bool;

    // Declare variables to be used throughout the path
    let mut new_x :u8;
    let mut new_y :u8;
    
    // Iterate over the path
    for i in 1..path.len() {
        // Update current and next position
        current_position = (*path.get(i-1).unwrap()).to_string();
        current_position_bytes = current_position.as_bytes();
        next_position = (*path.get(i).unwrap()).to_string();
        next_position_bytes = next_position.as_bytes();

        // Fill in the starting and ending point with the new values
        game.dark_squares.insert(current_position.clone(), EMPTY_CHECKER.to_string());
        game.dark_squares.insert(next_position.clone(), ally_checker.to_string());

        // If moved 2 squares, it means it captured a piece
        if cmp::max(next_position_bytes[1], current_position_bytes[1]) - cmp::min(next_position_bytes[1], current_position_bytes[1]) == 2 {
            // Check if it went left or right
            going_left = if next_position_bytes[0] < current_position_bytes[0] {true} else {false};

            // Calculate the new position based on direction and color
            new_x = if going_left {current_position_bytes[0]-1} else {current_position_bytes[0]+1};
            new_y = if is_black {current_position_bytes[1]+1} else {current_position_bytes[1]-1};

            // Empty the captured square
            game.dark_squares.insert(format!("{}{}", new_x as char, new_y as char), EMPTY_CHECKER.to_string());
            
            // Subtract amount of opponent pieces by one
            if is_black {game.amount_white_pieces -= 1;} else {game.amount_black_pieces -= 1;}
        }

    }

    // Check if turned into a king
    if next_position_bytes[1] == 56 {
        game.dark_squares.insert(next_position.clone(), ally_king.to_string());
    }

    game
}

// Check if there is any possible capture to be made
fn check_mandatory_move(game: Game, black_turn: bool, checkers_capturing_position: &mut HashSet<String>) {
    // Clear the set to start filling it again
    checkers_capturing_position.clear();

    // Set allies and enemies based on own color
    let ally_checker :&str = if black_turn {BLACK_CHECKER} else {WHITE_CHECKER};
    let ally_king :&str = if black_turn {BLACK_KING} else {WHITE_KING};

    // Iterate over the dark squares looking for checkers that can be moved
    for (position, value) in game.clone().dark_squares {
        // When a checker that can be moved is found, check if it can capture an enemy
        if value == ally_checker {
            // And add it to the set if so
            if check_can_capture(game.clone(), position.clone(), black_turn, false) { checkers_capturing_position.insert(position); }
        }
        else if value == ally_king {
            if check_can_capture(game.clone(), position.clone(), black_turn, true) { checkers_capturing_position.insert(position); }
        }
    }
}

// Check if the selected piece is able to capture an enemy in any position
fn check_can_capture(game: Game, initial_position: String, is_black: bool, is_king: bool) -> bool {

    // If is a king, call specialized function on the four possible directions
    if is_king {
        // Up and left
        if check_king_can_capture(game.clone(), initial_position.clone(), is_black, true, true) {return true;}
        // Up and right
        if check_king_can_capture(game.clone(), initial_position.clone(), is_black, false, true) {return true;}
        // Down and left
        if check_king_can_capture(game.clone(), initial_position.clone(), is_black, true, false) {return true;}
        // Down and right
        if check_king_can_capture(game.clone(), initial_position.clone(), is_black, false, false) {return true;}
        
        return false;
    }

    // If not, call specialized function on the two possible directions
    // Left
    if check_checker_can_capture(game.clone(), initial_position.clone(), is_black, true) {return true;}
    // Right
    if check_checker_can_capture(game.clone(), initial_position.clone(), is_black, false) {return true;}

    false
}

// Check if the selected king is able to capture an enemy in one of the for possible directions
fn check_king_can_capture(game: Game, initial_position: String, is_black: bool, going_left: bool, going_up :bool) -> bool {
    let initial_position_bytes = initial_position.as_bytes();
    let mut found_opponent :bool = false;

    // Set allies and enemies based on own color
    let ally_checker :&str = if is_black {BLACK_CHECKER} else {WHITE_CHECKER};
    let ally_king :&str = if is_black {BLACK_KING} else {WHITE_KING};
    let opponent_checker :&str = if is_black {WHITE_CHECKER} else {BLACK_CHECKER};
    let opponent_king :&str = if is_black {WHITE_KING} else {BLACK_KING};

    // Check how far it can go based on own initial position
    let range_x :u8 = if going_left {initial_position_bytes[0]-65} else {72-initial_position_bytes[0]};
    let range_y :u8 = if going_up {56-initial_position_bytes[1]} else {initial_position_bytes[1]-49};

    // Set traversing positions
    let mut new_x_position :u8 = initial_position_bytes[0];
    let mut new_y_position :u8 = initial_position_bytes[1];

    // Variable to store current iterating tile
    let mut diagonal;
    // Follow the selected diagonal until getting to a wall
    for _i in 1..(cmp::min(range_x, range_y)+1) {
        // Update position based on the selected direction
        new_x_position = if going_left {new_x_position - 1} else {new_x_position + 1};
        new_y_position = if going_up {new_y_position + 1} else {new_y_position - 1};

        // Fecth the tile at the current position
        diagonal = &game.dark_squares[&format!("{}{}", new_x_position as char, new_y_position as char)];
        // Cannot jump over an ally, so if couldn't capture an enemy yet, it won't anymore
        if (diagonal == ally_checker) || (diagonal == ally_king) {
            return false;
        }
        // If found an enemy
        if (diagonal == opponent_checker) || (diagonal == opponent_king) {
            // Cannot jump two adjacent enemues, so if couldn't capture an enemy yet, it won't anymore
            if found_opponent {
                return false;
            }
            // Mark as enemy found for future use
            else {
                found_opponent = true;
            }
        }
        // If found an empty square
        else if diagonal == EMPTY_CHECKER {
            // If already found an enemy, it means there is a landing spot to capture it
            if found_opponent {
                return true;
            }
        }
    }

    // Reached a wall without being able to capture an enemy
    false
}

// Check if the selected checker is able to capture an enemy in one of the for possible directions
fn check_checker_can_capture(game: Game, initial_position: String, is_black: bool, going_left: bool) -> bool {
    let initial_position_bytes = initial_position.as_bytes();

    // Set enemies based on own color
    let opponent_checker :&str = if is_black {WHITE_CHECKER} else {BLACK_CHECKER};
    let opponent_king :&str = if is_black {WHITE_KING} else {BLACK_KING};

    // Define possible positions base on selected direction
    let new_x_1 :u8 = if going_left {initial_position_bytes[0]-1} else {initial_position_bytes[0]+1};
    let new_x_2 :u8 = if going_left {initial_position_bytes[0]-2} else {initial_position_bytes[0]+2};
    let new_y_1 :u8 = if is_black {initial_position_bytes[1]+1} else {initial_position_bytes[1]-1};
    let new_y_2 :u8 = if is_black {initial_position_bytes[1]+2} else {initial_position_bytes[1]-2};

    // Check if current position allows to capture an enemy
    let meets_x_requirement :bool = if going_left {initial_position_bytes[0] >= 67} else {initial_position_bytes[0] <= 70};
    let meets_y_requirement :bool = if is_black {initial_position_bytes[1] <= 54} else {initial_position_bytes[1] >= 51};

    // If it is possible to capture an enemy, check if it does
    if meets_x_requirement && meets_y_requirement {
        let diagonal_1 = &game.dark_squares[&format!("{}{}", new_x_1 as char, new_y_1 as char)];
        let diagonal_2 = &game.dark_squares[&format!("{}{}", new_x_2 as char, new_y_2 as char)];
        if (diagonal_1 == opponent_checker || diagonal_1 == opponent_king) && diagonal_2 == EMPTY_CHECKER {return true;}
    }

    // If it didn't return false
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

// TODO implement
fn make_cpu_movement(game: Game, black_turn: bool, checkers_capturing_position: &mut HashSet<String>) -> Game {

    game
}
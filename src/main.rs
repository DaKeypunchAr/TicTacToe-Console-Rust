use std::io;
use std::io::Write;

fn print(input: &str)
{
    print!("{}", input);
    io::stdout().flush().expect("Failed to flush stdout");
}

fn main() {
    print("Welcome to our tictactoe project!\nWould you like to play?: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    input = input.to_lowercase().trim().to_string();

    match input.as_str() {
        "yes" | "y" => {
            print("Choose player (O or X): ");
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line!");

            input = input.to_lowercase().trim().to_string();

            match input.to_lowercase().trim() {
                "o" => start_game('O'),
                "x" => start_game('X'),
                _ => panic!("Invalid player! Sorry blud! 😔")
            }
        },
        "no" | "n" => println!("Hope that you will use me later! 😊"),
        _ => panic!("Invalid input! Sorry blud! 😔")
    }
    println!("Thanks for using!");
}

#[derive(Copy, Clone, PartialEq)]
enum Position {
    NONE,
    O,
    X
}

#[derive(PartialEq)]
enum GameState {
    PLAY,
    OWin,
    XWin,
    TIE
}

fn start_game(player: char) {
    let mut board = [Position::NONE; 9];
    let mut game_state = GameState::PLAY;
    let mut input = String::new();
    let mut player_turn = player; // that player is first

    while game_state == GameState::PLAY {
        print_board(&board);

        print("Give a valid position index: ");

        // Taking input
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line!");

        let position : u8;
        match input.trim().parse::<u8>() {
            Ok(number) => position = number,
            Err(e) => panic!("Invalid position! {}", e)
        }

        if position > 8 {
            panic!("Invalid position! It's more than 9, i.e., {}", position);
        }
        if board[position as usize] != Position::NONE {
            panic!("Invalid position! Already placed!");
        }

        board[position as usize] = if player_turn == 'O' { Position::O } else { Position::X };
        player_turn = if player_turn == 'O' { 'X' } else { 'O' };

        update_state(&board, &mut game_state);
    }

    match game_state {
        GameState::OWin => println!("O Won and X Lost!"),
        GameState::XWin => println!("X Won and O Lost!"),
        GameState::TIE => println!("It was a tie between O and X"),
        GameState::PLAY => unreachable!()
    }
}

fn update_state(board : &[Position; 9], game_state: &mut GameState)
{
    const WIN_POSITIONS : [(u8, u8, u8); 8] = [
        (0, 1, 2), (3, 4, 5), (6, 7, 8),
        (0, 3, 6), (1, 4, 7), (2, 5, 8),
        (0, 4, 8), (2, 4, 6)
    ];

    // Check for Win
    for &(p1, p2, p3) in &WIN_POSITIONS {
        if board[p1 as usize] != Position::NONE && board[p1 as usize] == board[p2 as usize] && board[p1 as usize] == board[p3 as usize] {
            *game_state = if board[p1 as usize] == Position::O { GameState::OWin } else { GameState::XWin };
            return;
        }
    }

    // Check for Tie
    for i in 0..9 {
        if board[i] == Position::NONE {
            return;
        }
    }
    *game_state = GameState::TIE;
}

fn print_board(board: &[Position; 9]) {
    for i in 0..9 {
        print!("{}", match board[i] { Position::NONE => i.to_string(), Position::O => 'O'.to_string(), Position::X => 'X'.to_string()});
        if i % 3 == 2 {
            println!();
        }
        else {
            print!(" | ");
        }
    }
    io::stdout().flush().expect("Failed to flush stdout!");
}

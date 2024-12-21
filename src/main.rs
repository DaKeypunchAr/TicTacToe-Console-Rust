use std::io;
use std::io::Write;

fn print(input: &str) {
    print!("{}", input);
    io::stdout().flush().expect("Failed to flush stdout");
}

const NUM_TRIES: u8 = 5;

fn main() {
    print("Welcome to our tictactoe project!\n(You can exit anytime with q if you are confused!)\nWould you like to play? (y/n): ");

    let mut input = String::new();
    let mut num_of_tries = NUM_TRIES;
    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        input.make_ascii_lowercase();

        match input.trim() {
            "q" => break,
            "yes" | "y" => {
                num_of_tries = NUM_TRIES;
                if !start_game() {
                    break;
                }
            }
            "no" | "n" => {
                println!("Hope that you will use me later! 😊");
                return;
            }
            _ => {
                num_of_tries -= 1;
                if num_of_tries == 0 {
                    println!("You lost all your tries! Bye!");
                    return;
                }
                print("Give a valid input! {num_of_tries} tries left! (y/n): ");
                continue;
            }
        }

        print("Would you like to play again? (y/n): ");
    }
    println!("Thanks for using! You are free to use me again!");
}

const GAME_RUNNING: u8 = 0b0000_0000;
const GAME_WIN: u8 = 0b0000_0001;
const GAME_TIE: u8 = 0b0000_0010;

fn start_game() -> bool {
    let mut board = [None; 9];
    let mut game_state = GAME_RUNNING;
    let mut o_turn: bool;

    print("Choose player (O or X): ");

    let mut input = String::new();

    let mut num_of_tries = NUM_TRIES;
    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        input.make_ascii_lowercase();

        match input.trim() {
            "q" => return false,
            "o" => {
                o_turn = false;
            }
            "x" => o_turn = true,
            _ => {
                num_of_tries -= 1;
                if num_of_tries == 0 {
                    println!("You lost all your tries! Bye!");
                    return false;
                }
                print!("Choose a valid player! {num_of_tries} tries left!");
                print(" (O or X): ");
                continue;
            }
        }

        break;
    }

    while game_state == GAME_RUNNING {
        o_turn = !o_turn;
        print_board(&board);
        println!("\n\nturn: {}", if o_turn { 'O' } else { 'X' });
        print("Give a valid position index: ");

        let mut position: u8;

        let mut num_of_tries = NUM_TRIES;
        loop {
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line!");

            input.make_ascii_lowercase();
            if input.trim() == "q" {
                return false;
            }

            match input.trim().parse::<u8>() {
                Ok(number) => position = number,
                Err(_) => {
                    num_of_tries -= 1;
                    print!("Give a valid position between 0..8! {num_of_tries} ");
                    print("tries left! : ");
                    if num_of_tries == 0 {
                        return false;
                    }
                    continue;
                }
            }

            if position > 8 {
                num_of_tries -= 1;
                println!("Invalid position! It's more than 9, i.e., {}", position);
                print!("Give a valid input between 0..8! {num_of_tries} ");
                print("tries left!: ");
                if num_of_tries == 0 {
                    return false;
                }
                continue;
            }
            if board[position as usize].is_some() {
                num_of_tries -= 1;
                println!("Invalid position! Already placed!");
                print!("Give a valid input between 0..8! {num_of_tries} ");
                print("tries left!: ");
                if num_of_tries == 0 {
                    return false;
                }
                continue;
            }

            break;
        }

        board[position as usize] = Some(o_turn);

        game_state = update_state(&board);
    }

    println!(
        "{}",
        match game_state {
            GAME_WIN => {
                if o_turn {
                    "O Won and X Lost!"
                } else {
                    "X Won and O Lost!"
                }
            }
            GAME_TIE => "It was a tie between O and X",
            _ => unreachable!(),
        }
    );

    true
}

fn update_state(board: &[Option<bool>; 9]) -> u8 {
    const WIN_POSITIONS: [(u8, u8, u8); 8] = [
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),
        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8),
        (0, 4, 8),
        (2, 4, 6),
    ];

    // Check for Win
    for &tuple in &WIN_POSITIONS {
        let tuple = (tuple.0 as usize, tuple.1 as usize, tuple.2 as usize);
        if board[tuple.0].is_some()
            && board[tuple.0] == board[tuple.1]
            && board[tuple.0] == board[tuple.2]
        {
            return GAME_WIN;
        }
    }

    // Check for Tie
    for pos in board {
        if pos.is_none() {
            return GAME_RUNNING;
        }
    }

    GAME_TIE
}

fn print_board(board: &[Option<bool>; 9]) {
    for i in 0u8..9u8 {
        print!(
            "{}",
            match board[i as usize] {
                None => (i + b'0') as char,
                Some(true) => 'O',
                Some(false) => 'X',
            }
        );
        if i % 3 == 2 {
            if i != 8 {
                println!("\n--+---+--");
            }
        } else {
            print!(" | ");
        }
    }
    io::stdout().flush().expect("Failed to flush stdout!");
}

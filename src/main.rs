use json;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::{thread::sleep, time::Duration};

mod board_state;
use board_state::BoardState;

#[derive(Debug)]
enum Transition<'a> {
    AppInit,
    GameInit,
    GameStart(&'a io::Stdin),
    TurnInit,
    TurnStart(&'a io::Stdin),
    Throw,
    Sleep(u64),
}

#[derive(Debug)]
enum State {
    AppIniting,
    GameIniting,
    GameStarting,
    TurnIniting,
    TurnStarting,
}

enum PieceType {
    Empty = 0,
    Player1LPiece = 1,
    Player2LPiece = 2,
    NeutralPiece = 4,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> fmt::Display for Transition<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> Transition<'a> {
    fn parse(line: &str, stdin: &'a io::Stdin) -> Transition<'a> {
        use Transition::*;
        match line {
            "game-init" => GameInit,
            "game-start" => GameStart(stdin),
            "turn-init" => TurnInit,
            "turn-start" => TurnStart(stdin),
            "throw" => Throw,
            _ => {
                // TODO: make this more idiomatic, if possible
                if line.starts_with("sleep") {
                    let mut split = line.split(" ");
                    split.next();
                    let seconds = split.next().unwrap().parse::<u64>().unwrap();
                    Sleep(seconds)
                } else {
                    panic!("cannot parse Transition from line \"{}\"", line)
                }
            }
        }
    }
}

fn parse_game_state_json(stdin: &io::Stdin) -> json::JsonValue {
    let mut json_game_state = String::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        json_game_state += &line;
        // TODO: find better way to see if json input has ended...
        if line.ends_with("}") {
            break;
        }
    }

    match json::parse(&json_game_state) {
        Ok(game_state) => game_state,
        Err(err) => panic!("could not parse json: {}", err),
    }
}

impl State {
    fn next(self, transition: &Transition, output: &mut impl Write) -> Result<State, io::Error> {
        use State::*;
        use Transition::*;

        match (&self, transition) {
            (AppIniting, AppInit) => {
                writeln!(output, "bot-start")?;
                Ok(GameIniting)
            }

            (GameIniting, GameInit) => Ok(GameStarting),

            (GameStarting, GameStart(stdin)) => {
                // TODO: do something with board state?
                let _game_state = parse_game_state_json(stdin);

                Ok(TurnIniting)
            },

            (TurnIniting, TurnInit) => Ok(TurnStarting),
            (TurnIniting, Sleep(seconds)) => { sleep(Duration::from_secs(*seconds)); Ok(TurnIniting) },

            (TurnStarting, TurnStart(stdin)) => {
                let game_state = parse_game_state_json(stdin);

                // TODO: check if this assumption is right (game_state["player"] + 1 == current_player )
                let current_player = game_state["player"].as_u8().unwrap() + 1;

                let board_game_state = BoardState::load(game_state);

                let optimal_move = board_game_state.calculate_optimal_move(current_player);

                
                // TODO: write optimal move!
                let place_pieces_command_json = json::object! {
                    "playerLPieceCoordinates": optimal_move.lPiece,
                    "neutralPieceCoordinates": optimal_move.neutralPieces
                };
                writeln!(output, "{}", place_pieces_command_json)?;

                writeln!(output, "turn-end")?;
                Ok(TurnIniting)
            },
            (TurnStarting, Sleep(seconds)) => { sleep(Duration::from_secs(*seconds)); Ok(TurnIniting) }

            (_, Throw) => panic!("on demand!"),

            _ => panic!("didn't expect transition {} in state {}!", transition, &self),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = &mut io::stdout();
    let mut state = State::AppIniting;

    state = state.next(&Transition::AppInit, stdout).unwrap();

    loop {
        let mut line = String::new();
        if let Ok(_) = stdin.read_line(&mut line) {
            let transition = Transition::parse(&line.trim_end(), &stdin);
            state = state.next(&transition, stdout).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn app_initing_cannot_sleep() {
        State::AppIniting.next(&Transition::Sleep(0), &mut Vec::new()).unwrap();
    }

    #[test]
    #[should_panic]
    fn game_initing_cannot_sleep() {
        State::GameIniting.next(&Transition::Sleep(0), &mut Vec::new()).unwrap();
    }

    #[test]
    #[should_panic]
    fn game_starting_cannot_sleep() {
        State::GameStarting.next(&Transition::Sleep(0), &mut Vec::new()).unwrap();
    }

    #[test]
    fn app_initing_app_init_write_bot_start() { 
        let buffer = &mut Vec::new();
        
        State::AppIniting.next(&Transition::AppInit, buffer).unwrap();

        assert_eq!(buffer, "bot-start\n".as_bytes());
    }
}
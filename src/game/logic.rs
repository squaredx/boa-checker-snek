use log::info;
use serde_json::{json, Value};

use crate::{Battlesnake, Board, Game, Coord};
use crate::game::movement::{Movement, WeightedMovementSet};

pub struct PotentialMoves {
    position: Coord,
    movement: Movement,
}

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn handle_info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "squaredx", // TODO: Your Battlesnake Username
        "color": "#0099cc", // TODO: Choose color
        "head": "silly", // TODO: Choose head
        "tail": "nr-booster", // TODO: Choose tail
    });
}

// start is called when your Battlesnake begins a game
pub fn handle_start(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn handle_end(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn handle_move(_game: &Game, turn: &u32, _board: &Board, you: &Battlesnake) -> Value {
    let mut moves = WeightedMovementSet::new();
    //TODO: Optimize by having a vector of potential move so that we can share in the avoid functions

    let board_width = &_board.width;
    let board_height = &_board.height;

    avoid_out_of_bounds(board_width, board_height, you, &mut moves);
    avoid_myself(&you, &mut moves);
    //avoid_other_snakes(&you, &mut moves);
    //avoid_small_spaces
    //if health < 50
    //find_closest_food


    let chosen = moves.pick_movement().unwrap_or(&Movement::Up);
    info!("MOVE {}: {}", turn, chosen.to_string());
    return json!({ "move": chosen.to_string() });
}

fn avoid_out_of_bounds(width: &i32, height: &i32, snake: &Battlesnake, set: &mut WeightedMovementSet) {
    let my_head = &snake.body[0];
    let potential_moves = make_potential_moves(my_head);

    for potential_move in potential_moves {
        if potential_move.position.x < 0 || potential_move.position.x >= *width || potential_move.position.y < 0 || potential_move.position.y >= *height {
            println!("OOB - Removing {}", potential_move.movement.to_string());
            set.remove(&potential_move.movement);
        }
    }
}

fn avoid_myself(snake: &Battlesnake, set: &mut WeightedMovementSet) {
    let my_head = &snake.body[0];
    let my_tail = get_tail(snake);
    let potential_moves = make_potential_moves(my_head);
    let stacked = is_stacked(snake);

    for potential_move in potential_moves {
        if snake.body.contains(&potential_move.position) && !(potential_move.position == *my_tail && !stacked) {
            println!("MYSELF - Removing {}", potential_move.movement.to_string());
            set.remove(&potential_move.movement);
        }
    }
}

fn make_potential_moves(head_coord: &Coord) -> Vec<PotentialMoves> {
    vec![
        PotentialMoves {
            position: Coord { x: head_coord.x, y: head_coord.y + 1 },
            movement: Movement::Up,
        },
        PotentialMoves {
            position: Coord { x: head_coord.x, y: head_coord.y - 1 },
            movement: Movement::Down,
        },
        PotentialMoves {
            position: Coord { x: head_coord.x - 1, y: head_coord.y },
            movement: Movement::Left,
        },
        PotentialMoves {
            position: Coord { x: head_coord.x + 1, y: head_coord.y },
            movement: Movement::Right,
        },
    ]
}

fn is_stacked(snake: &Battlesnake) -> bool {
    let body = &snake.body;
    let mut prev_coord = None;

    for coord in body {
        if let Some(prev) = prev_coord {
            if coord == prev {
                return true;
            }
        }
        prev_coord = Some(coord);
    }
    false
}

fn get_tail(snake: &Battlesnake) -> &Coord {
    snake.body.last().unwrap()
}
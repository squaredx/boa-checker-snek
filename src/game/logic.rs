use log::info;
use serde_json::{json, Value};
use pathfinding::prelude::astar;

use crate::{Battlesnake, Board, Game, Coord};
use crate::game::movement::{Movement, WeightedMovementSet};

static FOOD_GREEDY_FACTOR: f32 = 0.5;

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
    let mut moves = WeightedMovementSet::new(&you.body[0]);

    let board_width = &_board.width;
    let board_height = &_board.height;

    avoid_out_of_bounds(board_width, board_height,  &mut moves);
    avoid_myself(&you, &mut moves);
    avoid_baddies(&_board.snakes, &mut moves);
    //avoid_small_spaces

    if you.health <= 50 {
        let closest_food = find_closest_food(&you.body[0], &_board);
        if let Some(food) = closest_food {
            let path_to_food = find_path_to_food(&you, &_board, &you.body[0], &food);

            if let Some(path) = path_to_food {
                let next_coord = path.0.get(1).cloned().unwrap();
                let movement = convert_coord_to_movement(&you.body[0], &next_coord);
                println!("FOOD - Health: {}, Distance: {}, Movement: {}", you.health, path.1, movement.to_string());

                let updated_probability = moves.get_probability(&movement) * ( 1.0 + FOOD_GREEDY_FACTOR);

                moves.update_probability(&movement, updated_probability);
                
                for future_move in moves.moves.clone() {
                    if future_move.movement != movement {
                        moves.update_probability(&future_move.movement,  moves.get_probability(&movement) * FOOD_GREEDY_FACTOR);
                    }
                }
            }
        }
    }

    println!("MOVES: {}", moves.to_string());

    let chosen = moves.pick_movement().unwrap_or(&Movement::Up);
    info!("MOVE {}: {}", turn, chosen.to_string());
    return json!({ "move": chosen.to_string() });
}

fn avoid_out_of_bounds(width: &i32, height: &i32, set: &mut WeightedMovementSet) {
    for future_move in set.moves.clone() {
        if future_move.position.x < 0 || future_move.position.x >= *width || future_move.position.y < 0 || future_move.position.y >= *height {
            println!("OOB - Removing {}", future_move.movement.to_string());
            set.remove(&future_move.movement);
        }
    }
}

fn avoid_myself(snake: &Battlesnake, set: &mut WeightedMovementSet) {
    let my_tail = get_tail(snake);
    let stacked = is_stacked(snake);

    for future_moves in set.moves.clone() {
        if snake.body.contains(&future_moves.position) && !(future_moves.position == *my_tail && !stacked) {
            println!("MYSELF - Removing {}", future_moves.movement.to_string());
            set.remove(&future_moves.movement);
        }
    }
}

fn avoid_baddies(baddies: &Vec<Battlesnake>, set: &mut WeightedMovementSet) {
    for baddy in baddies {
        let baddy_head = &baddy.body[0];
        let baddy_tail = get_tail(baddy);
        let baddy_stacked = is_stacked(baddy);

        for future_move in set.moves.clone() {
            let distance = manhattan_distance(&future_move.position, baddy_head);
            println!("BADDY - Manhattan distance is {}", distance);
            if distance <= 2 {
                let previous_probability = set.get_probability(&future_move.movement);
                let updated_probability = previous_probability * (1.0 - (distance as f32 / 2.0));
                println!("BADDY - Reducing {} probability from {} to {}", future_move.movement.to_string(), updated_probability, updated_probability);
                set.update_probability(&future_move.movement, updated_probability);
            } else if baddy.body.contains(&future_move.position) && !(future_move.position == *baddy_tail && !baddy_stacked) {
                println!("BADDY - Removing {}", future_move.movement.to_string());
                set.remove(&future_move.movement);
            }
        }
    }
}

fn find_path_to_food(snake: &Battlesnake, board: &Board, start: &Coord, goal: &Coord) -> Option<(Vec<Coord>, i32)> {
    astar(
        start,
        |p| get_neighbors(snake, board, p),
        |p| manhattan_distance(p, goal),
        |p| *p == *goal,
    )
}

fn find_closest_food(head_coord: &Coord, board: &Board) -> Option<Coord> {
    let mut closest_food: Option<Coord> = None;
    let mut closest_distance = i32::MAX;

    for food in &board.food {
        let distance = manhattan_distance(head_coord, food);
        if distance < closest_distance {
            closest_distance = distance;
            closest_food = Some(food.clone());
        }
    }

    closest_food
}

fn get_neighbors(snake: &Battlesnake, board: &Board, coord: &Coord) -> Vec<(Coord, i32)> {
    let mut neighbors = Vec::new();
    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)]; // Up, Down, Left, Right

    for (dx, dy) in directions.iter() {
        let new_coord = Coord {
            x: coord.x + dx,
            y: coord.y + dy,
        };
        if astar_is_valid(snake, board, &new_coord) {
            neighbors.push((new_coord, 1)); // Each move costs 1
        }
    }

    neighbors
}

fn astar_is_valid(snake: &Battlesnake, board: &Board, coord: &Coord) -> bool {
    coord.x >= 0 && coord.x < board.width && 
    coord.y >= 0 && coord.y < board.height &&
    !board.snakes.iter().any(|s| s.body.contains(coord)) &&
    !snake.body.contains(coord)
}

fn manhattan_distance(a: &Coord, b: &Coord) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn convert_coord_to_movement(head_coord: &Coord, next_coord: &Coord) -> Movement {
    if head_coord.x < next_coord.x {
        return Movement::Right;
    } else if head_coord.x > next_coord.x {
        return Movement::Left;
    } else if head_coord.y < next_coord.y {
        return Movement::Up;
    } else if head_coord.y > next_coord.y {
        return Movement::Down;
    } else {
        return Movement::Up;
    }
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
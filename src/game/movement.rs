use std::cmp::Ordering;
use rand::Rng;

use crate::Coord;

#[derive(PartialEq, Clone)]
pub enum Movement {
    Right,
    Left,
    Up,
    Down,
}

impl std::fmt::Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Movement::Right => write!(f, "right"),
            Movement::Left => write!(f, "left"),
            Movement::Up => write!(f, "up"),
            Movement::Down => write!(f, "down"),
        }
    }
}

#[derive(Clone)]
pub struct WeightedMovement {
    pub movement: Movement,
    pub position: Coord,
    pub probability: f32,
}


pub struct WeightedMovementSet {
    pub moves: Vec<WeightedMovement>,
}

impl WeightedMovementSet {
    pub fn new(head_coord: &Coord) -> WeightedMovementSet {
        WeightedMovementSet {
            moves: vec![
                WeightedMovement {
                    movement: Movement::Right,
                    position: Coord { x: head_coord.x + 1, y: head_coord.y },
                    probability: 1.0,
                },
                WeightedMovement {
                    movement: Movement::Left,
                    position: Coord { x: head_coord.x - 1, y: head_coord.y },
                    probability: 1.0,
                },
                WeightedMovement {
                    movement: Movement::Up,
                    position: Coord { x: head_coord.x, y: head_coord.y + 1 },
                    probability: 1.0,
                },
                WeightedMovement {
                    movement: Movement::Down,
                    position: Coord { x: head_coord.x, y: head_coord.y - 1 },
                    probability: 1.0,
                },
            ],
        }
    }

    pub fn remove(&mut self, movement: &Movement) {
        self.moves.retain(|m| m.movement != *movement);
    }

    pub fn update_probability(&mut self, movement: &Movement, probability: f32) {
        self.moves
            .iter_mut()
            .find(|wm| wm.movement == *movement)
            .map(|wm| wm.probability = probability);
    }

    pub fn get_probability(&self, movement: &Movement) -> f32 {
        self.moves
            .iter()
            .find(|wm| wm.movement == *movement)
            .map(|wm| wm.probability)
            .unwrap_or(0.0)
    }


    pub fn pick_movement(&self) -> Option<&Movement> {
        let mut rng = rand::thread_rng();

        let max_probability = self
            .moves
            .iter()
            .map(|wm| wm.probability)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

        if let Some(max_probability) = max_probability {
            let candidates: Vec<&WeightedMovement> = self
                .moves
                .iter()
                .filter(|wm| wm.probability == max_probability)
                .collect();

            if !candidates.is_empty() {
                let random_index = rng.gen_range(0..candidates.len());
                return Some(&candidates[random_index].movement);
            }
        }
        None
    }
}

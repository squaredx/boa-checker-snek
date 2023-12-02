use std::cmp::Ordering;
use rand::Rng;

#[derive(PartialEq)]
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

pub struct WeightedMovement {
    pub movement: Movement,
    pub probability: f32,
}

pub struct WeightedMovementSet {
    pub moves: Vec<WeightedMovement>,
}

impl WeightedMovementSet {
    pub fn new() -> WeightedMovementSet {
        WeightedMovementSet {
            moves: vec![
                WeightedMovement {
                    movement: Movement::Right,
                    probability: 1.0,
                },
                WeightedMovement {
                    movement: Movement::Left,
                    probability: 1.0,
                },
                WeightedMovement {
                    movement: Movement::Up,
                    probability: 1.0,
                },
                WeightedMovement {
                    movement: Movement::Down,
                    probability: 1.0,
                },
            ],
        }
    }

    pub fn remove(&mut self, movement: &Movement) {
        self.moves.retain(|m| m.movement != *movement);
    }

    pub fn update_probability(&mut self, movement: Movement, probability: f32) {
        self.remove(&movement);
        self.moves.push(WeightedMovement {
            movement,
            probability,
        });
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

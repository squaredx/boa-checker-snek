use std::cmp::Ordering;

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
        self.moves
            .iter()
            .max_by(|a, b| {
                a.probability
                    .partial_cmp(&b.probability)
                    .unwrap_or(Ordering::Equal)
            })
            .map(|weighted_movement| &weighted_movement.movement)
    }
}

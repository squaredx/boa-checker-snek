pub enum Movement {
    Right,
    Left,
    Up,
    Down,
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

    pub fn pick_movement(&self) -> Movement {
        pub fn pick_movement(&self) -> Option<&Movement> {
            self.moves
                .iter()
                .max_by(|a, b| a.probability.partial_cmp(&b.probability).unwrap())
                .map(|wm| &wm.movement)
        }
    }
}

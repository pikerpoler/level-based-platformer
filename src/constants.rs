pub const TILE_SIZE: i32 = 16;
pub const GAMEPAD_SENSITIVITY_THRESHOLD: f32 = 0.5;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum IntGridValues {
    // Empty = 0,
    Dirt = 1,
    Ladder = 2,
    Stone = 3,
    Goal = 4,
    SpawnPoint = 5,
}

impl From<i32> for IntGridValues {
    fn from(value: i32) -> Self {
        match value {
            1 => IntGridValues::Dirt,
            2 => IntGridValues::Ladder,
            3 => IntGridValues::Stone,
            4 => IntGridValues::Goal,
            5 => IntGridValues::SpawnPoint,
            _ => IntGridValues::Dirt,
        }
    }
}

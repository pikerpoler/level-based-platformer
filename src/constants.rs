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

pub mod sprites {
    pub struct FrameRange {
        pub first: usize,
        pub last: usize,
    }

    pub mod fox {
        use super::FrameRange;
        pub const IDLE_FRAMES: FrameRange = FrameRange { first: 0, last: 3 };
        pub const WALK_FRAMES: FrameRange = FrameRange { first: 6, last: 11 };
        pub const CLIMB_FRAMES: FrameRange = FrameRange {
            first: 12,
            last: 15,
        };
        pub const CLIMB_FRAMES_IDLE: FrameRange = FrameRange {
            first: 12,
            last: 12,
        };
        pub const _DEATH_FRAMES: FrameRange = FrameRange {
            first: 24,
            last: 25,
        };
        pub const JUMP_UP_FRAMES: FrameRange = FrameRange {
            first: 30,
            last: 30,
        };
        pub const JUMP_DOWN_FRAMES: FrameRange = FrameRange {
            first: 31,
            last: 31,
        };
    }
}

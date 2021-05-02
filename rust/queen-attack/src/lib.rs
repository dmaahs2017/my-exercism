#[derive(Debug)]
pub struct ChessPosition {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Self { x: rank, y: file }),
            _ => None,
        }
    }

    pub fn slope(&self, other: &Self) -> f32 {
        (other.y - self.y) as f32 / (other.x - self.x) as f32
    }
}

impl Queen {
    pub fn new(pos: ChessPosition) -> Self {
        Self { pos }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let slope = self.pos.slope(&other.pos);
        slope == 1. || slope == -1. || slope == 0. || slope.is_infinite()
    }
}

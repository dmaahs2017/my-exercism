//! I am quite happy with how this solution turned out. I even got to try out some dynamic
//! dispatch. Though I realize I could've more simply just made a function that did the same thing
//! as the [Instructions](Instructions) trait

/// Converts a string of encoded instructions into an iterator of encoded instructions
pub trait Instructions: AsRef<str> {
    // Because I wanted to try out dynamic dispatch and didn't want to return a vec
    fn decode_instructions(&self) -> Box<dyn Iterator<Item = Instruction> + '_> {
            let iter = self.as_ref()
            .chars()
            .map(|c| match c {
                'L' => Instruction::TurnLeft,
                'R' => Instruction::TurnRight,
                'A' => Instruction::Advance,
                _ => panic!("Invalid instructions"),
            });

            Box::new(iter)
    }
}
impl<T: AsRef<str>> Instructions for T {}

/// Represents the possible instructions a robot can do
pub enum Instruction {
    TurnLeft,
    TurnRight,
    Advance,
}

/// Represents the directions the robot can face
#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {

    /// Get the relative direction to the left of the current dir.
    fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West =>  Direction::South,
        }
    }

    /// Get the relative direction to the right of the current dir.
    fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West =>  Direction::North,
        }
    }
}

// Just to group related data
/// X and Y coordinates
struct Vec2 {
    x: i32,
    y: i32,
}

pub struct Robot {
    pos: Vec2,
    dir: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            pos: Vec2 { x, y },
            dir: d,
        }
    }

    /// Robot turn right
    pub fn turn_right(self) -> Self {
        Self {
            pos: self.pos,
            dir: self.dir.right(),
        }
    }

    /// Robot turn left
    pub fn turn_left(self) -> Self {
        Self {
            pos: self.pos,
            dir: self.dir.left(),
        }
    }

    /// Advance the robot 1 position in the direction it is facing
    pub fn advance(self) -> Self {
        let mut pos = self.pos;
        match self.dir {
            Direction::North => pos.y += 1,
            Direction::East => pos.x += 1,
            Direction::South => pos.y -= 1,
            Direction::West => pos.x -= 1,
        };
        
        Self {
            pos,
            dir: self.dir
        }
    }

    // Changed the api to AsRef<str> because it's a more ergonomic
    /// Apply a string of instructions to the robot.
    /// Instructions are "LRA", turn left, right and advance respectively.
    pub fn instructions(self, instructions: impl AsRef<str>) -> Self {
        instructions.decode_instructions().fold(self, |acc, ins| {
            match ins {
                Instruction::TurnRight => acc.turn_right(),
                Instruction::TurnLeft => acc.turn_left(),
                Instruction::Advance => acc.advance(),
            }
        })
    }

    /// Get the position (x, y) coordinates of the robot
    pub fn position(&self) -> (i32, i32) {
        (self.pos.x, self.pos.y)
    }

    /// Get the current direction the robot is facing
    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
struct Frame {
    first: u16,
    second: Option<u16>,
}

impl Frame {
    fn complete(&self) -> bool {
        self.strike() || self.second.is_some()
    }

    fn strike(&self) -> bool {
        self.first == 10
    }

    fn spare(&self) -> bool {
        self.first + self.second.unwrap_or_default() == 10
    }

    fn score_spare(&self, next: u16) -> u16 {
        10 + next
    }

    fn score_strike(&self, first: u16, second: u16) -> u16 {
        10 + first + second
    }

    fn score_open(&self) -> u16 {
        self.first + self.second.unwrap_or_default()
    }
}

pub struct BowlingGame {
    scorecard: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            scorecard: Default::default(),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.complete() {
            return Err(Error::GameComplete);
        }

        if let Some(frame) = self.scorecard.last() {
            if frame.complete() {
                self.roll_new_frame(pins)?;
            } else {
                self.roll_incomplete_frame(pins)?;
            }
        } else {
            self.roll_new_frame(pins)?;
        }

        Ok(())
    }

    fn roll_new_frame(&mut self, pins: u16) -> Result<(), Error> {
        match pins {
            (0..=10) => self.scorecard.push(Frame {
                first: pins,
                second: None,
            }),
            _ => return Err(Error::NotEnoughPinsLeft),
        };

        Ok(())
    }

    fn roll_incomplete_frame(&mut self, pins: u16) -> Result<(), Error> {
        let frame = self
            .scorecard
            .last_mut()
            .expect("This function requires an incomplete frame in the scorecard");

        if pins <= 10 - frame.first {
            frame.second = Some(pins);
        } else {
            return Err(Error::NotEnoughPinsLeft);
        }
        Ok(())
    }

    pub fn complete(&self) -> bool {



        // short circuit evaluation will prevent scorecard[9] from ever indexing out of bounds
        self.scorecard.len() == 10 && self.scorecard[9].complete()
    }

    pub fn score(&self) -> Option<u16> {
        if !self.complete() {
            return None;
        }

        let mut score = 0;
        for i in 0..10 {
            let f = &self.scorecard[i];
            if f.spare() {
                let next = self.scorecard.get(i + 1).map_or(0, |f| f.first);
                score += f.score_spare(next);
            } else {
                score += f.score_open()
            }

        }

        Some(score)
    }
}

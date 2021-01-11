use std::collections::HashSet;
#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a[u32],
    latest: Option<u32>,
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a[u32]) -> Self {
        HighScores {
            scores,
            latest: None,
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.latest
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.len() > 0 {
            Some(self.scores[0])
        } else {
            None
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let scores: HashSet<u32> = self.scores.iter()
            .fold(HashSet::new(), |mut acc, &s| { acc.insert(s); acc });
        
        unimplemented!()
    }
}

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
pub struct Allergies {
    score: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, EnumIter)]
#[repr(u8)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score: score as u8 }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        (self.score >> *allergen as u8) % 2 == 1
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::iter()
            .filter(|a| self.is_allergic_to(a))
            .collect()
    }
}

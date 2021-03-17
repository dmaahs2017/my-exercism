pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: PartialEq + PartialOrd + Default + std::ops::Add<Output = T> + Copy,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        // check for 0 values
        if sides.iter().any(|&s| s == T::default()) {
            return None;
        }

        // check for 2 sides that aren't >= the remaining side
        if sides[0] + sides[1] < sides[2]
            || sides[0] + sides[2] < sides[1]
            || sides[1] + sides[2] < sides[0]
        {
            return None;
        }

        Some(Triangle { sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1]
            && self.sides[0] == self.sides[2]
            && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1]
            && self.sides[0] != self.sides[2]
            && self.sides[1] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        !self.is_equilateral() && !self.is_scalene()
    }
}

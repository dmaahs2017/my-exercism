use combination::combine;

pub fn count(lines: &[&str]) -> u32 {
    let r = RectCounter::from_ascii_strs(lines);
    r.count_rects()
}

struct RectCounter {
    verticies: Vec<(i32, i32)>,
}

impl RectCounter {
    fn from_ascii_strs(lines: &[&str]) -> Self {
        let mut verticies = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '+' {
                    verticies.push((i as i32, j as i32));
                }
            }
        }

        Self { verticies }
    }


    fn count_rects(&self) -> u32 {
        let combos = combine::from_vec_at(&self.verticies, 4);
        let mut rects = 0;
        for combo in combos {
            if is_rect(combo) {
                rects += 1;
            }
        }

        rects
    }
}

fn is_rect(mut verts: Vec<(i32, i32)>) -> bool {
    verts.sort();
    let top_a = verts[0];
    let top_b = verts[1];
    
    let bot_a = verts[2];
    let bot_b = verts[3];

    //let has_same_horiz_dist = top_b.1 - top_a.1 == bot_b.1 - bot_a.1;
    //let has_same_vert_dist = bot_a.0 - top_a.0 == bot_b.0 - top_b.0;
    let horiz_are_same = top_a.0 == top_b.0 && bot_a.0 == bot_b.0;
    let verts_are_same = top_a.1 == bot_a.1 && top_b.1 == bot_b.1;
    let diag_are_diff = top_a.0 != bot_b.0 && top_a.1 != bot_b.1
        && top_b.0 != bot_a.0 && top_b.1 != bot_a.1;
    //dbg!(has_same_vert_dist, has_same_horiz_dist);

    //has_same_vert_dist && has_same_horiz_dist && 
        horiz_are_same && verts_are_same
            && diag_are_diff
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_ascii_strs_works() {
        let lines = &["--+--", "+---+"];
        let r = RectCounter::from_ascii_strs(lines);
        assert_eq!(r.verticies, vec![(0,2), (1,0), (1, 4)]);



        #[rustfmt::skip]
        let lines = &[
            "  +-+",
            "  | |",
            "+-+-+",
            "| |  ",
            "+-+  ",
        ];
        let r = RectCounter::from_ascii_strs(lines);

        assert_eq!(r.verticies, vec!
            [
            (0, 2), (0, 4),
            (2,0), (2, 2), (2, 4),
            (4, 0), (4, 2)
            ])
    }
}

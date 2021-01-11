pub struct PascalsTriangle {
    triangle: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle = Vec::with_capacity(row_count as usize);

        if row_count > 0 {
            triangle.push(vec![1]);
        }

        if row_count > 1 {
            for i in 1..row_count {
                let prev_row = triangle
                    .get((i - 1) as usize)
                    .expect("There to always be a previous row");
                let mut cur_row = Vec::with_capacity((i + 1) as usize);
                for j in 0..=i {
                    if j == 0 || j == prev_row.len() as u32 {
                        cur_row.push(1);
                    } else {
                        cur_row.push(
                            prev_row.get((j - 1) as usize).expect("to have element")
                                + prev_row.get(j as usize).expect("to have element"),
                        );
                    }
                }
                triangle.push(cur_row);
            }
        }

        PascalsTriangle { triangle }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}


pub struct SudokuField {
    size_param: usize,
    edge_size: usize,
    num_cells: usize,
    fields: Vec<Option<u8>>,
}

impl SudokuField {
    pub fn new(size_param: usize) -> Result<SudokuField, String> {
        if size_param < 2 {
            return Err(format!("Invalid size {} provided (must at least be 2).", size_param));
        }
        let edge_size = size_param * size_param;
        let num_cells = edge_size * edge_size;
        let fields = vec![None; num_cells];
        Ok(SudokuField {
            size_param,
            edge_size,
            num_cells,
            fields,
        })
    }

    pub fn set_fields(&mut self, fields: Vec<u8>) {
        self.fields = fields.iter().map(|&x| {
            if x > 0 && x <= self.edge_size as u8 {
                Some(x)
            } else { None }
        }).collect();
    }

    fn at(&self, x: usize, y: usize) -> usize {
        y * self.edge_size + x
    }

    fn get_at(&self, x: usize, y: usize) -> Option<u8> {
        self.fields[self.at(x, y)]
    }

    pub fn pretty_format(&self) -> String {
        let num_chars = self.edge_size.to_string().len();

        let mut s = String::new();
        for row in 0..self.edge_size + 1 {
            if row == 0 {
                let block = String::from("─").repeat((num_chars + 1) * self.size_param + 1);
                s.push_str(format!("┌{}{}┐\n", (block.clone() + "┬").repeat(self.size_param - 1), &block).as_str());
            } else if row == self.edge_size {
                let block = String::from("─").repeat((num_chars + 1) * self.size_param + 1);
                s.push_str(format!("└{}{}┘\n", (block.clone() + "┴").repeat(self.size_param - 1), &block).as_str());
                break;
            } else if row % self.size_param == 0 {
                let block = String::from("─").repeat((num_chars + 1) * self.size_param + 1);
                s.push_str(format!("├{}{}┤\n", (block.clone() + "┼").repeat(self.size_param - 1), &block).as_str());
            }

            for col in 0..self.edge_size {
                if col % self.size_param == 0 {
                    s.push_str("| ");
                }

                let mut cell = match self.get_at(col, row) {
                    Some(x) => x.to_string(),
                    None => ".".to_string()
                };
                cell = " ".to_string().repeat(num_chars - cell.len()) + cell.as_str() + " ";
                s.push_str(cell.as_str())
            }
            s.push_str("|\n")
        }
        s
    }

    pub fn is_complete(&self) -> bool {
        for i in self.fields.iter() {
            if let None = *i {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_var_sizes() {
        for i in 2..11 {
            assert!(SudokuField::new(i).is_ok());
        }
    }

    #[test]
    fn new_invalid_size() {
        assert!(SudokuField::new(1).is_err());
        assert!(SudokuField::new(0).is_err());
    }

    #[test]
    fn pretty_format() {

        let mut s = SudokuField::new(2).unwrap();
        s.set_fields(vec![0, 1, 2, 3,
                          4, 5, 6, 7,
                          1, 2, 3, 4,
                          2, 3, 4, 5]);
        let exp = String::from("\
┌─────┬─────┐
| . 1 | 2 3 |
| 4 . | . . |
├─────┼─────┤
| 1 2 | 3 4 |
| 2 3 | 4 . |
└─────┴─────┘\n");
        assert_eq!(exp, s.pretty_format());

        let mut s = SudokuField::new(4).unwrap();
        s.set_fields(vec![ 0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
                           1,  2, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                           2,  1,  4,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
                           3, 17, 18,  6, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                           4,  1,  2,  3,  8,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
                           5, 17, 18, 19, 20, 10, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                           6,  1,  2,  3,  4,  5, 12,  7,  8,  9, 10, 11, 12, 13, 14, 15,
                           7, 17, 18, 19, 20, 21, 22, 14, 24, 25, 26, 27, 28, 29, 30, 31,
                           8,  1,  2,  3,  4,  5,  6,  7,  1,  9, 10, 11, 12, 13, 14, 15,
                           9, 17, 18, 19, 20, 21, 22, 23, 24,  3, 26, 27, 28, 29, 30, 31,
                          10,  1,  2,  3,  4,  5,  6,  7,  8,  9,  5, 11, 12, 13, 14, 15,
                          11, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,  7, 28, 29, 30, 31,
                          12,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11,  9, 13, 14, 15,
                          13, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 11, 30, 31,
                          14,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 13, 15,
                          15,  0, 16, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 15,
                          ]);
        let exp = String::from("\
┌─────────────┬─────────────┬─────────────┬─────────────┐
|  .  1  2  3 |  4  5  6  7 |  8  9 10 11 | 12 13 14 15 |
|  1  2  .  . |  .  .  .  . |  .  .  .  . |  .  .  .  . |
|  2  1  4  3 |  4  5  6  7 |  8  9 10 11 | 12 13 14 15 |
|  3  .  .  6 |  .  .  .  . |  .  .  .  . |  .  .  .  . |
├─────────────┼─────────────┼─────────────┼─────────────┤
|  4  1  2  3 |  8  5  6  7 |  8  9 10 11 | 12 13 14 15 |
|  5  .  .  . |  . 10  .  . |  .  .  .  . |  .  .  .  . |
|  6  1  2  3 |  4  5 12  7 |  8  9 10 11 | 12 13 14 15 |
|  7  .  .  . |  .  .  . 14 |  .  .  .  . |  .  .  .  . |
├─────────────┼─────────────┼─────────────┼─────────────┤
|  8  1  2  3 |  4  5  6  7 |  1  9 10 11 | 12 13 14 15 |
|  9  .  .  . |  .  .  .  . |  .  3  .  . |  .  .  .  . |
| 10  1  2  3 |  4  5  6  7 |  8  9  5 11 | 12 13 14 15 |
| 11  .  .  . |  .  .  .  . |  .  .  .  7 |  .  .  .  . |
├─────────────┼─────────────┼─────────────┼─────────────┤
| 12  1  2  3 |  4  5  6  7 |  8  9 10 11 |  9 13 14 15 |
| 13  .  .  . |  .  .  .  . |  .  .  .  . |  . 11  .  . |
| 14  1  2  3 |  4  5  6  7 |  8  9 10 11 | 12 13 13 15 |
| 15  . 16  . |  .  .  .  . |  .  .  .  . |  .  .  . 15 |
└─────────────┴─────────────┴─────────────┴─────────────┘\n");
        assert_eq!(exp, s.pretty_format());
    }

    #[test]
    fn uncomplete() {
        let s = SudokuField::new(2).unwrap();
        assert!(!s.is_complete());
    }

    #[test]
    fn complete() {
        let mut s = SudokuField::new(2).unwrap();
        s.set_fields(vec![1; 16]);
        assert!(s.is_complete());
    }
}

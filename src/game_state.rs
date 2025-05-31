pub struct GameState {
    pub field: Vec<Vec<u8>>
}

impl GameState {
    /// initialize a limited playing field of 10 by 10 empty spaces.
    

    pub fn from_field(field: Vec<Vec<u8>>) -> Self {
        GameState { field: field }
    }

    /// Calculate future value of each cell and assign
    /// new value to field of GameState.
    /// Rules:
    /// <2 or >3 neighbours: become 0,
    /// 3 Neighbours: become 1,
    /// 2 Neighbours: stay the same
    pub fn update(&mut self) {
        let cell_neighbours = self.calc_neighbours();
        // fill new field
        let width = self.field.len();
        let height = self.field[0].len();
        let mut field_new = vec![vec![0; height]; width];
        for i in 0..width {
            for j in 0..height {
                if (cell_neighbours[i][j] <2) | (cell_neighbours[i][j] >3) {    
                    field_new[i][j] = 0;
                } else if cell_neighbours[i][j] == 2 && self.field[i][j] == 0 {
                    field_new[i][j] = 0;
                } else {
                    field_new[i][j] = 1;
                }
            }
        }
        // print!("new: {:?}", field_new);
        self.field = field_new;
    }

    /// calculate amount of neighbours for all cells inside self.field
    pub fn calc_neighbours(&self) -> Vec<Vec<u8>> {
        let width = self.field.len();
        let height = self.field[0].len();
        let mut cell_neighbours = vec![vec![0; height]; width];
        for i in 0..width {
            for j in 0..height {
                let mut sum = 0;
                let neighbours: [isize; 3] = [-1, 0, 1];
                for di in neighbours {
                    for dj in neighbours{
                        // skips middle cell, wrap around vector edges
                        // with way to many casts, wth.
                        if di == 0 && dj == 0 {
                            continue
                        }
                        let cell_i = (i as isize + di + width as isize) % width as isize;
                        let cell_j = (j as isize + dj + height as isize) % height as isize;
                        sum += self.field[cell_i as usize][cell_j as usize];
                    }
                }
                cell_neighbours[i][j] = sum;
            }
        }
        // print!("nbrs: {:?}", cell_neighbours);
        cell_neighbours
    }

    /// prints the current game state field vector using "██" and "  ".
    pub fn print(&self) {
        print!("\x1b[H"); // move cursor to top left
        for row in &self.field {
            for &cell in row {
                let symbol = if cell == 1 {
                    "██"
                } else {
                    "  "
                    };
                print!("{}", symbol);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_state() -> GameState{
        let field = vec![
            vec![1,1,0,0],
            vec![1,0,0,0],
            vec![0,0,0,0],
            vec![0,0,0,0],
        ];
        GameState { field: field }
    }

    #[test]
    fn test_calc_neighbours() {
        let expected = vec![
            vec![2,2,1,2],
            vec![2,3,1,2],
            vec![1,1,0,1],
            vec![2,2,1,1],
        ];
        let mock = make_test_state();
        let result = mock.calc_neighbours();
        dbg!(&result);
        assert_eq!(result, expected);
    }
}
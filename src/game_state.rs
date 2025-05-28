pub struct GameState {
    generation: u8,
    pub field: Vec<Vec<usize>>
}

impl GameState {
    /// initialize a limited playing field (currently 2D)
    /// of 10 spaces.
    pub fn init() -> GameState{
        return GameState {
            generation: 0,
            field: vec![vec![0; 10];10]
        }
    }

    /// Calculate future value of each cell and assign
    /// new value to field of GameState.
    /// Rules:
    /// 2 Neighbours -> toggle state
    /// 1 Neighbours -> stay the same
    /// 0 Neighbours -> become 0
    pub fn update(&mut self) {
        
        // calculate amount of neighbours
        // for all cells
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

        // fill new field
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

        self.generation += 1;
        self.field = field_new;
    }

    pub fn print(&self) {
        let width = self.field.len();
        // let height = self.field[0].len();
        print!("State:\n");
        for i in 0..width {
            print!("{:?}\n", self.field[i]);
        }
    }
}
pub struct GameState {
    pub field: Vec<Vec<u8>>
}

impl GameState {
    /// initialize a limited playing field of 10 by 10 empty spaces.
    pub fn init() -> GameState{
        GameState {
            field: vec![vec![0; 10];10]
        }
    }

    pub fn init_glider() -> GameState {
        GameState {
            field: vec![
                vec![0; 10],
                vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 0, 0, 0, 0, 0, 0, 0],
                vec![0; 10],
                vec![0; 10],
                vec![0; 10],
                vec![0; 10],
                vec![0; 10],
                vec![0; 10],
            ]
        }
    }
    
    pub fn init_rnd() -> GameState {
        use rand;
        let size = 40;
        let mut rnd_vec = vec![vec![0; size]; size];
        for i in 0..size {
            for j in 0..size {
                let rnd = rand::random_range(0..=1);
                print!("{}", rnd);
                rnd_vec[i][j] = rnd;
            }
        }
        GameState {
            field: rnd_vec
        }
    }

    /// Calculate future value of each cell and assign
    /// new value to field of GameState.
    /// Rules:
    /// <2 or >3 neighbours: become 0,
    /// 3 Neighbours: become 1,
    /// 2 Neighbours: stay the same
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

        self.field = field_new;
    }

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
pub struct GameState {
    generation: u8,
    pub field: [usize; 10]
}

impl GameState {
    /// initialize a limited playing field (currently 2D)
    /// of 10 spaces.
    pub fn init() -> GameState{
        return GameState {
            generation: 0,
            field: [1, 0, 1, 0, 1, 0, 1, 0, 1, 0]
        }
    }

    /// TO DO: increase rendering space
    pub fn add_space(){
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
        let mut cell_neighbours: [u8; 10] = [0; 10];
        let mut field_new = [0; 10];
        let mut n = 0;
        for _ in self.field {
            cell_neighbours[n] = if n == 0 {
                (self.field[self.field.len()-1] + self.field[n+1]) as u8
            } else if n == self.field.len()-1 {
                (self.field[n-1] + self.field[self.field.len()-1]) as u8
            } else {
                (self.field[n-1] + self.field[n+1]) as u8
            };
            n += 1;
        }
        // print!("nbrs: {:?}", cell_neighbours);
        // fill new field
        n = 0;
        for i in cell_neighbours {
            if i == 2 {
                if self.field[n] == 0 {
                    field_new[n] = 1;
                }
                else {
                    field_new[n] = 0;
                }
            } else if i == 1 {
                field_new[n] = self.field[n];
            }
            else {
                field_new[n] = 0;
            }
            n += 1;
        }
        // print!("new: {:?}", field_new);

        self.generation += 1;
        self.field = field_new;
    }

    pub fn print(&self) {
        println!("state: {:?}", self.field)
    }
}
struct GameState {
    generation: u8,
    field: [u8; 10]
}

impl GameState {
    fn init() -> GameState{
        /// initialize a limited playing field (currently 2D)
        /// of 10 spaces, all empty.
        return GameState {
            generation: 0,
            field: [0; 10]
        }
    }

    fn add_space(){
        /// increase rendering space, later
    }

    fn update(&self) -> GameState {
        /// create new GameState, where all positions are
        /// decided by their respective values from the
        /// previous generation. also increase generation
        /// value.
        
        // calculate field value (amount of neighbours)
        // for all fields.
        let field_values = [0, self.field.len()];
        let field_new = [0, 10];

        // read out field and calc field_values
        for i in self.field {
            field_values[i] = field[i-1] + field[i+1]
        }

        // fill new field by rules:
        // Toggle 1/0 if value = 2
        // Keep if value = 1
        // 1->0 if value = 0
        for i in range(0, field_values.len()) {
            if i = 2 {
                if self.field[i] = 0 {
                    field_new[i] = 1;
                }
                else {
                    field_new[i] = 0;
                }
            }
            if i = 1 {
                field_new[i] = self.field[1];
            }
            else {
                field_new[i] = 0;
            }
        }

        return GameState{
            generation: self.generation + 1,
            field: field_new
        };
    }
}
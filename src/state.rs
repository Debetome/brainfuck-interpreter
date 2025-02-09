pub struct State {
    cells: Vec<u8>,
    cell_index: usize,    
}

impl State {
    pub fn new() -> Self {
        let mut cells: Vec<u8> = Vec::new();        
        cells.push(0);        
        Self {
            cells,
            cell_index: 0,            
        }
    }    

    pub fn increase_index(&mut self) {
        self.cell_index += 1;
        if self.cells.get(self.cell_index).is_none() {
            self.cells.push(0);
        }
    }
    
    pub fn decrease_index(&mut self) {        
        if self.cell_index == 0 {
            self.cell_index = self.cells.len() - 1;
            return;
        }
        self.cell_index -= 1;
    }

    pub fn increase_value(&mut self) { self.cells[self.cell_index] += 1; } 
    pub fn decrease_value(&mut self) { self.cells[self.cell_index] -= 1; }
    pub fn get_value(&self, index: usize) -> u8 { self.cells[index] }
    pub fn get_index(&self) -> usize { self.cell_index }
}
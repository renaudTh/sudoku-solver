use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MyList {
    size: usize,
    grid: Vec<u8>
}
#[wasm_bindgen]
impl MyList {
    pub fn new(init : Vec<u8>) -> MyList{
        MyList {
            size: init.len(),
            grid: Vec::from(init)
        }
    }
    pub fn get_grid(&self) -> *const u8 {
        return self.grid.as_ptr();
    }
    pub fn get_size(&self) -> usize {
        self.size
    }
    pub fn sort(&mut self) {
        self.grid.sort();
    }
}
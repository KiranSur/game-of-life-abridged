mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn neighbor_count(&self, row:u32, col:u32) -> u8 {
        let mut tot = 0;
        for row_shift in [self.height-1, 0, 1].iter().cloned() {
            for col_shift in [self.width-1, 0, 1].iter().cloned() {
                if row_shift == 0 && col_shift == 0 {
                    continue;
                }

                let n_row = (row + row_shift) % self.height;
                let n_col = (col + col_shift) % self.width;
                let index = self.get_index(n_row, n_col);
                tot += self.cells[index] as u8
            }
        }
        tot
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let n_count = self.neighbor_count(row, col);
                let curr_cell = self.cells[index];

                let next_cell = match (curr_cell, n_count) {

                    // Less than 2 or more than 3 live neighbors = dead
                    (Cell::Alive, x) if (x < 2 || x > 3) => Cell::Dead,

                    // Alive cell with 2-3 live neighbors = alive
                    (Cell::Alive, _) => Cell::Alive,

                    // Dead cell with 3 live neighbors = alive
                    (Cell::Dead, 3) => Cell::Alive,

                    (otherwise, _) => otherwise
                };

                next[index] = next_cell;

            }
        }
        self.cells = next;
    }

    pub fn init() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }
    
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
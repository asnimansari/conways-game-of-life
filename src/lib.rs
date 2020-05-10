mod utils;

extern crate js_sys;
extern crate fixedbitset;

use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;
use std::fmt;
use wasm_bindgen::__rt::core::fmt::{Formatter, Error};
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern {
//     struct Universe;
// }

// #[wasm_bindgen]
// #[repr(u8)]
// #[derive(Copy, Clone, Debug, PartialEq, Eq)]
// pub enum Cell {
//     Dead = 0,
//     Alive = 1,
// }

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_column in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_column == 0 {
                    continue;
                }
                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (column + delta_column) % self.width;
                let idx = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();


        // let mut next = self.cells.clone();
        for row in 0..self.height {
            for column in 0..self.width {
                let idx = self.get_index(row, column);
                let cell = self.cells[idx];

                let live_neighbours = self.get_live_neighbour_count(row, column);
                //
                // log!(
                //     "cell[{}, {}] is initially {:?} and has {} live neighbors",
                //     row,
                //     column,
                //     cell,
                //     live_neighbours
                // );


                next.set(idx, match (cell, live_neighbours) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise
                });
            }
        }
        // let _timer = Timer::new("free old cells");
        self.cells = next;
    }

    pub fn new() -> Self {
        utils::set_panic_hook();

        let width = 128;
        let height = 128;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, js_sys::Math::random() < 0.5)
        }


        Self {
            width,
            height,
            cells,
        }
    }

    // pub fn render(&self) -> String {
    //     self.to_string()
    // }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        (0..width * self.height).for_each(|i| self.cells.set(i as usize, false));
    }
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        (0..self.width * height).for_each(|i| self.cells.set(i as usize, false));
    }
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.set(idx, !self.cells[idx]);
    }
}

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}


pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

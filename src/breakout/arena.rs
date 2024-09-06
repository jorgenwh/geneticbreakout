// An attribute to hide warnings for unused code.
#![allow(dead_code)]

pub struct Arena {
    h_blocks: u16,
    v_blocks: u16,
    block_width: u16,
    block_height: u16,
    block_padding: u16,
    paddle_to_blocks_distance: u16,
    paddle_width: u16,
    blocks_array: Vec<Vec<bool>>,
    blocks_perimeter: Vec<Vec<bool>>,
}

impl Default for Arena {
    fn default() -> Self {
        Self {
            h_blocks: 14,
            v_blocks: 8,
            block_width: 20,
            block_height: 10,
            block_padding: 4,
            paddle_to_blocks_distance: 30,
            paddle_width: 20,
            blocks_array: vec![vec![true; 14]; 8],
            blocks_perimeter: vec![vec![false; 14]; 8],
        }
    }
}

impl Arena {
    pub fn new(
        h_blocks: u16,
        v_blocks: u16,
        block_width: u16,
        block_height: u16,
        block_padding: u16,
        paddle_to_blocks_distance: u16,
        paddle_width: u16,
    ) -> Self {
        Self {
            h_blocks,
            v_blocks,
            block_width,
            block_height,
            block_padding,
            paddle_to_blocks_distance,
            paddle_width,
            blocks_array: vec![vec![true; h_blocks as usize]; v_blocks as usize],
            blocks_perimeter: vec![vec![false; h_blocks as usize]; v_blocks as usize],
        }
    }

    pub fn find_perimeter(&mut self) -> &Vec<Vec<bool>> {
        for i in 0..self.blocks_array.len() {
            for j in 0..self.blocks_array[i].len() {
                if self.blocks_array[i][j] {
                    if i == 0
                        || i == self.blocks_array.len() - 1
                        || j == 0
                        || j == self.blocks_array[i].len() - 1
                    {
                        self.blocks_perimeter[i][j] = true;
                    } else {
                        if !self.blocks_array[i - 1][j]
                            || !self.blocks_array[i + 1][j]
                            || !self.blocks_array[i][j - 1]
                            || !self.blocks_array[i][j + 1]
                        {
                            self.blocks_perimeter[i][j] = true;
                        }
                    }
                }
            }
        }
        &self.blocks_perimeter
    }

    pub fn break_block(&mut self, x: u16, y: u16) {
        if x < self.h_blocks && y < self.v_blocks {
            self.blocks_array[y as usize][x as usize] = false;
        }
    }

    pub fn print_blocks(&self) {
        for i in 0..self.blocks_array.len() {
            for j in 0..self.blocks_array[i].len() {
                print!("{}, ", self.blocks_array[i][j]);
            }
            println!();
        }
    }

    pub fn print_perimeter(&self) {
        for i in 0..self.blocks_perimeter.len() {
            for j in 0..self.blocks_perimeter[i].len() {
                print!("{}, ", self.blocks_perimeter[i][j]);
            }
            println!();
        }
    }

    pub fn arena_info(&self) {
        println!(
            "Arena has horizontal blocks: {}, vertical blocks: {}, paddle to blocks distance: {}",
            self.h_blocks, self.v_blocks, self.paddle_to_blocks_distance,
        );
    }

    pub fn arena_size(&self) -> (u16, u16) {
        let width = self.h_blocks * self.block_width + (self.h_blocks + 1) * self.block_padding;
        let height = self.v_blocks * self.block_height
            + self.paddle_to_blocks_distance * self.block_height
            + (self.v_blocks + self.paddle_to_blocks_distance + 1) * self.block_padding;
        (width, height)
    }

    pub fn get_blocks(&self) -> (u16, u16) {
        (self.h_blocks, self.v_blocks)
    }

    pub fn get_blocks_array(&self) -> &Vec<Vec<bool>> {
        &self.blocks_array
    }
}

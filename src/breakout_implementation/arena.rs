pub struct Arena {
    h_blocks: u16,
    v_blocks: u16,
    block_width: u16,
    block_height: u16,
    block_padding: u16,
    paddle_to_blocks_distance: u16,
    paddle_width: u16,
    blocks_array: Vec<Vec<bool>>,
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
            blocks_array: vec![vec![true; 20]; 10],
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
            blocks_array: vec![vec![true; block_width as usize]; block_height as usize],
        }
    }

    pub fn draw_block_polygon(&self) {}

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
}

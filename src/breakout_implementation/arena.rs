pub struct Arena {
    h_blocks: u16,
    v_blocks: u16,
    paddle_to_blocks_distance: u16,
    block_gap_size: u16,
    block_width: u16,
    block_height: u16,
}

impl Default for Arena {
    fn default() -> Self {
        Self {
            h_blocks: 15,
            v_blocks: 8,
            paddle_to_blocks_distance: 20,
            block_gap_size: 5,
            block_width: 50,
            block_height: 20,
        }
    }
}

#[allow(dead_code)]
impl Arena {
    pub fn new(
        h_blocks: u16,
        v_blocks: u16,
        paddle_to_blocks_distance: u16,
        block_gap_size: u16,
        block_width: u16,
        block_height: u16,
    ) -> Self {
        Self {
            h_blocks,
            v_blocks,
            paddle_to_blocks_distance,
            block_gap_size,
            block_width,
            block_height,
        }
    }

    pub fn arena_info(&self) {
        println!(
            "Arena has horizontal blocks: {}, vertical blocks: {}, paddle to blocks distance: {},
            block gap size: {}, block width: {}, block height: {}",
            self.h_blocks,
            self.v_blocks,
            self.paddle_to_blocks_distance,
            self.block_gap_size,
            self.block_width,
            self.block_height
        );
    }
}

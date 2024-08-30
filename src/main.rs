mod breakout_implementation {
    pub mod arena;
    pub mod ball;
}

fn main() {
    let arena = breakout_implementation::arena::Arena::default();
    arena.arena_info();
    println!("Arena size (width by height): {:?}", arena.arena_size());
    println!("Blocks in the arena: {:?}", arena.get_blocks());
    let (width, height) = arena.get_blocks();
    let blocks_array: Vec<Vec<bool>> = vec![vec![true; width as usize]; height as usize];

    //print the blocks array
    for i in 0..blocks_array.len() {
        for j in 0..blocks_array[i].len() {
            print!("{}, ", blocks_array[i][j]);
        }
        println!();
    }
}

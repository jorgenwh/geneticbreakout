mod breakout {
    pub mod arena;
    pub mod ball;
}

fn main() {
    let mut arena = breakout::arena::Arena::default();

    arena.arena_info();
    println!("Arena size (width by height): {:?}", arena.arena_size());
    println!("Blocks in the arena: {:?}", arena.get_blocks());

    println!("Blocks array:");
    arena.print_blocks();

    println!("Perimeter blocks:");
    arena.init_perimeter();
    arena.print_perimeter();

    println!("Breaking block at x: 14, y: 8");
    arena.break_block(14, 8);
    arena.init_perimeter();
    arena.print_perimeter();

    arena.print_blocks();
}

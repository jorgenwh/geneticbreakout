mod breakout_implementation {
    pub mod arena;
    pub mod ball;
}

fn main() {
    let arena = breakout_implementation::arena::Arena::default();
    arena.arena_info();
}

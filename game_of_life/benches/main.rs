use jpcd_game_of_life::cell::Cell;

// main used for benchmarks - build with profiling profile (release and debug)
fn main() {
    let mut universe = jpcd_game_of_life::universe::Universe::new(256, 256);
    (0..256 * 256).into_iter()
        .filter(|i| i % 11 == 0 || i % 7 == 0 || i % 9 == 0)
        .for_each(|id| universe.toggle(id%256, id/256));

    for _ in 0..1000 {
        universe.tick();
        let sum: usize = universe.enumerate_rows().map(|(n, c)| {
            c.iter().enumerate()
                .filter(|ce| ce.1 == &Cell::Alive)
                .count()
        }).sum();
        println!("{sum}");
    }
}
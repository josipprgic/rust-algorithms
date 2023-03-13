#![feature(test)]

extern crate test;
extern crate jpcd_game_of_life;

use jpcd_game_of_life::cell::Cell;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = jpcd_game_of_life::universe::Universe::new(64, 64);
    (0..64 * 64).into_iter()
        .filter(|i| i % 2 == 0 || i % 7 == 0)
        .for_each(|id| universe.toggle(id%64, id/64));

    b.iter(|| {
        universe.tick();
    });
}
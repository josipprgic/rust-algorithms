use log::log;
use crate::cell::Cell;

pub struct Universe {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    iter_cnt: u64
}

impl Universe {

    pub fn tick(&mut self) {
        let mut cells = self.cells.clone();
        for i in 0..(self.width * self.height) {
            cells[i as usize] = match self.count_neighbors(i) {
                0 | 1 => Cell::Dead,
                2 => self.cells[i],
                3 => Cell::Alive,
                _ => Cell::Dead
            }
        }

        self.cells = cells;
        self.iter_cnt += 1;
    }

    pub fn new(width: usize, height: usize) -> Universe {
        Universe {
            cells: (0..width*height).into_iter().map(|_| Cell::Dead).collect::<Vec<Cell>>(),
            width,
            height,
            iter_cnt: 0
        }
    }

    pub fn toggle(&mut self, x: usize, y: usize) {
        if x > self.width {
            log::error!{"Width parameter: {x} does NOT fit the universe of width {}", self.width}
        }
        if y > self.height {
            log::error!{"Height parameter: {y} does NOT fit the universe of height {}", self.height}
        }

        let idx = y * self.width + x;
        self.cells[idx] = self.cells[idx].toggle()
    }

    fn count_neighbors(&self, idx: usize) -> u8 {
        let up_cell = if idx < self.width { self.width * (self.height - 1) + idx } else {idx - self.width};
        let down_cell = if idx >= self.width * (self.height - 1) {idx} else { idx + self.width };
        let idx = vec![
            if up_cell == 0 {self.width * self.height - 1} else { up_cell - 1},
            up_cell,
            if up_cell == self.width * self.height - 1 {0} else {up_cell + 1},
            if idx == 0 { self.width * self.height - 1} else { idx - 1 },
            if idx == self.width * self.height - 1 {0} else { idx + 1 },
            if down_cell == 0 {self.width * self.height - 1} else { down_cell - 1},
            down_cell,
            if down_cell == self.width * self.height - 1 {0} else {down_cell + 1}];

        let mut count = 0;

        for i in idx {
            if self.cells[i] == Cell::Alive {
                count += 1;
            }
            if count > 3 {
                return count
            }
        }

        count
    }
}

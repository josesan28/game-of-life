use crate::game_of_life::GameOfLife;

pub fn add_glider(game: &mut GameOfLife, x: u32, y: u32) {
    let coords = [(1,0), (2,1), (0,2), (1,2), (2,2)];
    for (dx, dy) in coords.iter() {
        game.set_cell(x + dx, y + dy, true);
    }
}

pub fn add_blinker(game: &mut GameOfLife, x: u32, y: u32) {
    for i in 0..3 {
        game.set_cell(x + i, y, true);
    }
}

pub fn add_toad(game: &mut GameOfLife, x: u32, y: u32) {
    let coords = [(1,0), (2,0), (3,0), (0,1), (1,1), (2,1)];
    for (dx, dy) in coords.iter() {
        game.set_cell(x + dx, y + dy, true);
    }
}

use ncurses::getch;
use crate::{assets, tetromino};


pub fn update_input(
    x: &mut i32,
    y: &mut i32,
    field : &assets::FIELD,
    piece_id : i32,
    rotation_id : &mut i32,
    rotate_hold : &mut bool,
    game_over : &mut bool
) {
    let key : i32 = getch();
    if key == 27 {
        *game_over = true;
    } else if key == 260 { // L
        *x -= if tetromino::does_piece_fit(field, piece_id, *rotation_id, *x - 1, *y) {1} else {0};
    } else if key == 261 { // R
        *x += if tetromino::does_piece_fit(field, piece_id, *rotation_id, *x + 1, *y) {1} else {0};
    } else if key == 258 { // D
        *y += if tetromino::does_piece_fit(field, piece_id, *rotation_id, *x, *y + 1) {1} else {0};
    } else if (key == 32) | (key == 259) { // U
        *rotation_id += if *rotate_hold && tetromino::does_piece_fit(field, piece_id, *rotation_id + 1, *x, *y) {1} else {0};
        *rotate_hold = false;
    } else {
        *rotate_hold = true;
    }
}

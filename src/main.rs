
extern crate ncurses;
mod assets;
mod tetromino;
mod input;
mod ui;

use std::{thread, time};
use rand::Rng;
use ncurses::*;
use ui::{Origin, Size};

fn main() {
    /* Init screen */
    let screen_size : Size = ui::init_screen();

    /* Start field in the center. */
    let start_y = (screen_size.lines - (assets::WINDOW_HEIGHT as i32)) / 2;
    let start_x = (screen_size.columns - (assets::WINDOW_WIDTH as i32)) / 2;
    let field_origin : Origin = Origin{y: start_y, x: start_x};
    let field_size : Size = Size{lines: assets::FIELD_HEIGHT, columns: assets::FIELD_WIDTH};
    let score_position : Origin = Origin{y: start_y - 2, x: start_x};

    /* Status/help info. */
    ui::draw_score(&score_position, 0);

    let win : WINDOW = ui::create_win(field_origin, field_size);

    /* Create field */
    let mut field : assets::FIELD = ui::build_field();
    
    /* Game logic */
    let mut rng = rand::thread_rng();
    let mut piece_id : i32 = 0;
    let mut rotation_id : i32 = 0;
    let mut x : i32 = assets::FIELD_WIDTH / 2;
    let mut y : i32 = assets::FIELD_WIDTH / 2;
    let mut speed : i32 = 50;
    let mut speed_count : i32 = 0;
    let mut force_down : bool;
    let mut rotate_hold : bool = true;
    let mut piece_count : i32 = 0;
    let mut score : i32 = 0;
    let mut v_lines : Vec<i32> = Vec::new();
    let mut game_over : bool = false;

    while !game_over { 
        thread::sleep(time::Duration::from_millis(10));
        speed_count += 1;
        force_down = speed_count == speed;

        input::update_input(&mut x, &mut y, &field, piece_id, &mut rotation_id, &mut rotate_hold, &mut game_over);

        // Force the piece down the playfield if it's time
        if force_down {
            // Update difficulty every 50 pieces
            speed_count = 0;
            piece_count += 1;
            if piece_count % 50 == 0 {
                if speed >= 10 {
                    speed -= 1;
                };
            }

            // Test if piece can be moved down
            if tetromino::does_piece_fit(&field, piece_id, rotation_id, x, y + 1) {
                y += 1; // it can, so do it!
            } else {
                // It can't! lock the piece in place
                for i in 0..tetromino::TETROMINO_WIDTH {
                    for j in 0..tetromino::TETROMINO_HEIGHT {
                        let p_idx : usize = tetromino::rotate(i, j, rotation_id);
                        if tetromino::TETROMINOS[piece_id as usize].chars().nth(p_idx).unwrap() != '.' {
                            let f_id: usize = ((y + j) * assets::FIELD_WIDTH + (x + i)).try_into().unwrap();
                            field[f_id] = tetromino::PIECE_CHARS[piece_id as usize];
                        }
                    }
                }

                // Check for lines
                for j in 0..tetromino::TETROMINO_HEIGHT {
                    if y + j < assets::FIELD_HEIGHT - 1 {
                        let mut line : bool = true;
                        let mut f_id : usize;
                        for p_x in 1..assets::FIELD_WIDTH - 1 {
                            f_id = ((y + j) * assets::FIELD_WIDTH + p_x).try_into().unwrap();
                            line &= field[f_id] != ' '
                        }

                        if line {
                            // Remove Line
                            for p_x in 1..assets::FIELD_WIDTH - 1 {
                                f_id = ((y + j) * assets::FIELD_WIDTH + p_x).try_into().unwrap();
                                field[f_id] = '=';
                            }
                            v_lines.push(y + j);
                        }
                    }
                }

                score += 25;
                if v_lines.len() > 0 {
                    score += (1 << v_lines.len() as u8) * 100;
                }

                // Pick new piece
                x = assets::FIELD_WIDTH / 2;
                y = 0;
                rotation_id = 0;
                piece_id = (rng.gen::<u32>() % 7).try_into().unwrap();

				// If piece does not fit straight away, game over!
                game_over = !tetromino::does_piece_fit(&field, piece_id, rotation_id, x, y);
            }
        }

        // Display ======================
        // Draw field
        ui::draw_field(&win, &field);
		
        // Draw Current Piece
        ui::draw_piece(&win, x, y, piece_id, rotation_id);

        // Draw score
        ui::draw_score(&score_position, score);

        // Animate completion
        ui::animate_completion(&win, &mut field, &v_lines);
        v_lines.clear();
        // Refresh frame
        wrefresh(win);
    }

    ui::close_screen(win);
}

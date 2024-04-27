
// use gettextrs::{*, LocaleCategory};
use ncurses::*;
use std::{error, thread, time};
use crate::{assets, colors, tetromino};

pub struct Size {
    pub lines : i32,
    pub columns : i32
}

pub struct Origin {
    pub y : i32,
    pub x : i32
}

pub fn init_screen() -> Result<Size, Box<dyn error::Error>>
{
    /* Set LC_ALL */
    // setLocale(LocaleCategory::LcAll, "");

    /* Start ncurses. */
    initscr();
    raw();
    keypad(stdscr(), true);
    cbreak();
    noecho();

    if !has_colors() {
        endwin();
        return Err("Your terminal does not support color.\n".into());
    }

    start_color();
    colors::init_color_pairs();

    if (LINES() < 24) || (COLS() < 80) {
        endwin();
        return Err("Your terminal needs to be at least 80 x 24.\n".into());
    }

    /* Update the screen. */
    refresh();

    /* Invisible cursor. */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    /* Non-blocking inputs */
    timeout(0);

    /* Get the screen bounds. */
    let mut m_x : i32 = 0;
    let mut m_y : i32 = 0;
    getmaxyx(stdscr(), &mut m_y, &mut m_x);
    let screen_size : Size =  Size {lines: m_y, columns: m_x};
    Ok(screen_size)
}

pub fn close_screen(win: WINDOW)
{
    destroy_win(win);

    /* Terminate ncurses. */
    endwin();
}


pub fn create_win(origin: Origin, size: Size) -> WINDOW
{   
    let win : WINDOW;
    // win = newwin(size.lines, size.columns, origin.y, origin.x);
    // mvwvline(win, 0, 0, ACS_VLINE(), size.lines - 1);
    // mvwvline(win, 0, size.columns - 1, ACS_VLINE(), size.lines - 1);
    // mvwhline(win, size.lines - 1, 1, ACS_HLINE(), size.columns - 2);
    // mvwaddch(win, size.lines - 1, 0, ACS_LLCORNER());
    // mvwaddch(win, size.lines - 1, size.columns - 1, ACS_LRCORNER());
    /* This is a hack so that each block has aspect ratio 1:1 */
    win = newwin(size.lines, 2 * size.columns, origin.y, origin.x);
    mvwvline(win, 0, 1, ACS_VLINE(), size.lines - 1);
    mvwvline(win, 0, 2 * (size.columns - 1), ACS_VLINE(), size.lines - 1);
    mvwhline(win, size.lines - 1, 2, ACS_HLINE(), 2 * (size.columns - 1) - 1);
    mvwaddch(win, size.lines - 1, 1, ACS_LLCORNER());
    mvwaddch(win, size.lines - 1, 2 * (size.columns - 1), ACS_LRCORNER());
    wrefresh(win);
    win
}

pub fn destroy_win(win: WINDOW)
{
    let ch = ' ' as chtype;
    wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
    wrefresh(win);
    delwin(win);
}

pub fn build_field() -> assets::FIELD {
    let mut field : assets::FIELD = Vec::new();
    for j in 0..assets::FIELD_HEIGHT {
        field.push(assets::V_BORDER);
        for _i in 1..assets::FIELD_WIDTH-1 {
            let c : char;
            if j == assets::FIELD_HEIGHT - 1 {
                c = assets::H_BORDER;
            } else {
                c = ' ';
            }
            field.push(c);
        }
        field.push(assets::V_BORDER);
    }
    field
}

pub fn get_piece_color_id(piece_char : char) -> i16 {
    let mut color_id: i16 = 0;
    for i in 0..tetromino::PIECE_CHARS.len() {
        if piece_char == tetromino::PIECE_CHARS[i] {
            color_id = (i as i16) + 1;
            break;
        }
    }
    color_id
}

pub fn draw_field(win: &WINDOW, field: &assets::FIELD) {
    // Draw Field
    let mut f_idx : usize;
    let mut color_id: i16;
    let mut fg: i16;
    let mut bg: i16;
    for i in 1..assets::FIELD_WIDTH - 1 {
        for j in 0..assets::FIELD_HEIGHT - 1 {
            f_idx = (j * assets::FIELD_WIDTH + i).try_into().unwrap();
            color_id = get_piece_color_id(field[f_idx]);
            if color_id > 0 {
                fg = color_id;
                bg = color_id;
            } else {
                fg = 7 + 8;
                bg = color_id;
            }
            colors::set_color(fg, bg, Some(win));
            // mvwaddch(win.clone(), j, i, field[f_idx] as chtype);
            /* This is a hack so that each block has aspect ratio 1:1 */
            mvwaddch(win.clone(), j, 2 * i, field[f_idx] as chtype);
            mvwaddch(win.clone(), j, 2 * i + 1, field[f_idx] as chtype);
            colors::unset_color(fg, bg, Some(win)); 
        }
    }
}

pub fn draw_piece(win: &WINDOW, x: i32, y: i32, piece_id: i32, rotation_id: i32) {
    let mut p_idx : usize;
    let idx : usize = piece_id.try_into().unwrap();
    let mut color_id : i16;
    for i in 0..tetromino::TETROMINO_WIDTH {
        for j in 0..tetromino::TETROMINO_HEIGHT {
            p_idx = tetromino::rotate(i, j, rotation_id);
            if tetromino::TETROMINOS[idx].chars().nth(p_idx).unwrap() != '.' {
                color_id = (piece_id as i16) + 1;
                colors::set_color(color_id, color_id, Some(win));
                // mvwaddch(win.clone(), y + j, x + i, tetromino::PIECE_CHARS[idx] as chtype);
                /* This is a hack so that each block has aspect ratio 1:1 */
                mvwaddch(win.clone(), y + j, 2 * (x + i), tetromino::PIECE_CHARS[idx] as chtype);
                mvwaddch(win.clone(), y + j, 2 * (x + i) + 1, tetromino::PIECE_CHARS[idx] as chtype);
                colors::unset_color(color_id, color_id, Some(win));
            }
        }
    }
}

pub fn draw_score(position: &Origin, score: i32, highlight: bool) {
    // Set string
    let score_str : String = format!("SCORE: {:width$}", score, width=(2 * assets::FIELD_WIDTH as usize) - 11);
    let fg : i16;
    let bg : i16 = 0;
    if highlight {
        fg = 7 + 8;
    } else {
        fg = 3 + 8;
    }
    // Draw score box
    colors::set_color(fg, bg, None);
    mvhline(position.y - 1, position.x, ACS_HLINE(), score_str.len() as i32);
    mvaddch(position.y - 1, position.x - 1, ACS_ULCORNER());
    mvaddch(position.y - 1, position.x + (score_str.len() as i32), ACS_URCORNER());
    mvvline(position.y, position.x - 1, ACS_VLINE(), 1);
    mvvline(position.y, position.x + (score_str.len() as i32), ACS_VLINE(), 1);
    mvhline(position.y + 1, position.x, ACS_HLINE(), score_str.len() as i32);
    mvaddch(position.y + 1, position.x - 1, ACS_LLCORNER());
    mvaddch(position.y + 1, position.x + (score_str.len() as i32), ACS_LRCORNER());
    // Draw score
    mvaddstr(position.y, position.x, &score_str).unwrap();
    colors::unset_color(fg, bg, None);
}

pub fn animate_completion(win: &WINDOW, field : &mut assets::FIELD, v_lines : &Vec<i32>) {
    // Animate Line Completion
    if v_lines.len() > 0 {
        // Display Frame (cheekily to draw lines)
        wrefresh(*win);
        thread::sleep(time::Duration::from_millis(400));
        
        for &v in v_lines.iter() {
            let mut idx_cur : usize;
            let mut idx_prv : usize;
            for i in 1..assets::FIELD_WIDTH - 1 {
                for j in (1..v + 1).rev() {
                    idx_cur = (j * assets::FIELD_WIDTH + i).try_into().unwrap();
                    idx_prv = ((j - 1) * assets::FIELD_WIDTH + i).try_into().unwrap();
                    field[idx_cur] = field[idx_prv];
                }
                field[i as usize] = ' ';
            }
        }
        draw_field(win, field);
    }
}

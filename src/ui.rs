
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
    win = newwin(size.lines, size.columns, origin.y, origin.x);
    mvwvline(win, 0, 0, ACS_VLINE(), size.lines - 1);
    mvwvline(win, 0, size.columns - 1, ACS_VLINE(), size.lines - 1);
    mvwhline(win, size.lines - 1, 1, ACS_HLINE(), size.columns - 2);
    mvwaddch(win, size.lines - 1, 0, ACS_LLCORNER());
    mvwaddch(win, size.lines - 1, size.columns - 1, ACS_LRCORNER());
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
            if j == assets::FIELD_HEIGHT-1 {
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

pub fn draw_field(win: &WINDOW, field: &assets::FIELD) {
    // Draw Field
    let mut f_idx : usize;
    for i in 1..assets::FIELD_WIDTH - 1 {
        for j in 0..assets::FIELD_HEIGHT - 1 {
            f_idx = (j * assets::FIELD_WIDTH + i).try_into().unwrap();
            mvwaddch(win.clone(), j, i, field[f_idx] as chtype);
        }
    }
}

pub fn draw_piece(win: &WINDOW, x: i32, y: i32, piece_id: i32, rotation_id: i32) {
    let mut p_idx : usize;
    let idx : usize = piece_id.try_into().unwrap();
    for i in 0..tetromino::TETROMINO_WIDTH {
        for j in 0..tetromino::TETROMINO_HEIGHT {
            p_idx = tetromino::rotate(i, j, rotation_id);
            if tetromino::TETROMINOS[idx].chars().nth(p_idx).unwrap() != '.' {
                mvwaddch(win.clone(), y + j, x + i, tetromino::PIECE_CHARS[idx] as chtype);
            }
        }
    }
}

pub fn draw_score(position: &Origin, score: i32) {
    // Draw Score
    let score_str : String = format!("SCORE: {:05}", score);
    colors::set_color(1, 0);
    mvaddstr(position.y, position.x, &score_str).unwrap();
    colors::unset_color(1, 0);
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

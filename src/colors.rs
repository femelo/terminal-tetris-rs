use ncurses::*;

const COLOR_BLACK : i16 = 0;
const COLOR_RED : i16 = 1;
const COLOR_GREEN : i16 = 2;
const COLOR_YELLOW : i16 = 3;
const COLOR_BLUE : i16 = 4;
const COLOR_MAGENTA : i16 = 5;
const COLOR_CYAN : i16 = 6;
const COLOR_WHITE : i16 = 7;

const A_BOLD : u32 = 1 << (13 + 8);

/** Helper functions for setting colors
    References:
    - https://www.linuxjournal.com/content/about-ncurses-colors-0
    - https://tldp.org/HOWTO/NCURSES-Programming-HOWTO/color.html
**/
pub fn color_num(fg : i16, bg: i16) -> i16 {
    let b : i16 = 1 << 7;
    let bbb : i16 = (7 & bg) << 4;
    let ffff : i16 = 7 & fg;

    return b | bbb | ffff;
}

pub fn curs_color(fg: i16) -> i16 {
    let color = match 7i16 & fg { /* RGB */
        0 => COLOR_BLACK,   /* 000 */
        1 => COLOR_BLUE,    /* 001 */
        2 => COLOR_GREEN,   /* 010 */
        3 => COLOR_CYAN,    /* 011 */
        4 => COLOR_RED,     /* 100 */
        5 => COLOR_MAGENTA, /* 101 */
        6 => COLOR_YELLOW,  /* 110 */
        7 => COLOR_WHITE,   /* 111 */
        _ => COLOR_BLACK
    };
    color
}

pub fn init_color_pairs() {
    let mut color_pair : i16;

    for bg in 0i16..8 {
        for fg in 0i16..8 {
            color_pair = color_num(fg, bg);
            init_pair(color_pair, curs_color(fg), curs_color(bg));
        }
    }
}

pub fn is_bold(fg: i16) -> bool {
    /* Return the intensity bit */
    let i : i16 = 1 << 3;
    return i & fg != 0;
}

pub fn set_color(fg: i16, bg: i16, window : Option<&WINDOW>) {
    /* Set the color pair (color_num) and bold/bright (A_BOLD) */
    if let Some(win) = window {
        wattr_on(win.clone(), COLOR_PAIR(color_num(fg, bg)));
        if is_bold(fg) {
            wattron(win.clone(), A_BOLD);
        }
    } else {
        attron(COLOR_PAIR(color_num(fg, bg)));
        if is_bold(fg) {
            attron(A_BOLD);
        }
    }    
}

pub fn unset_color(fg: i16, bg: i16, window : Option<&WINDOW>)
{
    /* Unset the color pair (color_num) and bold/bright (A_BOLD) */
    if let Some(win) = window {
        wattr_off(win.clone(), COLOR_PAIR(color_num(fg, bg)));
        if is_bold(fg) {
            wattr_off(win.clone(), A_BOLD);
        }
    } else {
        attroff(COLOR_PAIR(color_num(fg, bg)));
        if is_bold(fg) {
            attroff(A_BOLD);
        }
    }
}

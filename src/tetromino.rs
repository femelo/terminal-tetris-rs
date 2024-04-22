use crate::assets;

pub const TETROMINO_WIDTH : i32 = 4;
pub const TETROMINO_HEIGHT: i32 = 4;

pub const TETROMINOS : &[&str] = &[
    "..X...X...X...X.",
    "..X..XX..X......",
    "..X..XX..X......",
    ".X...XX...X.....",
    "..X..XX...X.....",
    ".....XX...X...X.",
    ".....XX..X...X.."
];

pub const PIECE_CHARS : &[char] = &['A', 'B', 'C', 'D', 'E', 'F', 'G'];

pub fn rotate(p_x: i32, p_y: i32, r: i32) -> usize {
    let index : usize = match r % 4 {
        0 => (4 * p_y + p_x).try_into().unwrap(),      //   0 degrees
        1 => (12 + p_y - 4 * p_x).try_into().unwrap(), //  90 degrees
        2 => (15 - 4 * p_y - p_x).try_into().unwrap(), // 180 degrees
        3 => (3 - p_y + 4 * p_x).try_into().unwrap(),  // 270 degrees
        _ => 0usize
    };
    return index
}

pub fn does_piece_fit(field: &assets::FIELD, piece_id: i32, rotation_id: i32, p_x: i32, p_y: i32) -> bool
{
    // All Field cells > 0 are occupied
    for i in 0..TETROMINO_WIDTH {
        for j in 0..TETROMINO_HEIGHT {
            // Get index in piece
            let p_idx : usize = rotate(i, j, rotation_id);

            // Get field coordinates
            let f_x : i32 = p_x + i;
            let f_y : i32 = p_y + j;

            // Get field index
            let f_idx : usize = (f_y * assets::FIELD_WIDTH + f_x).try_into().unwrap(); 

            // Check that test is in bounds. Note out of bounds does
			// not necessarily mean a fail, as the long vertical piece
			// can have cells that lie outside the boundary, so we'll
			// just ignore them
            let id : usize = piece_id.try_into().unwrap();
            if (f_x >= 0) & (f_x < assets::FIELD_WIDTH) {
                if (f_y >= 0) & (f_y < assets::FIELD_HEIGHT) {
                    // inside bounds so do collision check
                    if (TETROMINOS[id].chars().nth(p_idx).unwrap() != '.') & (field[f_idx] != ' ') {
                        return false; // fail on first hit
                    }
                }
            }
        }
    }
	return true
}

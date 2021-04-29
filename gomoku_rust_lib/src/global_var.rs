//! Global variables used in rust.

// Search algorithm global vars
pub static DEPTH: i32 = 3;
// depth reach with negamax
pub static mut MAX_DEPTH_REACH: i32 = 0;

// Capturing stone count
pub static mut WHITE_CAPTURED_STONE: i8 = 0;
pub static mut BLACK_CAPTURED_STONE: i8 = 0;

// Player color
pub static PLAYER_WHITE_NB: i8 = 1;
pub static PLAYER_BLACK_NB: i8 = -1;

// Axes global var
pub static AXE_MOUVEMENT_VALUE: [usize; 4] = [20, 19, 18, 1];

// BOARD VAR

pub static BOARD_MIN_LIMITS: usize = 0;
pub static BOARD_MAX_LIMITS: usize = 360;

// Move validation check
pub static VALID_MOVE: i8 = 0;
pub static OUT_OF_BOARD_MOVE: i8 = -1;
pub static OVERLAPPING_STONE_MOVE: i8 = -2;
pub static DOUBLE_TRIPLE_MOVE: i8 = -3;

// Pattern use in bitpattern
pub static PATTERN: [(u8, usize, usize, i32, &str, i8); 9] = [
    (0xF8, 6, 0, 200, "five", 0),           // five XXXXX...
    (0x78, 6, 0, 90, "four", 1),            // four .XXXX...
    (0x74, 7, 4, 80, "split four 3", 1),    // split four 3 .XXX.X..
    (0x5C, 7, 2, 80, "split four 1", 1),    // split four 1 .X.XXX..
    (0x6C, 7, 3, 60, "split four 2", 1),    // split four 2 .XX.XX..
    (0x70, 5, 0, 50, "three", 2),           // three  .XXX....
    (0x58, 6, 2, 30, "split three", 2),     // split three .X.XX...
    (0x68, 6, 3, 30, "split three rev", 2), // split three rev .XX.X...
    (0x60, 4, 0, 10, "double", 3),          // double 	.XX.....
];

pub static CAPTURE_PATTERN: [(u8, usize, &str); 2] = [
    (0x90, 5, "capturing pair"), // capturing pair	X..X....
    (0x60, 4, "double"),         // double 	.XX.....
];

pub static BLOCKER: [(u8, usize); 5] = [
    (0x82, 7), // X.....X.
    (0x84, 6), // X....X..
    (0x88, 5), // X...X...
    (0x90, 4), // X..X....
    (0xA0, 3), // X.X.....
];

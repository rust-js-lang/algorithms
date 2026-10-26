use std::fmt;

/// Class that represents queen position on the chessboard.
#[cfg_attr(rust_js, rust_js::class)]
pub struct QueenPosition {
    pub row_index: i32,
    pub column_index: i32,
}

unsafe impl js::Defined for QueenPosition {}

#[cfg_attr(rust_js, rust_js::class)]
impl QueenPosition {
    pub fn new(row_index: i32, column_index: i32) -> Self {
        QueenPosition {
            row_index,
            column_index,
        }
    }

    #[cfg_attr(rust_js, rust_js::getter)]
    pub fn left_diagonal(&self) -> i32 {
        // Each position on the same left (\) diagonal has the same difference of
        // rowIndex and columnIndex. This fact may be used to quickly check if two
        // positions (queens) are on the same left diagonal.
        // @see https://youtu.be/xouin83ebxE?t=1m59s
        self.row_index - self.column_index
    }

    #[cfg_attr(rust_js, rust_js::getter)]
    pub fn right_diagonal(&self) -> i32 {
        // Each position on the same right diagonal (/) has the same
        // sum of rowIndex and columnIndex. This fact may be used to quickly
        // check if two positions (queens) are on the same right diagonal.
        // @see https://youtu.be/xouin83ebxE?t=1m59s
        self.row_index + self.column_index
    }
}

impl fmt::Display for QueenPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.row_index, self.column_index)
    }
}

js::export_default!(QueenPosition);

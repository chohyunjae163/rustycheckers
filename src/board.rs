#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GamePiece {
    pub color : PieceColor,
    pub crowned : bool,
}




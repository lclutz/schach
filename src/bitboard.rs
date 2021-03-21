pub struct Bitboard {
    pub kings: u64,
    pub queens: u64,
    pub knights: u64,
    pub bishops: u64,
    pub rooks: u64,
    pub pawns: u64,
}

impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard {
            kings: 0,
            queens: 0,
            knights: 0,
            bishops: 0,
            rooks: 0,
            pawns: 0,
        }
    }
}

mod ai;
mod compute_tables;
mod generation;
mod init;
mod magic;
mod perft;
mod validity;




use crate::board::Squares::NoSq;



const MAX_PLY : i32 = 192;

const HASH_SIZE : i32 = 0xa000000;

const EMPTY_STRING : String = String::new();

const HASH_FLAG_EXACT : u8 = 0;

const HASH_FLAG_ALPHA : u8 = 1;

const HASH_FLAG_BETA : u8 = 2;


static BISHOP_MAGIC_NUMBERS : [u64; 64] = [ 0x40040844404084u64,
    0x2004208a004208u64, 0x10190041080202u64, 0x108060845042010u64,
    0x581104180800210u64, 0x2112080446200010u64, 0x1080820820060210u64,
    0x3c0808410220200u64, 0x4050404440404u64, 0x21001420088u64,
    0x24d0080801082102u64, 0x1020a0a020400u64, 0x40308200402u64,
    0x4011002100800u64, 0x401484104104005u64, 0x801010402020200u64,
    0x400210c3880100u64, 0x404022024108200u64, 0x810018200204102u64,
    0x4002801a02003u64, 0x85040820080400u64, 0x810102c808880400u64,
    0xe900410884800u64, 0x8002020480840102u64, 0x220200865090201u64,
    0x2010100a02021202u64, 0x152048408022401u64, 0x20080002081110u64,
    0x4001001021004000u64, 0x800040400a011002u64, 0xe4004081011002u64,
    0x1c004001012080u64, 0x8004200962a00220u64, 0x8422100208500202u64,
    0x2000402200300c08u64, 0x8646020080080080u64, 0x80020a0200100808u64,
    0x2010004880111000u64, 0x623000a080011400u64, 0x42008c0340209202u64,
    0x209188240001000u64, 0x400408a884001800u64, 0x110400a6080400u64,
    0x1840060a44020800u64, 0x90080104000041u64, 0x201011000808101u64,
    0x1a2208080504f080u64, 0x8012020600211212u64, 0x500861011240000u64,
    0x180806108200800u64, 0x4000020e01040044u64, 0x300000261044000au64,
    0x802241102020002u64, 0x20906061210001u64, 0x5a84841004010310u64,
    0x4010801011c04u64, 0xa010109502200u64, 0x4a02012000u64,
    0x500201010098b028u64, 0x8040002811040900u64, 0x28000010020204u64,
    0x6000020202d0240u64, 0x8918844842082200u64, 0x4010011029020020u64 ];

static ROOK_MAGIC_NUMBERS : [u64; 64] = [ 0x8a80104000800020u64,
    0x140002000100040u64, 0x2801880a0017001u64, 0x100081001000420u64,
    0x200020010080420u64, 0x3001c0002010008u64, 0x8480008002000100u64,
    0x2080088004402900u64, 0x800098204000u64, 0x2024401000200040u64,
    0x100802000801000u64, 0x120800800801000u64, 0x208808088000400u64,
    0x2802200800400u64, 0x2200800100020080u64, 0x801000060821100u64,
    0x80044006422000u64, 0x100808020004000u64, 0x12108a0010204200u64,
    0x140848010000802u64, 0x481828014002800u64, 0x8094004002004100u64,
    0x4010040010010802u64, 0x20008806104u64, 0x100400080208000u64,
    0x2040002120081000u64, 0x21200680100081u64, 0x20100080080080u64,
    0x2000a00200410u64, 0x20080800400u64, 0x80088400100102u64,
    0x80004600042881u64, 0x4040008040800020u64, 0x440003000200801u64,
    0x4200011004500u64, 0x188020010100100u64, 0x14800401802800u64,
    0x2080040080800200u64, 0x124080204001001u64, 0x200046502000484u64,
    0x480400080088020u64, 0x1000422010034000u64, 0x30200100110040u64,
    0x100021010009u64, 0x2002080100110004u64, 0x202008004008002u64,
    0x20020004010100u64, 0x2048440040820001u64, 0x101002200408200u64,
    0x40802000401080u64, 0x4008142004410100u64, 0x2060820c0120200u64,
    0x1001004080100u64, 0x20c020080040080u64, 0x2935610830022400u64,
    0x44440041009200u64, 0x280001040802101u64, 0x2100190040002085u64,
    0x80c0084100102001u64, 0x4024081001000421u64, 0x20030a0244872u64,
    0x12001008414402u64, 0x2006104900a0804u64, 0x1004081002402u64
];

enum Side { WHITE, BLACK, BOTH }

enum Squares {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
    NoSq
}

static castling_rights : [u32; 64] = [
    7, 15, 15, 15, 3, 15, 15, 11,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 
    15, 15, 15, 15, 15, 15, 15, 15, 
    15, 15, 15, 15, 15, 15, 15, 15, 
    15, 15, 15, 15, 15, 15, 15, 15, 
    15, 15, 15, 15, 15, 15, 15, 15,
    13, 15, 15, 15, 12, 15, 15, 14
];

enum MoveTypes {AllMoves, OnlyCaptures}

enum CastleRights {WK = 1, WQ = 2, BK = 4, BQ = 8}

static BISHOP_RELEVANT_BITS : [u32; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5, 
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6
];

static ROOK_RELEVANT_BITS : [u32; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    12, 11, 11, 11, 11, 11, 11, 12
];

static NOT_A_FILE : u64 = 18374403900871474942;

static NOT_H_FILE : u64 = 9187201950435737471;

static NOT_HG_FILE : u64 = 4557430888798830399;

static NOT_AB_FILE : u64 = 18229723555195321596;

static ROW_7 : u64 = 71776119061217280;

static ROW_1 : u64 = 65280;

static MG_VALUE : [i32; 12] = [82, 337, 365, 477, 1025, 0, -82, -337, -365, -477,
-1025, 0];

static EG_VALUE : [i32; 12] = [94, 281, 297, 512, 936, 0, -94, -281, -297, -512, -936,
0];

static PHASE_INC : [u32; 6] = [0, 1, 1, 2, 4, 0];

static MG_PAWN_TABLE : [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0,				  
    98, 134, 61, 95, 68, 126, 34, -11,	  
    -6, 7, 26, 31, 65, 56, 25, -20,		  
    -14, 13, 6, 21, 23, 12, 17, -23,	  
    -27, -2, -5, 12, 17, 6, 10, -25,	  
    -26, -4, -4, -10, 3, 3, 33, -12,	  
    -35, -1, -20, -23, -15, 24, 38, -22,  
    0, 0, 0, 0, 0, 0, 0, 0,				  
];

static EG_PAWN_TABLE : [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0,					 
    178, 173, 158, 134, 147, 132, 165, 187,	 
    94, 100, 85, 67, 56, 53, 82, 84,		 
    32, 24, 13, 5, -2, 4, 17, 17,			 
    13, 9, -3, -7, -7, -8, 3, -1,			 
    4, 7, -6, 1, 0, -5, -1, -8,				 
    13, 8, 8, 10, 13, 0, 2, -7,				 
    0, 0, 0, 0, 0, 0, 0, 0,					 
];

static MG_KNIGHT_TABLE : [i32; 64] = [
    -167, -89, -34, -49, 61, -97, -15, -107,  //
    -73, -41, 72, 36, 23, 62, 7, -17,		  //
    -47, 60, 37, 65, 84, 129, 73, 44,		  //
    -9, 17, 19, 53, 37, 69, 18, 22,			  //
    -13, 4, 16, 13, 28, 19, 21, -8,			  //
    -23, -9, 12, 10, 19, 17, 25, -16,		  //
    -29, -53, -12, -3, -1, 18, -14, -19,	  //
    -105, -21, -58, -33, -17, -28, -19, -23,  //
];


static EG_KNIGHT_TABLE : [i32; 64] = [
    -58, -38, -13, -28, -31, -27, -63, -99,	 //
    -25, -8, -25, -2, -9, -25, -24, -52,	 //
    -24, -20, 10, 9, -1, -9, -19, -41,		 //
    -17, 3, 22, 22, 22, 11, 8, -18,			 //
    -18, -6, 16, 25, 16, 17, 4, -18,		 //
    -23, -3, -1, 15, 10, -3, -20, -22,		 //
    -42, -20, -10, -5, -2, -20, -23, -44,	 //
    -29, -51, -23, -15, -22, -18, -50, -64,	 //
];

static MG_BISHOP_TABLE : [i32; 64] = [
    -29, 4, -82, -37, -25, -42, 7, -8,		//
    -26, 16, -18, -13, 30, 59, 18, -47,		//
    -16, 37, 43, 40, 35, 50, 37, -2,		//
    -4, 5, 19, 50, 37, 37, 7, -2,			//
    -6, 13, 13, 26, 34, 12, 10, 4,			//
    0, 15, 15, 15, 14, 27, 18, 10,			//
    4, 15, 16, 0, 7, 21, 33, 1,				//
    -33, -3, -14, -21, -13, -12, -39, -21,	//
];

static EG_BISHOP_TABLE : [i32; 64] = [
    -14, -21, -11, -8, -7, -9, -17, -24,  //
    -8, -4, 7, -12, -3, -13, -4, -14,	  //
    2, -8, 0, -1, -2, 6, 0, 4,			  //
    -3, 9, 12, 9, 14, 10, 3, 2,			  //
    -6, 3, 13, 19, 7, 10, -3, -9,		  //
    -12, -3, 8, 10, 13, 3, -7, -15,		  //
    -14, -18, -7, -1, 4, -9, -15, -27,	  //
    -23, -9, -23, -5, -9, -16, -5, -17,	  //
];

static MG_ROOK_TABLE : [i32; 64] = [
    32, 42, 32, 51, 63, 9, 31, 43,		 //
    27, 32, 58, 62, 80, 67, 26, 44,		 //
    -5, 19, 26, 36, 17, 45, 61, 16,		 //
    -24, -11, 7, 26, 24, 35, -8, -20,	 //
    -36, -26, -12, -1, 9, -7, 6, -23,	 //
    -45, -25, -16, -17, 3, 0, -5, -33,	 //
    -44, -16, -20, -9, -1, 11, -6, -71,	 //
    -19, -13, 1, 17, 16, 7, -37, -26,	 //
];

static EG_ROOK_TABLE : [i32; 64] = [
    13, 10, 18, 15, 12, 12, 8, 5,	  //
    11, 13, 13, 11, -3, 3, 8, 3,	  //
    7, 7, 7, 5, 4, -3, -5, -3,		  //
    4, 3, 13, 1, 2, 1, -1, 2,		  //
    3, 5, 8, 4, -5, -6, -8, -11,	  //
    -4, 0, -5, -1, -7, -12, -8, -16,  //
    -6, -6, 0, 2, -9, -9, -11, -3,	  //
    -9, 2, 3, -1, -5, -13, 4, -20,	  //
];

static MG_QUEEN_TABLE : [i32; 64] = [
    -28, 0, 29, 12, 59, 44, 43, 45,		  //
    -24, -39, -5, 1, -16, 57, 28, 54,	  //
    -13, -17, 7, 8, 29, 56, 47, 57,		  //
    -27, -27, -16, -16, -1, 17, -2, 1,	  //
    -9, -26, -9, -10, -2, -4, 3, -3,	  //
    -14, 2, -11, -2, -5, 2, 14, 5,		  //
    -35, -8, 11, 2, 8, 15, -3, 1,		  //
    -1, -18, -9, 10, -15, -25, -31, -50,  //
];

static EG_QUEEN_TABLE : [i32; 64] = [
    -9, 22, 22, 27, 27, 19, 10, 20,			 //
    -17, 20, 32, 41, 58, 25, 30, 0,			 //
    -20, 6, 9, 49, 47, 35, 19, 9,			 //
    3, 22, 24, 45, 57, 40, 57, 36,			 //
    -18, 28, 19, 47, 31, 34, 39, 23,		 //
    -16, -27, 15, 6, 9, 17, 10, 5,			 //
    -22, -23, -30, -16, -16, -23, -36, -32,	 //
    -33, -28, -22, -43, -5, -32, -20, -41,	 //
];

static MG_KING_TABLE : [i32; 64] = [
    -65, 23, 16, -15, -56, -34, 2, 13,		 //
    29, -1, -20, -7, -8, -4, -38, -29,		 //
    -9, 24, 2, -16, -20, 6, 22, -22,		 //
    -17, -20, -12, -27, -30, -25, -14, -36,	 //
    -49, -1, -27, -39, -46, -44, -33, -51,	 //
    -14, -14, -22, -46, -44, -30, -15, -27,	 //
    1, 7, -8, -64, -43, -16, 9, 8,			 //
    -15, 36, 12, -54, 8, -28, 24, 14,		 //
];


static EG_KING_TABLE : [i32; 64] = [
    -74, -35, -18, -18, -11, 15, 4, -17,	//
    -12, 17, 14, 17, 17, 38, 23, 11,		//
    10, 17, 23, 15, 20, 45, 44, 13,			//
    -8, 22, 24, 27, 26, 33, 26, 3,			//
    -18, -4, 21, 24, 27, 23, 9, -11,		//
    -19, -3, 11, 21, 23, 16, 7, -9,			//
    -27, -11, 4, 13, 14, 4, -5, -17,		//
    -53, -34, -21, -11, -28, -14, -24, -43	//
];

static MVV_LVA : [[i32; 12] ; 12] = [
    [105, 205, 305, 405, 505, 605, 105, 205, 305, 405, 505, 605],
    [104, 204, 304, 404, 504, 604, 104, 204, 304, 404, 504, 604],
    [103, 203, 303, 403, 503, 603, 103, 203, 303, 403, 503, 603], 
    [102, 202, 302, 402, 502, 602, 102, 202, 302, 402, 502, 602],
    [101, 201, 301, 401, 501, 601, 101, 201, 301, 401, 501, 601],
    [100, 200, 300, 400, 500, 600, 100, 200, 300, 400, 500, 600],

    [105, 205, 305, 405, 505, 605, 105, 205, 305, 405, 505, 605], 
    [104, 204, 304, 404, 504, 604, 104, 204, 304, 404, 504, 604],
    [103, 203, 303, 403, 503, 603, 103, 203, 303, 403, 503, 603], 
    [102, 202, 302, 402, 502, 602, 102, 202, 302, 402, 502, 602],
    [101, 201, 301, 401, 501, 601, 101, 201, 301, 401, 501, 601],
    [100, 200, 300, 400, 500, 600, 100, 200, 300, 400, 500, 600]

];



static MG_PIECE_SCORES : [[i32; 64]; 6] = [
    MG_PAWN_TABLE, 
    MG_KING_TABLE,
    MG_BISHOP_TABLE,
    MG_ROOK_TABLE,
    MG_QUEEN_TABLE,
    MG_KING_TABLE
];

static EG_PIECE_SCORES : [[i32; 64]; 6] = [
    EG_PAWN_TABLE, 
    EG_KING_TABLE,
    EG_BISHOP_TABLE,
    EG_ROOK_TABLE,
    EG_QUEEN_TABLE,
    EG_KING_TABLE
];

static FULL_DEPTH_MOVES : u8 = 4;

static REDUCTION_LIMIT : u8 = 4;

static REDUCE : u8 = 3;

pub fn flip(sq : u32) -> u32 {
    return sq ^ 56;
}

pub fn pop(num : &mut u64, pos : u32) {
    *num &= !(1 << pos);
}

pub fn set(num : &mut u64, pos : u32) {
    *num |= 1 << pos;
}

pub fn get(num : u64, pos : u32) -> u64 {
    return num & (1 << pos);
}

macro_rules! copy_board {
    () => {
        let bitboards_copy = bitboards;
        let occupancies_copy = occupancies;
        let curr_zobrist_hash_copy = curr_zobrist_hash;
        let side_copy = side;
        let castle_rights_copy = castle_rights;
        let half_moves_copy = half_moves;
    };
}

macro_rules! restore_board {
    () => {
        bitboards = bitboards_copy;
        occupancies = occupancies_copy;
        curr_zobrist_hash = castle_rights_copy;
        side = side_copy;
        castle_rights = castle_rights_copy;
        half_moves = half_moves_copy;
    };
}

pub fn get_move_source(passed_move : u64) -> u64 {
    return passed_move & 0x3f;
}

pub fn get_move_target(passed_move : u64) -> u64 {
    return (passed_move & 0xfc0) >> 6;
}

pub fn get_move_piece(passed_move : u64) -> u64 {
    return (passed_move & 0xf000) >> 12;
}

pub fn get_move_promoted(passed_move : u64) -> u64 {
    return (passed_move & 0xf0000) >> 16;
}

pub fn get_move_capture(passed_move : u64) -> u64 {
    return passed_move & 0x100000;
}

pub fn get_move_double(passed_move : u64) -> u64 {
    return passed_move & 0x200000;
}

pub fn get_move_en_passant(passed_move : u64) -> u64 {
    return passed_move & 0x400000;
}

pub fn get_move_castling(passed_move : u64) -> u64 {
    return passed_move & 0x800000;
}

pub fn get_move_capture_piece(passed_move : u64) -> u64 {
    return (passed_move & 0xf000000) >> 24;
}




#[derive(Copy)]
#[derive(Clone)]
pub struct Hash {
    key : u64,
    depth : u32,
    flags : u8,
    value : i32,
    best_move : u64,
}

impl Hash {
    pub fn default() -> Hash {
        Hash {
            key : 0,
            depth : 0,
            flags : 0,
            value : 0,
            best_move : 0,
        }
    }
}


pub struct Moves {
    moves: [u64; 256],
    count: i32,
}

pub struct Board {
    side : i32,
    nodes : u64,
    bitboards : [u64 ; 12],
    occupancies : [u64; 3],
    castle_rights : u32,
    half_moves : u32,
    move_index : u32,
    pawn_attacks : [[u64; 2]; 64],
    knight_attacks : [u64; 64],
    king_attacks : [u64; 64],
    bishop_masks : [u64; 64],
    rook_masks : [u64; 64],
    bishop_attacks : [[u64; 64]; 512],
    rook_attacks : [[u64; 64]; 4096],
    en_passant_sq : Squares,
    move_log : [u64; 512],
    move_log_pgn : [String ; 512],
    ply : u32,
    killer_moves : [[u64; 2]; MAX_PLY as usize],
    history_moves : [[i32; 12]; MAX_PLY as usize],
    pv_length : [u64; MAX_PLY as usize],
    pv_table : [[u64; MAX_PLY as usize]; MAX_PLY as usize],
    follow_pv : i32,
    score_pv : i32,
    curr_zobrist_hash : u64,
    zobrist_keys : [[u64 ; 12]; 64],
    en_passant_zobrist : [u64; 64],
    zobrist_side_key : u64,
    transposition_table : [Hash ; HASH_SIZE as usize],
    null_move_made : bool,
    repetition_table : [u64; 512],
    repetition_index : i32,
    file_masks : [u64; 64],
    rank_masks : [u64; 64],
    isolated_pawn_masks : [u64; 64],
    white_passed_pawn_masks : [u64; 64],
    black_passed_pawn_masks : [u64; 64],
    double_pawn_penalty : i32,
    isolated_pawn_penalty : i32,
    passed_pawn_bonus : [i32; 8],
    semi_open_file_score : i32,
    open_file_score : i32,
    is_checkmate : bool,
    is_stalemate : bool,
    player_moves : Moves,
}

impl Board {

    pub fn new(fen : String) -> Board {
        let board = Board {
            side : 0,
            nodes : 0,
            bitboards : [0; 12],
            occupancies : [0; 3],
            castle_rights : 0,
            half_moves : 0,
            move_index : 0,
            pawn_attacks : [[0; 2]; 64],
            knight_attacks : [0; 64],
            king_attacks : [0; 64],
            bishop_masks : [0; 64],
            rook_masks : [0; 64],
            bishop_attacks : [[0; 64]; 512],
            rook_attacks : [[0; 64]; 4096],
            en_passant_sq : NoSq,
            move_log : [0; 512],
            move_log_pgn : [EMPTY_STRING; 512],
            ply : 0,
            killer_moves : [[0, 0]; MAX_PLY as usize],
            history_moves : [[0; 12]; MAX_PLY as usize],
            pv_length: [0; MAX_PLY as usize],
            pv_table: [[0; MAX_PLY as usize]; MAX_PLY as usize],
            follow_pv : 0,
            score_pv : 0,
            curr_zobrist_hash : 0,
            zobrist_keys : [[0; 12]; 64],
            en_passant_zobrist : [0; 64],
            zobrist_side_key : 0,
            transposition_table : [Hash::default(); HASH_SIZE as usize],
            null_move_made : false,
            repetition_table : [0; 512],
            repetition_index : 0,
            file_masks : [0; 64],
            rank_masks : [0; 64],
            isolated_pawn_masks : [0; 64],
            white_passed_pawn_masks : [0; 64],
            black_passed_pawn_masks : [0; 64],
            double_pawn_penalty : -10,
            isolated_pawn_penalty : -10,
            semi_open_file_score : 10,
            open_file_score : 25,
            is_checkmate : false,
            is_stalemate : false,
            player_moves : Moves { moves: [0; 256], count: 0 },
            passed_pawn_bonus : [0, 5, 15, 20, 50, 70, 110, 160],
        };


        // Init here


        return board;
    }

    

}

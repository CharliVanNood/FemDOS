static CHARACTER_SIZE: usize = 5 * 7;

pub static CHARACTERS: [[bool; CHARACTER_SIZE]; 100] = [
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE],
    
    [ // character A
        false, false, false, false, false,
        false, false, true,  false, false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  true,  true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
    ], 
    [ // character B
        false, false, false, false, false,
        false, true,  true,  false, false,
        false, true,  false, true,  false,
        false, true,  true,  false, false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  true,  true,  false,
    ],
    [ // character C
        false, false, false, false, false,
        false, false, true,  true,  false,
        false, true,  false, false, false,
        false, true,  false, false, false,
        false, true,  false, false, false,
        false, true,  false, false, false,
        false, false, true,  true,  false,
    ],
    [ // character D
        false, false, false, false, false,
        false, true,  true,  false, false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  true,  true,  false,
    ], 
];
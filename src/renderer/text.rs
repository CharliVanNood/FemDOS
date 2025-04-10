static CHARACTER_SIZE: usize = 5 * 7;

pub static CHARACTERS: [[bool; CHARACTER_SIZE]; 123] = [
    // these characters are empty characters for padding, you're allowed to add them, it's so it lines up with vga
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],

    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],

    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],

    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],

    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [ // character -
        false, false, false, false, false,
        false, false, false, false, false,
        false, false, false, false, false,
        true,  true,  true,  true,  false,
        false, false, false, false, false,
        false, false, false, false, false,
        false, false, false, false, false,
    ],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [ // character 0
        false, false, false, false, false,
        false, false, true,  false, false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, false, true,  false, false,
    ],
    [ // character 1
        false, false, false, false, false,
        false, false, true,  false, false,
        false, true,  true,  false, false,
        false, false, true,  false, false,
        false, false, true,  false, false,
        false, false, true,  false, false,
        false, true,  true,  true,  false,
    ],
    [ // character 2
        false, false, false, false, false,
        false, true,  true,  false, false,
        false, false, false, true,  false,
        false, false, false, true,  false,
        false, false, true,  false, false,
        false, true,  false, false, false,
        false, true,  true,  true,  false,
    ],
    [ // character 3
        false, false, false, false, false,
        false, true,  true,  false, false,
        false, false, false, true,  false,
        false, true,  true,  false, false,
        false, false, false, true,  false,
        false, false, false, true,  false,
        false, true,  true,  false, false,
    ],
    [ // character 4
        false, false, false, false, false,
        false, true,  true,  false, false,
        true,  false, true,  false, false,
        true,  false, true,  false, false,
        true,  true,  true,  true,  false,
        false, false, true,  false, false,
        false, false, true,  false, false,
    ],
    [ // character 5
        false, false, false, false, false,
        false, true,  true,  true,  false,
        false, true,  false, false, false,
        false, true,  true,  false, false,
        false, false, false, true,  false,
        false, false, false, true,  false,
        false, true,  true,  false, false,
    ],
    [ // character 6
        false, false, false, false, false,
        false, false, true,  true,  false,
        false, true,  false, false, false,
        false, true,  true,  false, false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, false, true,  false, false,
    ],
    [ // character 7
        false, false, false, false, false,
        false, true,  true,  true,  false,
        false, false, false, true,  false,
        false, false, false, true,  false,
        false, false, true,  false, false,
        false, false, true,  false, false,
        false, false, true,  false, false,
    ],
    [ // character 8
        false, false, false, false, false,
        false, true,  true,  false, false,
        true,  false, false, true,  false,
        false, true,  true,  false, false,
        true,  false, false, true,  false,
        true,  false, false, true,  false,
        false, true,  true,  false, false,
    ],
    [ // character 9
        false, false, false, false, false,
        false, false, true,  true,  false,
        false, true,  false, true,  false,
        false, true,  true,  true,  false,
        false, false, false, true,  false,
        false, false, false, true,  false,
        false, true,  true,  false, false,
    ],

    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], 

    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [ // character >
        false, false, false, false, false,
        false, true,  false, false, false,
        false, false, true,  false, false,
        false, false, false, true,  false,
        false, false, true,  false, false,
        false, true,  false, false, false,
        false, false, false, false, false,
    ],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],

    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],

    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    [false; CHARACTER_SIZE], [false; CHARACTER_SIZE],
    
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
        false, true,  true,  false, false,
    ],
    [ // character E
        false, false, false, false, false,
        false, true,  true,  true,  false,
        false, true,  false, false, false,
        false, true,  true,  true,  false,
        false, true,  false, false, false,
        false, true,  false, false, false,
        false, true,  true,  true,  false,
    ],
    [ // character F
        false, false, false, false, false,
        false, true,  true,  true,  false,
        false, true,  false, false, false,
        false, true,  true,  true,  false,
        false, true,  false, false, false,
        false, true,  false, false, false,
        false, true,  false, false, false,
    ],
    [ // character G
        false, false, false, false, false,
        false, true,  true,  true,  false,
        false, true,  false, false, false,
        false, true,  false, false, false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  true,  true,  false,
    ],
    [ // character H
        false, false, false, false, false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  true,  true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
    ],
    [ // character I
        false, false, false, false, false,
        false, true,  true,  true,  false,
        false, false, true,  false, false,
        false, false, true,  false, false,
        false, false, true,  false, false,
        false, false, true,  false, false,
        false, true,  true,  true,  false,
    ],
    [ // character J
        false, false, false, false, false,
        false, true,  true,  true,  false,
        false, false, true,  false, false,
        false, false, true,  false, false,
        false, false, true,  false, false,
        false, false, true,  false, false,
        false, true,  true,  false, false,
    ],
    [ // character K
        false, false, false, false, false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  true,  false, false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
    ],
    [ // character L
        false, false, false, false, false,
        false, true,  false, false, false,
        false, true,  false, false, false,
        false, true,  false, false, false,
        false, true,  false, false, false,
        false, true,  false, false, false,
        false, true,  true,  true,  false,
    ],
    [ // character M
        false, false, false, false, false,
        true, true,   false, true,  false,
        true, false,  true,  false, true,
        true, false,  true,  false, true,
        true, false,  true,  false, true,
        true, false,  true,  false, true,
        true, false,  true,  false, true,
    ],
    [ // character N
        false, false, false, false, false,
        false, true,  true,  false, false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
    ],
    [ // character O
        false, false, false, false, false,
        false, true,  true,  false, false,
        true,  false, false, true,  false,
        true,  false, false, true,  false,
        true,  false, false, true,  false,
        true,  false, false, true,  false,
        false, true,  true,  false, false,
    ],
    [ // character P
        false, false, false, false, false,
        false, true,  true,  false, false,
        false, true,  false, true,  false,
        false, true,  true,  false, false,
        false, true,  false, false, false,
        false, true,  false, false, false,
        false, true,  false, false, false,
    ],
    [ // character Q
        false, false, false, false, false,
        false, false, true,  false, false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, false, true,  false, true,
    ],
    [ // character R
        false, false, false, false, false,
        false, true,  true,  false, false,
        false, true,  false, true,  false,
        false, true,  true,  false, false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
    ],
    [ // character S
        false, false, false, false, false,
        false, false, true,  true,  false,
        false, true,  false, false, false,
        false, true,  true,  false, false,
        false, false, false, true,  false,
        false, false, false, true,  false,
        false, true,  true,  false, false,
    ],
    [ // character T
        false, false, false, false, false,
        true,  true,  true,  true,  true,
        false, false, true,  false, false,
        false, false, true,  false, false,
        false, false, true,  false, false,
        false, false, true,  false, false,
        false, false, true,  false, false,
    ],
    [ // character U
        false, false, false, false, false,
        false, true,  false, true, false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  true,  false, false,
    ],
    [ // character V
        false, false, false, false, false,
        false, true,  false, true, false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, false, true,  false, false,
    ],
    [ // character W
        false, false, false, false, false,
        true,  false, true,  false, true,
        true,  false, true,  false, true,
        true,  false, true,  false, true,
        true,  false, true,  false, true,
        true,  false, true,  false, true,
        false, true,  false, true,  false,
    ],
    [ // character X
        false, false, false, false, false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, false, true,  false, false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
    ],
    [ // character Y
        false, false, false, false, false,
        false, true,  false, true,  false,
        false, true,  false, true,  false,
        false, false, true,  false, false,
        false, false, true,  false, false,
        false, false, true,  false, false,
        false, false, true,  false, false,
    ],
    [ // character Z
        false, false, false, false, false,
        true,  true,  true,  true,  false,
        false, false, false, true,  false,
        false, false, true,  false, false,
        false, true,  false, false, false,
        true,  false, false, false, false,
        true,  true,  true,  true,  false,
    ],
];
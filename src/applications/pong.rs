use crate::{print, window};

pub struct Pong {
    _ball_velocity: [i8; 2],
    ball_position: [i8; 2],
    pos_left: i8,
    _pos_right: i8,
}
impl Pong {
    pub fn init() -> Self {
        Self {
            _ball_velocity: [2, 1],
            ball_position: [0; 2],
            pos_left: 0,
            _pos_right: 0,
        }
    }

    pub fn render(&self) {
        for y in 0..24 {
            for x in 0..80 {
                if x == 1 && (y - 12 < self.pos_left + 4 && y - 12 > self.pos_left - 4) || 
                    x == 78 && (y - 12 < self.pos_left + 4 && y - 12 > self.pos_left - 4) || 
                    (x - 39 == self.ball_position[0] && y - 12 == self.ball_position[1]) {
                    window::set_terminal_color(15, 15);
                } else { window::set_terminal_color(15, 13); }
                print!(" ");
            }
        }
        window::set_terminal_color(13, 15);
    }
}

pub fn play() {
    let game = Pong::init();
    game.render();
}
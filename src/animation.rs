use macroquad::prelude::*;

pub const TIMESTEP: u128 = 500; // ms

pub const SPRITE: f32 = 16.0;
pub const FRAMES: usize = 2;

pub const SCALE: f32 = 4.0;
pub const UNIT: f32 = SPRITE * SCALE;
pub const SIZE: Vec2 = vec2(UNIT, UNIT);

pub const fn get_source_rect(x: usize, y: usize) -> Rect {
    Rect::new(SPRITE * x as f32, SPRITE * y as f32, SPRITE, SPRITE)
}

// generate an array of states for a single sprite sheet

#[derive(Copy, Clone, PartialEq)]
pub enum PlayerState {
    Sit,
    Hold,
    Idle,
    Walk,
    Hit,
    Raise,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub struct PlayerAnimation {
    state: PlayerState,
    dir: Dir,
    frame: usize,
}

impl PlayerAnimation {
    pub fn new(state: PlayerState, dir: Dir) -> PlayerAnimation {
        PlayerAnimation {
            state,
            dir,
            frame: 0,
        }
    }

    fn step(&mut self) {
        self.frame = (self.frame + 1) % FRAMES;
    }

    pub fn update(&mut self, state: PlayerState, dir: Dir) {
        if self.state != state || self.dir != dir {
            *self = Self::new(state, dir);
        } else {
            self.step();
        }
    }

    pub fn draw(&self, sprite: &Texture2D, pos: Vec2) {
        let column = self.frame
            + match self.dir {
                Dir::Up => 2,
                Dir::Down => 0,
                Dir::Left | Dir::Right => 4,
            };
        let params = DrawTextureParams {
            dest_size: Some(SIZE),
            source: Some(get_source_rect(column, self.state as usize)),
            rotation: 0.0,
            flip_x: self.dir == Dir::Left,
            flip_y: false,
            pivot: None,
        };
        draw_texture_ex(sprite, pos.x, pos.y, WHITE, params);
    }
}

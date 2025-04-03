use crate::qtree::*;
use macroquad::experimental::animation::*;
use macroquad::prelude::*;

pub const TIMESTEP: u128 = 500; // ms

pub const SPRITE: f32 = 16.0;
pub const FRAMES: usize = 2;

pub const SCALE: f32 = 4.0;
pub const UNIT: f32 = SPRITE * SCALE;

pub const SIZE: Vec2 = Vec2::splat(UNIT);
const HALF_SIZE: Vec2 = Vec2::splat(UNIT / 2.0);

pub const fn get_source_rect(x: usize, y: usize) -> Rect {
    Rect::new(SPRITE * x as f32, SPRITE * y as f32, SPRITE, SPRITE)
}

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

struct Player {
    image: Texture2D,
    sprite: AnimatedSprite,
    state: PlayerState,
    dir: Dir,
    pos: Vec2,
}

impl Positioned for Player {
    fn pos(&self) -> Vec2 {
        self.pos
    }
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
        let pos = pos - HALF_SIZE;
        let column = self.frame
            + match self.dir {
                Dir::Up => 2,
                Dir::Down => 0,
                Dir::Left | Dir::Right => 4,
            };
        let params = DrawTextureParams {
            dest_size: Some(SIZE),
            source: Some(get_source_rect(column, self.state as usize)),
            flip_x: self.dir == Dir::Left,
            ..Default::default()
        };
        draw_texture_ex(sprite, pos.x, pos.y, WHITE, params);
    }
}

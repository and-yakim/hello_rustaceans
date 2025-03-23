use super::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Quadrant {
    pub fn new(region: &Rect, pos: Vec2) -> Self {
        let center = region.center();
        if pos.x < center.x {
            if pos.y < center.y {
                Quadrant::TopLeft
            } else {
                Quadrant::BottomLeft
            }
        } else {
            if pos.y < center.y {
                Quadrant::TopRight
            } else {
                Quadrant::BottomRight
            }
        }
    }
}

#[repr(transparent)]
pub struct Square(Rect);

impl Square {
    pub fn new(x: f32, y: f32, size: f32) -> Self {
        Square(Rect {
            x,
            y,
            w: size,
            h: size,
        })
    }

    pub fn modify(&self, move_to: Vec2, scale: f32) -> Self {
        Self::new(move_to.x, move_to.y, self.w * scale)
    }
}

impl Deref for Square {
    type Target = Rect;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Square {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Rect> for Square {
    fn from(rect: Rect) -> Self {
        Square(rect)
    }
}

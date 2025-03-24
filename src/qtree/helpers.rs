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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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

    pub fn modify(&self, diff: Vec2, size: f32) -> Self {
        Self::new(self.x + diff.x, self.y + diff.y, size)
    }

    pub fn zero(pos: Vec2) -> Self {
        Self::new(pos.x, pos.y, 0.0)
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

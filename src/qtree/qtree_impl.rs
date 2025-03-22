use macroquad::math::{Rect, Vec2};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

pub trait Positioned {
    fn pos(&self) -> Vec2;
}

#[derive(Copy, Clone, PartialEq)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
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

#[derive(Clone, Debug)]
pub enum QTreeMut<T: Clone + Positioned> {
    BlankNode {
        region: Rect,
        children: Vec<QTreeMut<T>>,
    },
    ValueNode {
        region: Rect,
        values: Vec<T>,
    },
}

impl<T: Clone + Positioned> QTreeMut<T> {
    pub fn new(region: Rect, values: Vec<T>) -> Self {
        Self::ValueNode { region, values }
    }

    // pub fn from(tree: QTree<T>) -> Self {}

    pub fn region(&self) -> Rect {
        match self {
            Self::BlankNode { region, .. } => *region,
            Self::ValueNode { region, .. } => *region,
        }
    }

    fn cell_size0(&self, size: f32) -> f32 {
        match self {
            Self::BlankNode { children, .. } => {
                let arr = children
                    .iter()
                    .map(|node| node.cell_size0(self.region().w / 2.0));
                arr.reduce(f32::min).unwrap_or(self.region().w)
            }
            Self::ValueNode { .. } => self.region().w,
        }
    }

    pub fn cell_size(&self) -> f32 {
        self.cell_size0(self.region().w)
    }

    // pub fn resize(self, rect: Rect) -> Self {
    //     match self {
    //         Self::BlankNode { children, .. } => Self::BlankNode {
    //             region: rect,

    //             children: children
    //                 .iter()
    //                 .map(|node| node.to_owned().resize(rect))
    //                 .collect(),
    //         },
    //         Self::ValueNode { values, .. } => Self::ValueNode {
    //             region: rect,

    //             values,
    //         },
    //     }
    // }

    pub fn get_values(&self, addres: Vec<usize>) -> Vec<T> {
        Vec::new()
    }

    fn expand_to_contain(&mut self, pos: Vec2) {
        let region = self.region();
        let point = region.point();

        let (treat_as, rect) = if pos.x > point.x {
            if pos.y > point.y {
                (
                    Quadrant::TopLeft,
                    Rect::new(region.x, region.y, region.w * 2.0, region.h * 2.0),
                )
            } else {
                (
                    Quadrant::BottomLeft,
                    Rect::new(
                        region.x,
                        region.y - region.h,
                        region.w * 2.0,
                        region.h * 2.0,
                    ),
                )
            }
        } else {
            if pos.y > point.y {
                (
                    Quadrant::TopRight,
                    Rect::new(
                        region.x - region.w,
                        region.y,
                        region.w * 2.0,
                        region.h * 2.0,
                    ),
                )
            } else {
                (
                    Quadrant::BottomRight,
                    Rect::new(
                        region.x - region.w,
                        region.y - region.h,
                        region.w * 2.0,
                        region.h * 2.0,
                    ),
                )
            }
        };

        if let Self::BlankNode {
            region,
            mut children,
        } = Self::new(rect, Vec::new()).split()
        {
            children[treat_as as usize] = self.clone();
            *self = Self::BlankNode { region, children };
        };
    }

    fn add0(&mut self, value: T) {
        match self {
            Self::BlankNode { region, children } => {}
            Self::ValueNode { region, values } => {}
        }
    }

    pub fn add(&mut self, value: T) {
        while !self.region().contains(value.pos()) {
            self.expand_to_contain(value.pos());
        }
    }

    pub fn remove(&mut self, value: T) {}

    fn split(self) -> Self {
        match self {
            Self::BlankNode { .. } => self,
            Self::ValueNode { region, values } => {
                let children = Self::split_values(&region, values)
                    .iter()
                    .map(|(reg, val)| Self::ValueNode {
                        region: *reg,

                        values: val.to_vec(),
                    })
                    .collect();
                Self::BlankNode {
                    region: region,

                    children,
                }
            }
        }
    }

    fn split_values(region: &Rect, values: Vec<T>) -> [(Rect, Vec<T>); 4] {
        let (half_w, half_h) = (region.w / 2.0, region.h / 2.0);
        let (center_x, center_y) = (region.x + half_w, region.y + half_h);

        let mut top_left = Vec::new();
        let mut top_right = Vec::new();
        let mut bottom_left = Vec::new();
        let mut bottom_right = Vec::new();

        for value in values {
            let pos = &value.pos();
            if pos.x < center_x {
                if pos.y < center_y {
                    top_left.push(value);
                } else {
                    bottom_left.push(value);
                }
            } else {
                if pos.y < center_y {
                    top_right.push(value);
                } else {
                    bottom_right.push(value);
                }
            }
        }

        [
            (Rect::new(region.x, region.y, half_w, half_h), top_left),
            (
                Rect::new(region.x + half_w, region.y, half_w, half_h),
                top_right,
            ),
            (
                Rect::new(region.x, region.y + half_h, half_w, half_h),
                bottom_left,
            ),
            (
                Rect::new(region.x + half_w, region.y + half_h, half_w, half_h),
                bottom_right,
            ),
        ]
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Rect")]
struct RectDef {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct IndexedNode<T: Clone + Positioned> {
    #[serde(with = "RectDef")]
    region: Rect,
    parent: u16,
    children: Option<[u16; 4]>,
    values: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QTree<T: Clone + Positioned> {
    arr: Vec<IndexedNode<T>>,
}

impl<T: Clone + Positioned> QTree<T> {
    fn new(tree: QTreeMut<T>) -> Self {
        // vec![root, root.0, root.1, root.2, root.3, *layer3*, .. ]
        QTree { arr: Vec::new() }
    }
}

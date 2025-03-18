use macroquad::math::{Rect, Vec2};
use serde::{Deserialize, Serialize};

pub trait Positioned {
    fn pos(&self) -> Vec2;
}

// #[derive(Copy, Clone, PartialEq)]
// pub enum Dir {
//     Up,
//     Down,
//     Left,
//     Right,
// }

#[derive(Copy, Clone, PartialEq)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
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

    fn depth0(&self, depth: u8) -> u8 {
        match self {
            Self::BlankNode { children, .. } => {
                let arr = children.iter().map(|node| node.depth0(depth + 1));
                arr.max().unwrap_or(depth)
            }
            Self::ValueNode { .. } => depth,
        }
    }

    pub fn depth(&self) -> u8 {
        self.depth0(0)
    }

    pub fn resize(self, rect: Rect) -> Self {
        match self {
            Self::BlankNode { children, .. } => Self::BlankNode {
                region: rect,

                children: children
                    .iter()
                    .map(|node| node.to_owned().resize(rect))
                    .collect(),
            },
            Self::ValueNode { values, .. } => Self::ValueNode {
                region: rect,

                values,
            },
        }
    }

    // pub fn get_values(&self, addres: Vec<usize>) -> Vec<T> {}

    // pub fn add(&mut self, value: T) {
    //     match self {
    //         Self::BlankNode {
    //             region,

    //             children,
    //         } => {}
    //         Self::ValueNode {
    //             region,

    //             values,
    //         } => {}
    //     }
    // }

    // pub fn remove(&mut self, value: T) {}

    pub fn split_by_click(self, click: Vec2) -> Self {
        match self {
            Self::BlankNode {
                region,

                mut children,
            } => {
                let center = region.center();
                let i = if click.x < center.x {
                    if click.y < center.y {
                        Quadrant::TopLeft
                    } else {
                        Quadrant::BottomLeft
                    }
                } else {
                    if click.y < center.y {
                        Quadrant::TopRight
                    } else {
                        Quadrant::BottomRight
                    }
                };
                children[i as usize] = Self::split_by_click(children[i as usize].clone(), click);
                Self::BlankNode { region, children }
            }
            Self::ValueNode { .. } => self.split(),
        }
    }

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

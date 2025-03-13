use macroquad::math::{Rect, Vec2};
use serde::{Deserialize, Serialize};

pub trait Positioned {
    fn pos(&self) -> Vec2;
}

#[derive(Clone, Debug)]
pub enum QTreeMut<T: Clone + Positioned> {
    BlankNode {
        region: Rect,
        depth: u8,
        children: Vec<QTreeMut<T>>,
    },
    ValueNode {
        region: Rect,
        depth: u8,
        values: Vec<T>,
    },
}

impl<T: Clone + Positioned> QTreeMut<T> {
    pub fn new(region: Rect, values: Vec<T>) -> QTreeMut<T> {
        QTreeMut::ValueNode {
            region,
            depth: 0,
            values,
        }
    }

    fn add(&mut self, value: T) {}

    pub fn split(self) -> QTreeMut<T> {
        match self {
            QTreeMut::BlankNode { .. } => self,
            QTreeMut::ValueNode {
                region,
                depth,
                values,
            } => {
                let children = Self::split_values(&region, values)
                    .iter()
                    .map(|(reg, val)| QTreeMut::ValueNode {
                        region: *reg,
                        depth: depth + 1,
                        values: val.to_vec(),
                    })
                    .collect();
                QTreeMut::BlankNode {
                    region: region,
                    depth: depth,
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

#[derive(Serialize, Deserialize, Debug)]
struct IndexedNode<T: Clone + Positioned> {
    parent: u16,
    children: Option<[u16; 4]>,
    values: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QTree<T: Clone + Positioned> {
    tree: Vec<IndexedNode<T>>,
}

impl<T: Clone + Positioned> QTree<T> {
    fn new(tree: QTreeMut<T>) -> QTree<T> {
        // vec![root, root.0, root.1, root.2, root.3, *layer3*, .. ]
        QTree { tree: Vec::new() }
    }
}

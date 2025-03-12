use macroquad::math::{Rect, Vec2};

pub trait Positioned {
    fn pos(&self) -> Vec2;
}

#[derive(Clone, Debug)]
pub enum QTreeMut<T: Clone + Positioned> {
    BlankNode {
        region: Rect,
        depth: usize,
        children: Vec<QTreeMut<T>>,
    },
    ValueNode {
        region: Rect,
        depth: usize,
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
            let coords = &value.pos();
            if coords.x < center_x {
                if coords.y < center_y {
                    top_left.push(value);
                } else {
                    bottom_left.push(value);
                }
            } else {
                if coords.y < center_y {
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

use super::helpers::*;
use super::*;
use serde::{Deserialize, Serialize};

pub trait Positioned {
    fn pos(&self) -> Vec2;
}

#[derive(Clone, Debug)]
pub enum QTreeMut<T: Clone + Positioned> {
    BlankNode {
        region: Square,
        children: Vec<QTreeMut<T>>,
    },
    ValueNode {
        region: Square,
        values: Vec<T>,
    },
}

impl<T: Clone + Positioned> QTreeMut<T> {
    pub fn new(region: Square, values: Vec<T>) -> Self {
        Self::ValueNode { region, values }
    }

    // pub fn from(tree: QTree<T>) -> Self {}

    pub fn region(&self) -> Square {
        match self {
            Self::BlankNode { region, .. } => *region,
            Self::ValueNode { region, .. } => *region,
        }
    }

    fn cell_size(&self) -> f32 {
        match self {
            Self::BlankNode { children, .. } => {
                let arr = children.iter().map(|node| node.cell_size());
                arr.reduce(f32::min).unwrap_or(self.region().w)
            }
            Self::ValueNode { .. } => self.region().w,
        }
    }

    fn blank_children(region: Square) -> Vec<Self> {
        let size = region.w / 2.0;
        [
            region.modify(Vec2::ZERO, size),
            region.modify(vec2(size, 0.0), size),
            region.modify(vec2(0.0, size), size),
            region.modify(vec2(size, size), size),
        ]
        .map(|rect| Self::new(rect, Vec::new()))
        .to_vec()
    }

    fn expand_to_contain(&mut self, pos: Vec2) {
        let region = self.region();
        let treat_as = Quadrant::new(&Square::zero(pos), region.center());

        let rect = match treat_as {
            Quadrant::TopLeft => region.modify(Vec2::ZERO, region.w * 2.0),
            Quadrant::TopRight => region.modify(vec2(-region.w, 0.0), region.w * 2.0),
            Quadrant::BottomLeft => region.modify(vec2(0.0, -region.h), region.w * 2.0),
            Quadrant::BottomRight => region.modify(vec2(-region.w, -region.h), region.w * 2.0),
        };

        let mut children = Self::blank_children(rect);

        children[treat_as as usize] = self.clone();
        *self = Self::BlankNode {
            region: rect,
            children,
        };
    }

    fn add0(&mut self, value: T, target_size: f32) {
        match self {
            Self::BlankNode { region, children } => {
                let i = Quadrant::new(region, value.pos()) as usize;
                children[i].add0(value, target_size);
            }
            Self::ValueNode { region, values } => {
                if region.w > target_size {
                    let mut children = Self::blank_children(*region);
                    let i = Quadrant::new(region, value.pos()) as usize;
                    children[i].add0(value, target_size);
                    *self = Self::BlankNode {
                        region: *region,
                        children,
                    }
                } else {
                    values.push(value);
                }
            }
        }
    }

    pub fn add(&mut self, value: T) {
        while !self.region().contains(value.pos()) {
            self.expand_to_contain(value.pos());
        }
        let target_size = self.cell_size();
        self.add0(value, target_size)
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

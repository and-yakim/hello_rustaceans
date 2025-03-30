use super::helpers::*;
use super::*;

pub trait Positioned {
    fn pos(&self) -> Vec2;

    fn coords(&self, cell: f32) -> IVec2 {
        let pos = self.pos();
        ivec2((pos.x / cell).round() as i32, (pos.y / cell).round() as i32)
    }
}

impl<T: Copy + Into<Vec2>> Positioned for T {
    fn pos(&self) -> Vec2 {
        (*self).into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum QTreeMut<T: Clone + Positioned> {
    Node {
        region: Square,
        children: Vec<QTreeMut<T>>,
    },
    Leaf {
        region: Square,
        values: Vec<T>,
    },
}

impl<T: Clone + Positioned> QTreeMut<T> {
    pub fn new(region: Square, values: Vec<T>) -> Self {
        Self::Leaf { region, values }
    }

    // pub fn from(tree: QTree<T>) -> Self {}

    pub fn region(&self) -> Square {
        match self {
            Self::Node { region, .. } => *region,
            Self::Leaf { region, .. } => *region,
        }
    }

    fn cell_size(&self) -> f32 {
        match self {
            Self::Node { children, .. } => {
                let arr = children.iter().map(|node| node.cell_size());
                arr.reduce(f32::min).unwrap_or(self.region().w)
            }
            Self::Leaf { .. } => self.region().w,
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
        *self = Self::Node {
            region: rect,
            children,
        };
    }

    fn get_values(&self, pos: Vec2) -> Vec<T> {
        match self {
            Self::Node { region, children } => {
                let i = Quadrant::new(region, pos) as usize;
                children[i].get_values(pos)
            }
            Self::Leaf { values, .. } => values.to_vec(),
        }
    }

    fn add0(&mut self, value: T, target_size: f32) {
        match self {
            Self::Node { region, children } => {
                let i = Quadrant::new(region, value.pos()) as usize;
                children[i].add0(value, target_size);
            }
            Self::Leaf { region, values } => {
                if region.w > target_size {
                    let mut children = Self::blank_children(*region);
                    let i = Quadrant::new(region, value.pos()) as usize;
                    children[i].add0(value, target_size);
                    *self = Self::Node {
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

    pub fn size(&self) -> usize {
        match self {
            Self::Node { children, .. } => children.iter().map(Self::size).sum::<usize>() + 1,
            Self::Leaf { .. } => 1,
        }
    }
}

use macroquad::math::Rect;

#[derive(Clone, Debug)]
pub enum QTree<T> {
    BlankNode {
        region: Rect,
        depth: usize,
        children: Box<[QTree<T>; 4]>,
    },
    ValueNode {
        region: Rect,
        depth: usize,
        values: Vec<T>,
    },
}

impl<T> QTree<T> {
    pub fn new(region: Rect) -> QTree<T> {
        QTree::BlankNode {
            region,
            depth: 0,
            children: Self::region_to_children(region, 0),
        }
    }

    fn region_to_children(region: Rect, depth: usize) -> Box<[QTree<T>; 4]> {
        let (half_w, half_h) = (region.w / 2.0, region.h / 2.0);
        let split = [
            Rect::new(region.x, region.y, half_w, half_h),
            Rect::new(region.x + half_w, region.y, half_w, half_h),
            Rect::new(region.x, region.y + half_h, half_w, half_h),
            Rect::new(region.x + half_w, region.y + half_h, half_w, half_h),
        ];
        Box::new(split.map(|rect| QTree::ValueNode {
            region: rect,
            depth: depth + 1,
            values: Vec::new(),
        }))
    }
}

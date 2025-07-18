use crate::utils::math::Vec2;

const MAX_PARTICLES: usize = 10;
const MAX_DEPTH: usize = 8;

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Bounds {
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.x &&
        point.x <= self.x + self.width &&
        point.y >= self.y &&
        point.y <= self.y + self.height
    }

    pub fn intersects(&self, other: &Bounds) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }
}

pub struct QuadTree {
    bounds: Bounds,
    particles: Vec<usize>,
    children: Option<[Box<QuadTree>; 4]>,
    depth: usize,
}

impl QuadTree {
    pub fn new(bounds: Bounds, depth: usize) -> Self {
        QuadTree {
            bounds,
            particles: Vec::new(),
            children: None,
            depth,
        }
    }

    pub fn clear(&mut self) {
        self.particles.clear();
        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                child.clear();
            }
        }
    }

    pub fn insert(&mut self, index: usize, position: Vec2) -> bool {
        if !self.bounds.contains(position) {
            return false;
        }

        if self.children.is_none() && self.particles.len() < MAX_PARTICLES {
            self.particles.push(index);
            return true;
        }

        if self.children.is_none() && self.depth < MAX_DEPTH {
            self.subdivide();
        }

        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                if child.insert(index, position) {
                    return true;
                }
            }
        }

        self.particles.push(index);
        true
    }

    fn subdivide(&mut self) {
        let half_width = self.bounds.width / 2.0;
        let half_height = self.bounds.height / 2.0;
        let x = self.bounds.x;
        let y = self.bounds.y;

        let nw = Bounds { x, y, width: half_width, height: half_height };
        let ne = Bounds { x: x + half_width, y, width: half_width, height: half_height };
        let sw = Bounds { x, y: y + half_height, width: half_width, height: half_height };
        let se = Bounds { x: x + half_width, y: y + half_height, width: half_width, height: half_height };

        self.children = Some([
            Box::new(QuadTree::new(nw, self.depth + 1)),
            Box::new(QuadTree::new(ne, self.depth + 1)),
            Box::new(QuadTree::new(sw, self.depth + 1)),
            Box::new(QuadTree::new(se, self.depth + 1)),
        ]);

        // Re-insert particles into children
        let particles = std::mem::take(&mut self.particles);
        for index in particles {
            // Position will be handled by World implementation
            if !self.insert(index, Vec2::new(0.0, 0.0)) {
                self.particles.push(index);
            }
        }
    }

    pub fn query(&self, range: &Bounds, found: &mut Vec<usize>) {
        if !self.bounds.intersects(range) {
            return;
        }

        for index in &self.particles {
            found.push(*index);
        }

        if let Some(children) = &self.children {
            for child in children.iter() {
                child.query(range, found);
            }
        }
    }
}

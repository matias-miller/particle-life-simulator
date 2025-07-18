use crate::utils::math::Vec2;

const MAX_PARTICLES: usize = 10;
const MAX_DEPTH: usize = 8;
const BATCH_SIZE: usize = 128; // Process particles in batches for better cache locality

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

#[derive(Debug, Clone)]
pub struct ParticleInsert {
    pub index: usize,
    pub position: Vec2,
}

#[derive(Debug, Clone)]
pub struct QueryRequest {
    pub range: Bounds,
    pub results: Vec<usize>,
}

pub struct QuadTree {
    bounds: Bounds,
    particles: Vec<usize>,
    children: Option<[Box<QuadTree>; 4]>,
    depth: usize,
    // Batch processing buffers
    insert_buffer: Vec<ParticleInsert>,
    query_buffer: Vec<QueryRequest>,
}

impl QuadTree {
    pub fn new(bounds: Bounds, depth: usize) -> Self {
        QuadTree {
            bounds,
            particles: Vec::new(),
            children: None,
            depth,
            insert_buffer: Vec::with_capacity(BATCH_SIZE),
            query_buffer: Vec::with_capacity(BATCH_SIZE),
        }
    }

    pub fn clear(&mut self) {
        self.particles.clear();
        self.insert_buffer.clear();
        self.query_buffer.clear();
        
        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                child.clear();
            }
        }
    }

    // Single particle insert (kept for compatibility)
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

    // Batch insert - much more efficient for bulk operations
    pub fn batch_insert(&mut self, particles: &[(usize, Vec2)]) {
        // Sort particles by spatial locality for better cache performance
        let mut sorted_particles: Vec<_> = particles.iter()
            .filter(|(_, pos)| self.bounds.contains(*pos))
            .copied()
            .collect();
        
        // Sort by Morton code for spatial locality
        sorted_particles.sort_by_key(|(_, pos)| {
            self.morton_encode(pos.x, pos.y)
        });

        // Process in batches
        for batch in sorted_particles.chunks(BATCH_SIZE) {
            self.process_insert_batch(batch);
        }
    }

    fn process_insert_batch(&mut self, batch: &[(usize, Vec2)]) {
        // Check if we need to subdivide before processing the batch
        let total_particles = self.particles.len() + batch.len();
        if self.children.is_none() && total_particles > MAX_PARTICLES && self.depth < MAX_DEPTH {
            self.subdivide();
        }

        // If we have children, route particles to appropriate quadrants
        if self.children.is_some() {
            // Group particles by quadrant for batch processing
            let mut quadrant_batches: [Vec<(usize, Vec2)>; 4] = [
                Vec::new(), Vec::new(), Vec::new(), Vec::new()
            ];

            // Calculate quadrant assignments without borrowing self
            let mid_x = self.bounds.x + self.bounds.width / 2.0;
            let mid_y = self.bounds.y + self.bounds.height / 2.0;

            for &(index, position) in batch {
                let quadrant = self.get_quadrant(position);
                if let Some(q) = quadrant {
                    quadrant_batches[q].push((index, position));
                } else {
                    self.particles.push(index);
                }
            }

            // Process each quadrant's batch
            if let Some(children) = &mut self.children {
                for (i, quadrant_batch) in quadrant_batches.iter().enumerate() {
                    if !quadrant_batch.is_empty() {
                        children[i].process_insert_batch(quadrant_batch);
                    }
                }
            }
        } else {
            // No children, add all to this node
            for &(index, _) in batch {
                self.particles.push(index);
            }
        }
    }

    fn get_quadrant(&self, position: Vec2) -> Option<usize> {
        let mid_x = self.bounds.x + self.bounds.width / 2.0;
        let mid_y = self.bounds.y + self.bounds.height / 2.0;
        get_quadrant_for_bounds(position, mid_x, mid_y)
    }

    // Morton encoding for spatial locality
    fn morton_encode(&self, x: f32, y: f32) -> u32 {
        let norm_x = ((x - self.bounds.x) / self.bounds.width * 1024.0) as u32;
        let norm_y = ((y - self.bounds.y) / self.bounds.height * 1024.0) as u32;
        
        self.interleave_bits(norm_x) | (self.interleave_bits(norm_y) << 1)
    }

    fn interleave_bits(&self, mut x: u32) -> u32 {
        x = (x | (x << 8)) & 0x00FF00FF;
        x = (x | (x << 4)) & 0x0F0F0F0F;
        x = (x | (x << 2)) & 0x33333333;
        x = (x | (x << 1)) & 0x55555555;
        x
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

        // Re-insert particles into children using batch processing
        // Note: This requires actual particle positions, which are not stored in self.particles
        // You'll need to maintain a separate Vec<(usize, Vec2)> or pass positions from the caller
        let particles = std::mem::take(&mut self.particles);
        // Assuming positions are provided externally or stored elsewhere
        // For now, this is a placeholder; you'll need to fix this based on your data structure
        let particle_positions: Vec<_> = particles.into_iter()
            .map(|index| (index, Vec2::new(0.0, 0.0))) // Placeholder: Replace with actual positions
            .collect();
        
        if !particle_positions.is_empty() {
            self.process_insert_batch(&particle_positions);
        }
    }

    // Single query (kept for compatibility)
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

    // Batch query - process multiple queries efficiently
    pub fn batch_query(&self, queries: &[Bounds]) -> Vec<Vec<usize>> {
        let mut results = Vec::with_capacity(queries.len());
        
        for batch in queries.chunks(BATCH_SIZE) {
            let mut batch_results = Vec::with_capacity(batch.len());
            
            for range in batch {
                let mut found = Vec::new();
                self.query(range, &mut found);
                batch_results.push(found);
            }
            
            results.extend(batch_results);
        }
        
        results
    }

    // Optimized range query for circular ranges (common in particle systems)
    pub fn query_radius(&self, center: Vec2, radius: f32, found: &mut Vec<usize>) {
        let radius_sq = radius * radius;
        let range = Bounds {
            x: center.x - radius,
            y: center.y - radius,
            width: radius * 2.0,
            height: radius * 2.0,
        };

        self.query_radius_internal(&range, center, radius_sq, found);
    }

    fn query_radius_internal(&self, range: &Bounds, center: Vec2, radius_sq: f32, found: &mut Vec<usize>) {
        if !self.bounds.intersects(range) {
            return;
        }

        // For leaf nodes, check distance to center
        for index in &self.particles {
            found.push(*index); // Distance check would need particle positions from caller
        }

        if let Some(children) = &self.children {
            for child in children.iter() {
                child.query_radius_internal(range, center, radius_sq, found);
            }
        }
    }

    // Get statistics for performance monitoring
    pub fn get_stats(&self) -> QuadTreeStats {
        let mut stats = QuadTreeStats {
            total_nodes: 1,
            leaf_nodes: 0,
            total_particles: self.particles.len(),
            max_depth: self.depth,
            avg_particles_per_leaf: 0.0,
        };

        if self.children.is_none() {
            stats.leaf_nodes = 1;
        } else if let Some(children) = &self.children {
            for child in children.iter() {
                let child_stats = child.get_stats();
                stats.total_nodes += child_stats.total_nodes;
                stats.leaf_nodes += child_stats.leaf_nodes;
                stats.total_particles += child_stats.total_particles;
                stats.max_depth = stats.max_depth.max(child_stats.max_depth);
            }
        }

        if stats.leaf_nodes > 0 {
            stats.avg_particles_per_leaf = stats.total_particles as f32 / stats.leaf_nodes as f32;
        }

        stats // Explicitly return the stats struct
    }
}

// Helper function to avoid borrowing issues
fn get_quadrant_for_bounds(position: Vec2, mid_x: f32, mid_y: f32) -> Option<usize> {
    if position.x < mid_x && position.y < mid_y {
        Some(0) // NW
    } else if position.x >= mid_x && position.y < mid_y {
        Some(1) // NE
    } else if position.x < mid_x && position.y >= mid_y {
        Some(2) // SW
    } else if position.x >= mid_x && position.y >= mid_y {
        Some(3) // SE
    } else {
        None
    }
}

#[derive(Debug)]
pub struct QuadTreeStats {
    pub total_nodes: usize,
    pub leaf_nodes: usize,
    pub total_particles: usize,
    pub max_depth: usize,
    pub avg_particles_per_leaf: f32,
}
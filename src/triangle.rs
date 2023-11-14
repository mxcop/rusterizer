use glam::{Vec2, Vec3};

use crate::{barycentric, color::Color, edge_func};

pub struct Triangle {
    vertices: [Vertex; 3],
    area: f32,
}

impl Triangle {
    pub fn from_vertices(a: Vertex, b: Vertex, c: Vertex) -> Self {
        let area = edge_func(a.pos, b.pos, c.pos);

        Self {
            vertices: [a, b, c],
            area,
        }
    }

    fn mix_three(a: Vec2, b: Vec2, c: Vec2, mask: Vec3) -> Vec2 {
        a * mask.x + b * mask.y + c * mask.z
    }

    /// Compute the color of a position within this triangle.
    ///
    /// (returns None if the point is outside of the triangle)
    pub fn compute_pixel(&self, p: Vec2) -> Option<(Color, Vec2)> {
        barycentric(
            p,
            self.vertices[0].pos,
            self.vertices[1].pos,
            self.vertices[2].pos,
            self.area,
        )
        .map(|coords| {
            (Color::mix_three(
                self.vertices[0].color,
                self.vertices[1].color,
                self.vertices[2].color,
                coords,
            ), Self::mix_three(
                self.vertices[0].uv,
                self.vertices[1].uv,
                self.vertices[2].uv,
                coords,
            ))
        })
    }
}

pub struct Vertex {
    pos: Vec2,
    color: Color,
    uv: Vec2,
}

impl Vertex {
    pub fn new(pos: Vec2, color: Color, uv: Vec2) -> Self {
        Self { pos, color, uv }
    }
}

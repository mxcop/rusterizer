use glam::{Vec2, Vec3};
use minifb::{Window, WindowOptions};

pub mod color;
pub mod triangle;

pub fn init_window(w: usize, h: usize) -> Window {
    let mut window = Window::new(
        "Rusterizer :)",
        w,
        h,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    /* Limit frame-rate to 60 fps */
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    window
}

pub fn edge_func(p: Vec2, v0: Vec2, v1: Vec2) -> f32 {
    let seg_a = v1 - v0;
    let seg_b = p - v0;

    /* Cross product */
    seg_a.x * seg_b.y - seg_a.y * seg_b.x
}

pub fn barycentric(p: Vec2, v0: Vec2, v1: Vec2, v2: Vec2, area: f32) -> Option<Vec3> {
    let m0 = edge_func(p, v1, v2);
    let m1 = edge_func(p, v2, v0);
    let m2 = edge_func(p, v0, v1);

    let a = 1.0 / area;
    if m0 >= 0.0 && m1 >= 0.0 && m2 >= 0.0 {
        Some(Vec3::new(m0 * a, m1 * a, m2 * a))
    } else {
        None
    }
}

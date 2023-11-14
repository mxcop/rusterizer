use glam::Vec2;
use minifb::Key;
use rusterizer::{
    color::Color,
    init_window,
    triangle::{Triangle, Vertex},
};

const BUFFER_SCALE: usize = 10;

fn main() {
    let mut win_width: usize = 640;
    let mut win_height: usize = 360;

    let mut buffer: Vec<u32> = vec![0; win_width * win_height];
    let mut window = init_window(win_width, win_height);

    win_width /= BUFFER_SCALE;
    win_height /= BUFFER_SCALE;

    let img = image::open("assets/test.png").unwrap();
    let img = img.as_rgba8().unwrap();

    let triangle = {
        let vertex_a = Vertex::new(Vec2::new(5.0, 5.0), Color::red(), Vec2::new(0.0, 0.0));
        let vertex_b = Vertex::new(Vec2::new(30.0, 5.0), Color::green(), Vec2::new(1.0, 0.0));
        let vertex_c = Vertex::new(Vec2::new(5.0, 30.0), Color::blue(), Vec2::new(0.0, 1.0));

        Triangle::from_vertices(vertex_a, vertex_b, vertex_c)
    };

    let triangle2 = {
        let vertex_a = Vertex::new(Vec2::new(30.0, 5.0), Color::red(), Vec2::new(1.0, 0.0));
        let vertex_b = Vertex::new(Vec2::new(30.0, 30.0), Color::green(), Vec2::new(1.0, 1.0));
        let vertex_c = Vertex::new(Vec2::new(5.0, 30.0), Color::blue(), Vec2::new(0.0, 1.0));

        Triangle::from_vertices(vertex_a, vertex_b, vertex_c)
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.fill(0);

        /* Move one vertex with the mouse */
        // if window.get_mouse_down(minifb::MouseButton::Left) {
        //     window.get_mouse_pos(minifb::MouseMode::Clamp).map(|mouse| {
        //         v1 = Vec2::new(mouse.0, mouse.1) / 10.0;
        //     });
        // }

        /* Iterate over all pixels in the screen buffer */
        for (i, pixel) in buffer.iter_mut().enumerate() {
            /* Partition the index into cartesian coordinates */
            let p = Vec2::new((i % win_width) as f32, (i / win_width) as f32);

            if let Some((_color, uv)) = triangle.compute_pixel(p) {
                let sample = img.get_pixel((uv.x * img.width() as f32 - 1.0) as u32, (uv.y * img.height() as f32 - 1.0) as u32);
                
                *pixel = Color::from_bytes(sample.0[0], sample.0[1], sample.0[2], sample.0[3]).into_argb();
            };

            if let Some((_color, uv)) = triangle2.compute_pixel(p) {
                let sample = img.get_pixel((uv.x * img.width() as f32 - 1.0) as u32, (uv.y * img.height() as f32 - 1.0) as u32);
                
                *pixel = Color::from_bytes(sample.0[0], sample.0[1], sample.0[2], sample.0[3]).into_argb();
            };
        }

        /* Update buffer size if window is resized */
        let (new_w, new_h) = window.get_size();
        if new_w != win_width || new_h != win_height {
            win_width = new_w / BUFFER_SCALE;
            win_height = new_h / BUFFER_SCALE;
            buffer.resize(win_width * win_height, 0);
        }

        /* Update the screen buffer */
        window
            .update_with_buffer(&buffer, win_width, win_height)
            .unwrap();
    }
}

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use crate::model::Model;
use crate::vector::Vec3f;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

#[macro_use]
mod vector;
mod model;
mod triangle_renderer;

fn main() {
    let model = Model::parse(&String::from("data/models/african_head.obj"), &String::from("data/models/african_head_diffuse.tga")).unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Game", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        //.present_vsync()
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let tex_creator = canvas.texture_creator();
    let mut game_tex = tex_creator
        .create_texture(
            sdl2::pixels::PixelFormatEnum::ARGB8888,
            sdl2::render::TextureAccess::Streaming,
            WIDTH,
            HEIGHT,
        )
        .expect("Couldn't create buffer");

    let mut framebuffer = vec![0u32; (WIDTH * HEIGHT) as usize].into_boxed_slice();
    let mut zbuffer = vec![0.0; (WIDTH * HEIGHT) as usize].into_boxed_slice();
    let mut horizontal_x = 0;
    let update_rectangle = Rect::new(0, 0, WIDTH, HEIGHT);
    let horizontal_pitch = 4 * WIDTH as usize;
    let half_height = (HEIGHT/2) as f64;
    let half_width = (WIDTH/2) as f64;

    let light_dir = Vec3f { x:0.0, y:0.0, z:-1.0 };

    let color_red = 0xFFFF0000 as u32;

    'running: loop {
        // Handle user input
        // Escape key exits, everything else is ignored
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // Clear framebuffer
        framebuffer.fill(0);
        // Clear zbuffer
        zbuffer.fill(f64::MIN);

        for i in 0..model.faces.len() { 
            let face = &model.faces[i];
            let v1 = &model.vertices[face[0].vertex_index as usize];
            let v2 = &model.vertices[face[1].vertex_index as usize];
            let v3 = &model.vertices[face[2].vertex_index as usize];
            let screen1 = Vec3f{ x:((v1.x + 1.0) * half_width) + 0.5, y:((v1.y+1.0) * half_height) + 0.5, z:v1.z};
            let screen2 = Vec3f{ x:((v2.x + 1.0) * half_width) + 0.5, y:((v2.y+1.0) * half_height) + 0.5, z:v2.z};
            let screen3 = Vec3f{ x:((v3.x + 1.0) * half_width) + 0.5, y:((v3.y+1.0) * half_height) + 0.5, z:v3.z};

            let mut n = (v3-v1).cross(&(v2-v1));
            n = n.normalize();
            let intensity = n.dot(&light_dir);

            if intensity > 0.0 {
                let gray = (intensity * 255.0) as u32 & 0xFF;
                let tri_color = 0xFF000000 | gray << 16 | gray << 8 | gray;
                triangle_renderer::render_triangle(&screen1, &screen2, &screen3, &mut framebuffer, WIDTH as i64, HEIGHT as i64, &mut zbuffer, tri_color);
            }
        }

        draw_line(
            horizontal_x,
            13,
            40,
            80,
            &mut framebuffer,
            color_red,
        );
        horizontal_x = (horizontal_x + 1) % WIDTH as i32;

        // Update screen texture
        game_tex
            .update(
                update_rectangle,
                unsafe { &framebuffer.align_to().1 },
                horizontal_pitch,
            )
            .unwrap();
        // Copy texture to screen
        canvas.copy(&game_tex, None, None).unwrap();
        // Flip the screen buffers
        canvas.present();
    }
}

fn draw_line(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    canvas: &mut Box<[u32]>,
    color: u32,
) {
    let mut steep = false;
    if (x0 - x1).abs() < (y0 - y1).abs() {
        (x0, y0) = (y0, x0);
        (x1, y1) = (y1, x1);
        steep = true;
    }
    if x0 > x1 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }
    let dx = x1 - x0;
    let dx2 = dx * 2;
    let dy = y1 - y0;
    let derror = dy.abs() * 2;
    let mut error = 0;
    let mut y = y0;
    let yincr = if y1 > y0 { 1 } else { -1 };

    if steep {
        for x in x0..=x1 {
            canvas[((WIDTH as i32 * x) + y) as usize] = color;
            error += derror;
            if error > dx {
                y += yincr;
                error -= dx2;
            }
        }
    } else {
        for x in x0..=x1 {
            canvas[((WIDTH as i32 * y) + x) as usize] = color;
            error += derror;
            if error > dx {
                y += yincr;
                error -= dx2;
            }
        }
    }
}

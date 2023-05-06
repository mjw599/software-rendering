use crate::model::Model;
use crate::vector::Vec3f;
use crate::triangle_renderer::VertexData;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder, dpi::PhysicalSize,
};
use softbuffer::GraphicsContext;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

#[macro_use]
mod vector;
mod model;
mod triangle_renderer;

fn main() {

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_inner_size(PhysicalSize::new(WIDTH, HEIGHT)).build(&event_loop).unwrap();

    let mut context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();

    let model = Model::parse(&String::from("data/models/african_head.obj"), &String::from("data/models/african_head_diffuse.tga")).unwrap();

    let mut framebuffer = vec![0u32; (WIDTH * HEIGHT) as usize].into_boxed_slice();
    let mut zbuffer = vec![0.0; (WIDTH * HEIGHT) as usize].into_boxed_slice();
    let mut horizontal_x = 0;
    let half_height = (HEIGHT/2) as f64;
    let half_width = (WIDTH/2) as f64;

    let light_dir = Vec3f { x:0.0, y:0.0, z:-1.0 };

    let color_red = 0xFFFF0000 as u32;


    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::MainEventsCleared => {
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

                    let v1uv = &model.texture_coords[face[0].texture_index as usize];
                    let v2uv = &model.texture_coords[face[1].texture_index as usize];
                    let v3uv = &model.texture_coords[face[2].texture_index as usize];

                    let vertex1 = VertexData { vertex: &screen1, texture: v1uv };
                    let vertex2 = VertexData { vertex: &screen2, texture: v2uv };
                    let vertex3 = VertexData { vertex: &screen3, texture: v3uv };

                    let mut n = (v3-v1).cross(&(v2-v1));
                    n = n.normalize();
                    let intensity = n.dot(&light_dir);

                    if intensity > 0.0 {
                        triangle_renderer::render_triangle(&vertex1, &vertex2, &vertex3, &mut framebuffer, WIDTH as i64, HEIGHT as i64, &mut zbuffer, intensity, model.textures.get("diffuse").unwrap());
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

                context.set_buffer(&framebuffer, WIDTH as u16, HEIGHT as u16);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
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

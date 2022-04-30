use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
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

    let mut framebuffer = vec![0u8; (WIDTH * HEIGHT * 4) as usize].into_boxed_slice();
    let mut horizontal_x = 0;
    let update_rectange = Rect::new(0, 0, WIDTH, HEIGHT);
    let horizontal_pitch = 4 * WIDTH as usize;

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

        draw_line(
            horizontal_x,
            13,
            40,
            80,
            &mut framebuffer,
            Color::RGBA(255, 0, 0, 255),
        );
        draw_line(
            80,
            40,
            13,
            20,
            &mut framebuffer,
            Color::RGBA(0, 255, 0, 255),
        );
        horizontal_x = (horizontal_x + 1) % WIDTH as i32;

        // Update screen texture
        game_tex
            .update(
                update_rectange,
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
    canvas: &mut Box<[u8]>,
    color: Color,
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
            let index = (((WIDTH as i32 * x) + y) * 4) as usize;
            canvas[index] = color.b;
            canvas[index + 1] = color.g;
            canvas[index + 2] = color.r;
            canvas[index + 3] = color.a;
            error += derror;
            if error > dx {
                y += yincr;
                error -= dx2;
            }
        }
    } else {
        for x in x0..=x1 {
            let index = (((WIDTH as i32 * y) + x) * 4) as usize;
            canvas[index] = color.b;
            canvas[index + 1] = color.g;
            canvas[index + 2] = color.r;
            canvas[index + 3] = color.a;
            error += derror;
            if error > dx {
                y += yincr;
                error -= dx2;
            }
        }
    }
}

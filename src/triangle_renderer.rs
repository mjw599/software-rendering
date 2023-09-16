use image::{DynamicImage, GenericImageView};
use softbuffer::Buffer;

use crate::vector::*;

pub struct VertexData<'a> {
    pub vertex: &'a Vec3f,
    pub texture: &'a Vec3f
}

pub fn render_triangle( v1:&VertexData, v2:&VertexData, v3:&VertexData, canvas: &mut Buffer<'_>, canvas_width: i64, canvas_height: i64,
    zbuffer: &mut Box<[f64]>, intensity: f64, diffuse_texture: &DynamicImage ) {
    let bboxminx = v1.vertex.x.min( v2.vertex.x ).min( v3.vertex.x ).max( 0.0 );
    let bboxminy = v1.vertex.y.min( v2.vertex.y ).min( v3.vertex.y ).max( 0.0 );
    let bboxmaxx = v1.vertex.x.max( v2.vertex.x ).max( v3.vertex.x ).min( (canvas_width - 1) as f64 );
    let bboxmaxy = v1.vertex.y.max( v2.vertex.y ).max( v3.vertex.y ).min( (canvas_height - 1) as f64 );

    let texture_width = diffuse_texture.width() as f64;
    let texture_height = diffuse_texture.height() as f64;

    for y in (bboxminy as i64)..=(bboxmaxy as i64) {
        for x in (bboxminx as i64)..=(bboxmaxx as i64) {
            let barycentric_screen = barycentric( v1.vertex, v2.vertex, v3.vertex, x, y );
            if barycentric_screen.x < 0.0 || barycentric_screen.y < 0.0 || barycentric_screen.z < 0.0 {
                continue;
            }
            let z = (v1.vertex.z * barycentric_screen.x) + (v2.vertex.z * barycentric_screen.y) + (v3.vertex.z * barycentric_screen.z);
            let pixel_index = ((canvas_width * y) + x) as usize;
            if zbuffer[pixel_index] < z {
                zbuffer[pixel_index] = z;

                let u = (v1.texture.x * barycentric_screen.x) + (v2.texture.x * barycentric_screen.y) + (v3.texture.x * barycentric_screen.z);
                let v = 1.0 - ((v1.texture.y * barycentric_screen.x) + (v2.texture.y * barycentric_screen.y) + (v3.texture.y * barycentric_screen.z));

                let pixel_color = diffuse_texture.get_pixel((u * texture_width) as u32, (v * texture_height) as u32);

                let color = 0xFF000000 | get_color(pixel_color[0], intensity) << 16 | get_color(pixel_color[1], intensity) << 8 | get_color(pixel_color[2], intensity);

                canvas[pixel_index] = color;
            }
        }
    }
}

#[inline(always)]
fn get_color( color:u8, intensity:f64 ) -> u32 {
    ((color as f64) * intensity) as u32
}

fn barycentric( v1:&Vec3f, v2:&Vec3f, v3:&Vec3f, x:i64, y:i64 ) -> Vec3f {
    let temp1 = Vec3f{ x:v3.x-v1.x, y:v2.x-v1.x, z:v1.x-(x as f64) };
    let temp2 = Vec3f{ x:v3.y-v1.y, y:v2.y-v1.y, z:v1.y-(y as f64) };

    let u = temp1.cross( &temp2 );

    if u.z.abs() < 1e-2 {
        return Vec3f { x:-1.0, y:1.0, z:1.0 };
    }

    Vec3f { 
        x:1.0 - (u.x as f64 + u.y as f64)/u.z as f64, 
        y:u.y as f64/u.z as f64, 
        z:u.x as f64/u.z as f64 
    }
}
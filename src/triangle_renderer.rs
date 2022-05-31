use crate::vector::*;

pub fn render_triangle( v1:&Vec3f, v2:&Vec3f, v3:&Vec3f, canvas: &mut Box<[u32]>, canvas_width: i64, canvas_height: i64,
    zbuffer: &mut Box<[f64]>, color: u32 ) {
    let mut bboxmin = Vec2f { x:f64::MAX, y:f64::MAX };
    let mut bboxmax = Vec2f{ x:f64::MIN, y:f64::MIN };
    let clamp = Vec2i { x:canvas_width - 1, y:canvas_height - 1 };

    bboxmin.x = 0.0_f64.max(bboxmin.x.min(v1.x));
    bboxmin.x = 0.0_f64.max(bboxmin.x.min(v2.x));
    bboxmin.x = 0.0_f64.max(bboxmin.x.min(v3.x));

    bboxmin.y = 0.0_f64.max(bboxmin.y.min(v1.y));
    bboxmin.y = 0.0_f64.max(bboxmin.y.min(v2.y));
    bboxmin.y = 0.0_f64.max(bboxmin.y.min(v3.y));

    bboxmax.x = (clamp.x as f64).min(bboxmax.x.max(v1.x));
    bboxmax.x = (clamp.x as f64).min(bboxmax.x.max(v2.x));
    bboxmax.x = (clamp.x as f64).min(bboxmax.x.max(v3.x));

    bboxmax.y = (clamp.y as f64).min(bboxmax.y.max(v1.y));
    bboxmax.y = (clamp.y as f64).min(bboxmax.y.max(v2.y));
    bboxmax.y = (clamp.y as f64).min(bboxmax.y.max(v3.y));

    for x in (bboxmin.x as i64)..=(bboxmax.x as i64) {
        for y in (bboxmin.y as i64)..=(bboxmax.y as i64) {
            let barycentric_screen = barycentric( v1, v2, v3, x, y );
            if barycentric_screen.x < 0.0 || barycentric_screen.y < 0.0 || barycentric_screen.z < 0.0 {
                continue;
            }
            let z = (v1.z * barycentric_screen.x) + (v2.z * barycentric_screen.y) + (v3.z * barycentric_screen.z);
            let pixel_index = ((canvas_width * y) + x) as usize;
            if zbuffer[pixel_index] < z {
                zbuffer[pixel_index] = z;
                canvas[pixel_index] = color;
            }
        }
    }
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
use std::cmp;
use sdl2::pixels::Color;
// use rayon::prelude::*;

use crate::vector::Vec2i;
use crate::vector::Vec3f;
use crate::vector::Vec3i;

pub fn render_triangle( v1:&Vec2i, v2:&Vec2i, v3:&Vec2i, canvas: &mut Box<[u8]>, canvas_width: i64, canvas_height: i64,
    color: Color ) {
        let mut bboxmin = Vec2i { x:canvas_width - 1, y:canvas_height - 1 };
        let mut bboxmax = Vec2i::zero();
        let clamp = Vec2i { x:canvas_width - 1, y:canvas_height - 1 };

        bboxmin.x = cmp::max( 0, cmp::min(bboxmin.x, v1.x));
        bboxmin.x = cmp::max( 0, cmp::min(bboxmin.x, v2.x));
        bboxmin.x = cmp::max( 0, cmp::min(bboxmin.x, v3.x));

        bboxmin.y = cmp::max( 0, cmp::min(bboxmin.y, v1.y));
        bboxmin.y = cmp::max( 0, cmp::min(bboxmin.y, v2.y));
        bboxmin.y = cmp::max( 0, cmp::min(bboxmin.y, v3.y));

        bboxmax.x = cmp::min( clamp.x, cmp::max(bboxmax.x, v1.x));
        bboxmax.x = cmp::min( clamp.x, cmp::max(bboxmax.x, v2.x));
        bboxmax.x = cmp::min( clamp.x, cmp::max(bboxmax.x, v3.x));

        bboxmax.y = cmp::min( clamp.y, cmp::max(bboxmax.y, v1.y));
        bboxmax.y = cmp::min( clamp.y, cmp::max(bboxmax.y, v2.y));
        bboxmax.y = cmp::min( clamp.y, cmp::max(bboxmax.y, v3.y));

        // (bboxmin.x..=bboxmax.x).into_par_iter().for_each(|x| {
        //     (bboxmin.y..=bboxmax.y).into_par_iter().for_each(|y| {
        //         let barycentric_screen = barycentric( v1, v2, v3, x, y );
        //         if ! (barycentric_screen.x < 0.0 || barycentric_screen.y < 0.0 || barycentric_screen.z < 0.0)  {
        //             let index = (((canvas_width * y) + x) * 4) as usize;
        //             if index < canvas.len() {
        //                 canvas[index] = color.b;
        //                 canvas[index + 1] = color.g;
        //                 canvas[index + 2] = color.r;
        //                 canvas[index + 3] = color.a;
        //             }
        //         }
        //     });
        // });

        for x in bboxmin.x..=bboxmax.x {
            for y in bboxmin.y..=bboxmax.y {
                let barycentric_screen = barycentric( v1, v2, v3, x, y );
                if barycentric_screen.x < 0.0 || barycentric_screen.y < 0.0 || barycentric_screen.z < 0.0 {
                    continue;
                }
                let index = (((canvas_width * y) + x) * 4) as usize;
                if index < canvas.len() {
                    canvas[index] = color.b;
                    canvas[index + 1] = color.g;
                    canvas[index + 2] = color.r;
                    canvas[index + 3] = color.a;
                }
            }
        }
    }

fn barycentric( v1:&Vec2i, v2:&Vec2i, v3:&Vec2i, x:i64, y:i64 ) -> Vec3f {
    let temp1 = Vec3i{ x:v3.x-v1.x, y:v2.x-v1.x, z:v1.x-x };
    let temp2 = Vec3i{ x:v3.y-v1.y, y:v2.y-v1.y, z:v1.y-y };

    let u = temp1.cross( &temp2 );

    if u.z.abs() < 1 {
        return Vec3f { x:-1.0, y:1.0, z:1.0 };
    }

    Vec3f { 
        x:1.0 - (u.x as f64 + u.y as f64)/u.z as f64, 
        y:u.y as f64/u.z as f64, 
        z:u.x as f64/u.z as f64 
    }
}
use std::alloc::System;
#[global_allocator]
static A: System = System;
extern crate core;

use minifb::{Scale, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

mod functions;
use functions::*;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut cube = Obj{ mesh: vec![], faces: vec![] };

    cube.load_from_file("C:/Users/Cysie/CLionProjects/Renderer_V3/src/monke.obj");

    let mut window = Window::new(
        "Renderer",
        WIDTH,
        HEIGHT,
        WindowOptions {
            ..WindowOptions::default()
        },
    ).unwrap();

    window.set_position(360, 0);
    /*window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));*/

    while window.is_open() {
        /*buffer[((200 /*y*/ as usize) * (WIDTH)) + 200 /*x*/ as usize] = 0x00ffffff;*/
        cube.faces.sort_by(|x, y| {
            (-(cube.mesh[x[0]].z + cube.mesh[x[1]].z + cube.mesh[x[2]].z) / 3.0)
                .partial_cmp(&(-(cube.mesh[y[0]].z + cube.mesh[y[1]].z + cube.mesh[y[2]].z) / 3.0))
                .unwrap()
        });
        for i in &cube.faces{
            let a = &cube.mesh[i[0]];
            let b = &cube.mesh[i[1]];
            let c = &cube.mesh[i[2]];
            let line1 = Vec3{
                x: b.x-a.x,
                y: b.y-a.y,
                z: b.z-a.z,
            };
            let line2 = Vec3{
                x: c.x-a.x,
                y: c.y-a.y,
                z: c.z-a.z,
            };

            let mut normal = Vec3 {
                x: line1.y * line2.z - line1.z * line2.y,
                y: line1.z * line2.x - line1.x * line2.z,
                z: line1.x * line2.y - line1.y * line2.x,
            };

            let l = (normal.x.powf(2.0) + normal.y.powf(2.0) + normal.z.powf(2.0)).sqrt();
            normal.x /= l;
            normal.y /= l;
            normal.z /= l;

            if (normal.x * (a.x)
                + normal.y * (a.y)
                + normal.z * (a.z))
                < 0.0{
                let mut light_direction = Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                };
                let l = (light_direction.x * light_direction.x
                    + light_direction.y * light_direction.y
                    + light_direction.z * light_direction.z)
                    .sqrt();
                light_direction.x /= l;
                light_direction.y /= l;
                light_direction.z /= l;
                let dp = ((255.0*(normal.x * light_direction.x
                    + normal.y * light_direction.y
                    + normal.z * light_direction.z)) as u32 * 0x10101).max(0x3f3f3f);

                let ap = a.project(3.0);
                let bp = b.project(3.0);
                let cp = c.project(3.0);

                /*line(&mut buffer, ap,bp, 0xff00ff);
                line(&mut buffer, ap,cp, 0xff00ff);
                line(&mut buffer, cp,bp, 0xff00ff);*/
                triangle(&mut buffer,
                         ap,
                         bp,
                         cp,
                         dp);
            }



        }
       /* for i in &mut cube.mesh{
            i.x -= 0.01;
        }*/
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();//.expect("Oops!");

        /*buffer = vec![0; WIDTH * HEIGHT];*/
        buffer.clear();
        buffer.resize(WIDTH*HEIGHT,0);

    }
}
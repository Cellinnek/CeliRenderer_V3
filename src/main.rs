use std::alloc::System;
#[global_allocator]
static A: System = System;
extern crate core;

use minifb::{Scale, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

mod functions;
use functions::*;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut cube = Obj {
        mesh: vec![],
        faces: vec![],
        projected_mesh: vec![]
    };

    cube.load_from_file("C:/Users/Cysie/CLionProjects/Renderer_V3/src/monke.obj");
    cube.projected_mesh = vec![[0,0]; cube.mesh.len()];

    let mut window = Window::new("Renderer", WIDTH, HEIGHT, WindowOptions{
        scale: Scale::X1,
        ..WindowOptions::default()
    }
    ).unwrap();

    window.set_position(360, 0);
    /*window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));*/

    while window.is_open() {
        cube.faces.sort_unstable_by(|x, y|
            ((&cube.mesh[y[0]].z + &cube.mesh[y[1]].z + &cube.mesh[y[2]].z) / 3.0)
                .partial_cmp(&((&cube.mesh[x[0]].z + &cube.mesh[x[1]].z + &cube.mesh[x[2]].z) / 3.0))
                .unwrap()
        );

        cube.rotate(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 6.0,
        }, 0.05, 1);

        for i in &cube.faces {
            let a = &cube.mesh[i[0]];
            let b = &cube.mesh[i[1]];
            let c = &cube.mesh[i[2]];

            let normal = normal(a,b,c);

            if (dot(&normal,a)) < 0.0 {
                let mut light_direction = Vec3 {
                    x: 1.5,
                    y: -1.5,
                    z: -1.0,
                };
                light_direction.normalise();

                let dp = ((128.0
                    * dot(&normal, &light_direction)) as u32
                    * 0x10101
                    + 0x3f3f3f)
                    .max(0x2b2b2b);


                let ap = if cube.projected_mesh[i[0]] == [0,0] {
                    a.project(2.0)
                }
                else{
                    cube.projected_mesh[i[0]]
                };
                let bp = if cube.projected_mesh[i[1]] == [0,0] {
                    b.project(2.0)
                }
                else{
                    cube.projected_mesh[i[1]]
                };
                let cp = if cube.projected_mesh[i[2]] == [0,0] {
                    c.project(2.0)
                }
                else{
                    cube.projected_mesh[i[2]]
                };

                cube.projected_mesh[i[0]] = ap;
                cube.projected_mesh[i[1]] = bp;
                cube.projected_mesh[i[2]] = cp;

                triangle(&mut buffer, ap, bp, cp, dp);
            }
        }
        cube.projected_mesh.clear();
        cube.projected_mesh.resize(cube.mesh.len(), [0,0]);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap(); //.expect("Oops!");

        buffer.clear();
        buffer.resize(WIDTH * HEIGHT, 0);
    }
}

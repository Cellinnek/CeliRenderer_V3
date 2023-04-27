use std::alloc::System;
#[global_allocator]
static A: System = System;
extern crate core;

use minifb::{Scale, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const FOV: f32 = 2.0;

mod functions;
use functions::*;

fn main() {
    let mut fi = 0.0;

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

    let mut triangles:Vec<Triangle> = vec![];

    while window.is_open() {
        fi += 0.005;

        for i in &cube.faces {
            let vertex1 = cube.mesh[i[0]].rotate(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 6.0,
            }, fi, 1);
            let vertex2 = cube.mesh[i[1]].rotate(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 6.0,
            }, fi, 1);
            let vertex3 = cube.mesh[i[2]].rotate(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 6.0,
            }, fi, 1);

            let normal = normal(&vertex1,&vertex2,&vertex3);

            if (dot(&normal,&vertex1)) < 0.0 {
                if cube.projected_mesh[i[0]] == [0,0] {
                    cube.projected_mesh[i[0]] = vertex1.project(FOV);
                }
                if cube.projected_mesh[i[1]] == [0,0] {
                    cube.projected_mesh[i[1]] = vertex2.project(FOV);
                }
                if cube.projected_mesh[i[2]] == [0,0] {
                    cube.projected_mesh[i[2]] = vertex3.project(FOV);
                }

                let mut light_direction = Vec3 {
                    x: 1.5,
                    y: -1.5,
                    z: -1.0,
                };
                light_direction.normalise();

                let dp = ((200.0
                    * dot(&normal, &light_direction)) as u32
                     + 25) * 0x010101;

                /*let dp = ((150.0
                    * dot(&normal, &light_direction)) as u32
                    + (50.0 * (2.0 - cube.projected_mesh[i[0]][1] as f32/300.0)) as u32) * 0x010101;*/ // <- gradient uuu aaa

                triangles.push(Triangle{
                    a: cube.projected_mesh[i[0]],
                    b: cube.projected_mesh[i[1]],
                    c: cube.projected_mesh[i[2]],
                    depth: (vertex1.z + vertex2.z + vertex3.z) / 3.0,
                    color: dp
                });

            }
        }

        triangles.sort_unstable_by(|y, x| x.depth.partial_cmp(&y.depth).unwrap());

        for i in &triangles {i.draw(&mut buffer); }

        triangles.clear();

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap(); //.expect("Oops!");

        cube.projected_mesh.clear();
        cube.projected_mesh.resize(cube.mesh.len(), [0,0]);
        buffer.clear();
        buffer.resize(WIDTH * HEIGHT, 0);
    }
}

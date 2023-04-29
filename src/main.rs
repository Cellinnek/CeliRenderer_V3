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

    let mut cube = Obj {
        mesh: vec![],
        faces: vec![],
        projected_mesh: vec![]
    };

    cube.load_from_file("C:/Users/Cysie/CLionProjects/Renderer_V3/src/monke.obj");


    let mut window = Window::new("Renderer", WIDTH, HEIGHT, WindowOptions{
        scale: Scale::X1,
        ..WindowOptions::default()
    }
    ).unwrap();

    window.set_position(360, 0);
    /*window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));*/

    while window.is_open() {
        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
        let mut triangles:Vec<Triangle> = vec![];
        let mut rotated = vec![Vec3{
            x: 0.0,
            y: 0.0,
            z: -10.0,
        }; cube.mesh.len()];
        cube.projected_mesh = vec![[0,0]; cube.mesh.len()];

        fi += 0.005;

        for i in &cube.faces {
            if rotated[i[0]].z == -10.0 {
                rotated[i[0]] = cube.mesh[i[0]].rotate(Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 6.0,
                }, fi, 1);
            }
            if rotated[i[1]].z == -10.0 {
                rotated[i[1]] = cube.mesh[i[1]].rotate(Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 6.0,
                }, fi, 1);
            }
            if rotated[i[2]].z == -10.0 {
                rotated[i[2]] = cube.mesh[i[2]].rotate(Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 6.0,
                }, fi, 1);
            }


            let normal = normal(&rotated[i[0]],&rotated[i[1]],&rotated[i[2]]);

            if (dot(&normal,&rotated[i[0]])) < 0.0 {
                if cube.projected_mesh[i[0]] == [0,0] {
                    cube.projected_mesh[i[0]] = rotated[i[0]].project(FOV);
                }
                if cube.projected_mesh[i[1]] == [0,0] {
                    cube.projected_mesh[i[1]] = rotated[i[1]].project(FOV);
                }
                if cube.projected_mesh[i[2]] == [0,0] {
                    cube.projected_mesh[i[2]] = rotated[i[2]].project(FOV);
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
                    depth: (rotated[i[0]].z + rotated[i[1]].z + rotated[i[2]].z) / 3.0,
                    color: dp
                });

            }
        }

        triangles.sort_unstable_by(|y, x| x.depth.partial_cmp(&y.depth).unwrap());

        for i in &triangles {
            if i.depth > 0.0 {
                i.draw_face(&mut buffer);
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap(); //.expect("Oops!");
    }
}

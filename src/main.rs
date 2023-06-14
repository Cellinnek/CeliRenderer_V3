use std::alloc::System;

#[global_allocator]
static A: System = System;
extern crate core;

use minifb::{Key, Scale, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

mod functions;
use functions::*;

fn main() {
    let mut fov: f64 = -2.0;
    let mut camera = Camera{
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let mut fi:f64 = 0.0;
    let mut di:f64 = 0.0;

    let mut cube = Obj {
        mesh: vec![],
        faces: vec![],
        projected_mesh: vec![]
    };

    cube.load_from_file("C:/Users/Cysie/CLionProjects/CeliRenderer_V3/src/mountains.obj");

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
            z: 0.0,
        }; cube.mesh.len()];
        let mut rotated_index = vec![false; cube.mesh.len()];
        cube.projected_mesh = vec![[0,0]; cube.mesh.len()];
        let mut projected_index = vec![false; cube.mesh.len()];

        for i in window.get_keys() {
            match i {
                Key::W => {
                    camera.x += 0.04 * fi.sin();
                    camera.z += 0.04 * fi.cos();
                }
                Key::S => {
                    camera.x -= 0.04 * fi.sin();
                    camera.z -= 0.04 * fi.cos();
                }
                Key::A => {
                    camera.x += 0.04 * fi.cos();
                    camera.z -= 0.04 * fi.sin();
                }
                Key::D => {
                    camera.x -= 0.04 * fi.cos();
                    camera.z += 0.04 * fi.sin();
                }
                Key::Space => {
                    camera.y += 0.04;
                }
                Key::LeftShift => {
                    camera.y -= 0.04;
                }
                Key::Left => {
                    fi += 0.01;
                }
                Key::Right => {
                    fi -= 0.01;
                }
                Key::Up => {
                    di -= 0.01;
                }
                Key::Down => {
                    di += 0.01;
                }
                Key::Q => {
                    fov *= 0.99;
                }
                Key::E => {
                    fov /= 0.99;
                }
                _ => ()

            }
        }

        for i in &cube.faces {
            if rotated_index[i[0]] == false {
                rotated[i[0]] = cube.mesh[i[0]].rotate(Vec3 {
                    x: camera.x,
                    y: camera.y,
                    z: camera.z,
                }, fi, 1).rotate(Vec3 {
                    x: camera.x,
                    y: camera.y,
                    z: camera.z,
                }, di, 0);

                rotated[i[0]].x -= camera.x;
                rotated[i[0]].y -= camera.y;
                rotated[i[0]].z -= camera.z;

                rotated_index[i[0]] = true;
            }
            if rotated_index[i[1]] == false {
                rotated[i[1]] = cube.mesh[i[1]].rotate(Vec3 {
                    x: camera.x,
                    y: camera.y,
                    z: camera.z,
                }, fi, 1).rotate(Vec3 {
                    x: camera.x,
                    y: camera.y,
                    z: camera.z,
                }, di, 0);

                rotated[i[1]].x -= camera.x;
                rotated[i[1]].y -= camera.y;
                rotated[i[1]].z -= camera.z;

                rotated_index[i[1]] = true;
            }
            if rotated_index[i[2]] == false {
                rotated[i[2]] = cube.mesh[i[2]].rotate(Vec3 {
                    x: camera.x,
                    y: camera.y,
                    z: camera.z,
                }, fi, 1).rotate(Vec3 {
                    x: camera.x,
                    y: camera.y,
                    z: camera.z,
                }, di, 0);

                rotated[i[2]].x -= camera.x;
                rotated[i[2]].y -= camera.y;
                rotated[i[2]].z -= camera.z;

                rotated_index[i[2]] = true;
            }

            let normal = normal(&rotated[i[0]],&rotated[i[1]],&rotated[i[2]]);
            if ((dot(&normal,&rotated[i[0]])) < 0.0) &&
                (rotated[i[0]].z + rotated[i[1]].z + rotated[i[2]].z) > 0.0 &&
                !(rotated[i[0]].x > rotated[i[0]].z/-fov &&
                rotated[i[1]].x > rotated[i[1]].z/-fov &&
                rotated[i[2]].x > rotated[i[2]].z/-fov) &&
                !(rotated[i[0]].x < rotated[i[0]].z/fov &&
                rotated[i[1]].x < rotated[i[1]].z/fov &&
                rotated[i[2]].x < rotated[i[2]].z/fov) &&
                !(rotated[i[0]].y > rotated[i[0]].z/-fov &&
                rotated[i[1]].y > rotated[i[1]].z/-fov &&
                rotated[i[2]].y > rotated[i[2]].z/-fov) &&
                !(rotated[i[0]].y < rotated[i[0]].z/fov &&
                rotated[i[1]].y < rotated[i[1]].z/fov &&
                rotated[i[2]].y < rotated[i[2]].z/fov){

                if projected_index[i[0]] == false {
                    cube.projected_mesh[i[0]] = rotated[i[0]].project(fov);
                    projected_index[i[0]] = true;
                }
                if projected_index[i[1]] == false {
                    cube.projected_mesh[i[1]] = rotated[i[1]].project(fov);
                    projected_index[i[1]] = true;
                }
                if projected_index[i[2]] == false {
                    cube.projected_mesh[i[2]] = rotated[i[2]].project(fov);
                    projected_index[i[2]] = true;
                }

                let mut light_direction = Vec3 {
                    x: 1.5,
                    y: 1.5,
                    z: -1.0,
                };
                light_direction.normalise();

                light_direction = light_direction.rotate(Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }, fi, 1);
                light_direction = light_direction.rotate(Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }, di, 0);

                let shade = ((200.0
                    * dot(&normal, &light_direction)) as u32
                     + 25) * 0x010101;

                /*let dp = ((150.0
                    * dot(&normal, &light_direction)) as u32
                    + (50.0 * (2.0 - cube.projected_mesh[i[0]][1] as f64/300.0)) as u32) * 0x010101;*/ // <- gradient uuu aaa

                triangles.push(Triangle{
                    a: cube.projected_mesh[i[0]],
                    b: cube.projected_mesh[i[1]],
                    c: cube.projected_mesh[i[2]],
                    depth: rotated[i[0]].z + rotated[i[1]].z + rotated[i[2]].z,
                    color: shade
                });
            }
        }

        triangles.sort_unstable_by(|y, x| x.depth.partial_cmp(&y.depth).unwrap());

        for i in &triangles {
            i.draw_face(&mut buffer, i.color);
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap(); //.expect("Oops!");
    }
}

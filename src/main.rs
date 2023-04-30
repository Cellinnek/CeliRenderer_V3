use std::alloc::System;
use std::f32::consts::PI;

#[global_allocator]
static A: System = System;
extern crate core;

use minifb::{Key, MouseMode, Scale, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const FOV: f32 = 2.0;

mod functions;
use functions::*;

fn main() {
    let mut camera = Camera{
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let mut fi:f32 = 0.0;
    let mut di:f32 = 0.0;

    let mut cube = Obj {
        mesh: vec![],
        faces: vec![],
        projected_mesh: vec![]
    };

    cube.load_from_file("C:/Users/Cysie/CLionProjects/CeliRenderer_V3/src/monke.obj");

    let mut window = Window::new("Renderer", WIDTH, HEIGHT, WindowOptions{
        scale: Scale::X1,
        ..WindowOptions::default()
    }
    ).unwrap();


    window.set_position(360, 0);
    /*window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));*/
    let mut mx = window.get_mouse_pos(MouseMode::Pass).unwrap().0 + 400.0;
    let mut my = window.get_mouse_pos(MouseMode::Pass).unwrap().1 + 400.0;

    while window.is_open() {
        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
        let mut triangles:Vec<Triangle> = vec![];
        let mut rotated = vec![Vec3{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }; cube.mesh.len()];
        cube.projected_mesh = vec![[0,0]; cube.mesh.len()];

        mx -= window.get_mouse_pos(MouseMode::Pass).unwrap().0;
        my -= window.get_mouse_pos(MouseMode::Pass).unwrap().1;

        for i in window.get_keys() {
            match i {
                Key::W => {
                    camera.x += 0.01 * fi.sin();
                    camera.z += 0.01 * fi.cos();
                }
                Key::S => {
                    camera.x -= 0.01 * fi.sin();
                    camera.z -= 0.01 * fi.cos();
                }
                Key::D => {
                    camera.x += 0.01 * fi.cos();
                    camera.z -= 0.01 * fi.sin();
                }
                Key::A => {
                    camera.x -= 0.01 * fi.cos();
                    camera.z += 0.01 * fi.sin();
                }
                Key::Space => {
                    camera.y -= 0.01;
                }
                Key::LeftShift => {
                    camera.y += 0.01;
                }
                Key::Q => {
                    fi -= 0.005;
                }
                Key::E => {
                    fi += 0.005;
                }
                Key::Up => {
                    di -= 0.005;
                }
                Key::Down => {
                    di += 0.005;
                }
                _ => ()

            }
        }

        for i in &cube.faces {
            if rotated[i[0]].z == 0.0 {
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
            }
            if rotated[i[1]].z == 0.0 {
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
            }
            if rotated[i[2]].z == 0.0 {
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
            }

            let mut normal = normal(&rotated[i[0]],&rotated[i[1]],&rotated[i[2]]);

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
                    depth: rotated[i[0]].z + rotated[i[1]].z + rotated[i[2]].z,
                    color: dp
                });
            }
        }

        triangles.sort_unstable_by(|y, x| x.depth.partial_cmp(&y.depth).unwrap());

        for i in &triangles {
            if i.depth > 0.0 {
                i.draw_face(&mut buffer, i.color);
            }
        }

        fi += (mx)/300.0;
        di += (my)/300.0;
        (mx, my) = window.get_mouse_pos(MouseMode::Pass).unwrap();
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap(); //.expect("Oops!");
    }
}

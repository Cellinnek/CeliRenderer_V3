use std::time::{Instant};
use minifb::{Key, Scale, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

mod functions;
use functions::{Obj, Vec3, draw_triangle, normal};

fn main() {
    let mut fov: f32 = -2.0;
    let mut camera = Vec3{
        x: 0.0,
        y: 0.0,
        z: -6.0,
    };

    let mut fi:f32 = 0.0;
    let mut di:f32 = 0.0;

    let mut cube = Obj {
        mesh: vec![],
        faces: vec![],
        projected_mesh: vec![]
    };

    cube.load_from_file("C:/Users/Cystic/CLionProjects/CeliRenderer_V3/src/pot.obj");

    let mut transformed = vec![Vec3{
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }; cube.mesh.len()];

    let mut window = Window::new("Renderer", WIDTH, HEIGHT, WindowOptions{
        scale: Scale::X1,
        ..WindowOptions::default()
    }
    ).unwrap();

    window.set_position(360, 0);
    /*window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));*/
    let mut avr = 0.0;
    while window.is_open() {
        let start = Instant::now();
        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
        let mut zbuffer: Vec<f32> = vec![0.0; WIDTH * HEIGHT];
        let mut projected_index = vec![false; cube.mesh.len()];

        for key in window.get_keys() {
            match key {
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

        for (i,j) in cube.mesh.iter().enumerate(){
            transformed[i] = &(j.rotate(&camera, fi, 1).rotate(&camera, di, 0)) - &camera;
        }

        let mut light_direction = Vec3 {
            x: 1.5,
            y: 1.5,
            z: -1.0,
        };
        light_direction = light_direction.normalise();

        light_direction = light_direction.rotate(&Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }, fi, 1).rotate(&Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }, di, 0);

        for triangle_indexes in &cube.faces {
            if transformed[triangle_indexes[0]].z < 0.0 && transformed[triangle_indexes[1]].z < 0.0 && transformed[triangle_indexes[2]].z < 0.0{
                continue;
            };
            let normal = normal(&transformed[triangle_indexes[0]], &transformed[triangle_indexes[1]], &transformed[triangle_indexes[2]]);
            if (&normal * &transformed[triangle_indexes[0]]) < 0.0 &&
                !(transformed[triangle_indexes[0]].x > transformed[triangle_indexes[0]].z/-fov &&
                    transformed[triangle_indexes[1]].x > transformed[triangle_indexes[1]].z/-fov &&
                    transformed[triangle_indexes[2]].x > transformed[triangle_indexes[2]].z/-fov) &&
                !(transformed[triangle_indexes[0]].x < transformed[triangle_indexes[0]].z/fov &&
                    transformed[triangle_indexes[1]].x < transformed[triangle_indexes[1]].z/fov &&
                    transformed[triangle_indexes[2]].x < transformed[triangle_indexes[2]].z/fov) &&
                !(transformed[triangle_indexes[0]].y > transformed[triangle_indexes[0]].z/-fov &&
                    transformed[triangle_indexes[1]].y > transformed[triangle_indexes[1]].z/-fov &&
                    transformed[triangle_indexes[2]].y > transformed[triangle_indexes[2]].z/-fov) &&
                !(transformed[triangle_indexes[0]].y < transformed[triangle_indexes[0]].z/fov &&
                    transformed[triangle_indexes[1]].y < transformed[triangle_indexes[1]].z/fov &&
                    transformed[triangle_indexes[2]].y < transformed[triangle_indexes[2]].z/fov) {
                for &vert_index in triangle_indexes {
                    if !projected_index[vert_index] {
                        cube.projected_mesh[vert_index] = transformed[vert_index].project(fov);
                        projected_index[vert_index] = true;
                    }
                }

                let shade = ((200.0 * (&normal * &light_direction)) as u32 + 25) * 0x010101;

                draw_triangle(&mut buffer, &mut zbuffer,
                              [
                             cube.projected_mesh[triangle_indexes[0]],
                             cube.projected_mesh[triangle_indexes[1]],
                             cube.projected_mesh[triangle_indexes[2]]],
                              [
                             transformed[triangle_indexes[0]].z,
                             transformed[triangle_indexes[1]].z,
                             transformed[triangle_indexes[2]].z], shade);
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        window.set_title(&(1.0/start.elapsed().as_secs_f32()).to_string());
        avr = (avr + (1.0/start.elapsed().as_secs_f32()))/2.0;
    }
    println!("{}",avr*2.0);
}
use minifb::{Key, Scale, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

mod functions;
use functions::*;

fn main() {
    let mut fov: f32 = -2.0;
    let mut camera = Vec3{
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

    cube.load_from_file("C:/Users/Cysie/CLionProjects/CeliRenderer_V3/src/axis.obj");

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

    while window.is_open() {
        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
        let mut triangles:Vec<Triangle> = vec![];
        let mut transformed_index = vec![false; cube.mesh.len()];
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
            if !transformed_index[i[0]] {
                transformed[i[0]] = cube.mesh[i[0]].rotate(Vec3 {
                    x: camera.x,
                    y: camera.y,
                    z: camera.z,
                }, fi, 1).rotate(Vec3 {
                    x: camera.x,
                    y: camera.y,
                    z: camera.z,
                }, di, 0);

                transformed[i[0]].x -= camera.x;
                transformed[i[0]].y -= camera.y;
                transformed[i[0]].z -= camera.z;

                transformed_index[i[0]] = true;
            }
            if !transformed_index[i[1]] {
                transformed[i[1]] = cube.mesh[i[1]].rotate(Vec3 {
                    x: camera.x,
                    y: camera.y,
                    z: camera.z,
                }, fi, 1).rotate(Vec3 {
                    x: camera.x,
                    y: camera.y,
                    z: camera.z,
                }, di, 0);

                transformed[i[1]].x -= camera.x;
                transformed[i[1]].y -= camera.y;
                transformed[i[1]].z -= camera.z;

                transformed_index[i[1]] = true;
            }
            if !transformed_index[i[2]] {
                transformed[i[2]] = cube.mesh[i[2]].rotate(Vec3 {
                    x: camera.x,
                    y: camera.y,
                    z: camera.z,
                }, fi, 1).rotate(Vec3 {
                    x: camera.x,
                    y: camera.y,
                    z: camera.z,
                }, di, 0);

                transformed[i[2]].x -= camera.x;
                transformed[i[2]].y -= camera.y;
                transformed[i[2]].z -= camera.z;

                transformed_index[i[2]] = true;
            }

            let normal = normal(&transformed[i[0]], &transformed[i[1]], &transformed[i[2]]);
            if ((dot(&normal,&transformed[i[0]])) < 0.0) &&
                !(transformed[i[0]].x > transformed[i[0]].z/-fov &&
                    transformed[i[1]].x > transformed[i[1]].z/-fov &&
                    transformed[i[2]].x > transformed[i[2]].z/-fov) &&
                !(transformed[i[0]].x < transformed[i[0]].z/fov &&
                    transformed[i[1]].x < transformed[i[1]].z/fov &&
                    transformed[i[2]].x < transformed[i[2]].z/fov) &&
                !(transformed[i[0]].y > transformed[i[0]].z/-fov &&
                    transformed[i[1]].y > transformed[i[1]].z/-fov &&
                    transformed[i[2]].y > transformed[i[2]].z/-fov) &&
                !(transformed[i[0]].y < transformed[i[0]].z/fov &&
                    transformed[i[1]].y < transformed[i[1]].z/fov &&
                    transformed[i[2]].y < transformed[i[2]].z/fov){

                if !projected_index[i[0]] {
                    cube.projected_mesh[i[0]] = transformed[i[0]].project(fov);
                    projected_index[i[0]] = true;
                }
                if !projected_index[i[1]] {
                    cube.projected_mesh[i[1]] = transformed[i[1]].project(fov);
                    projected_index[i[1]] = true;
                }
                if !projected_index[i[2]] {
                    cube.projected_mesh[i[2]] = transformed[i[2]].project(fov);
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
                }, fi, 1).rotate(Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }, di, 0);

                let shade = ((200.0 * dot(&normal, &light_direction)) as u32 + 25) * 0x010101;

                triangles.push(Triangle{
                    a: cube.projected_mesh[i[0]],
                    b: cube.projected_mesh[i[1]],
                    c: cube.projected_mesh[i[2]],
                    depth: transformed[i[0]].z + transformed[i[1]].z + transformed[i[2]].z,
                    color: shade
                });
            }
        }

        triangles.sort_unstable_by(|y, x| x.depth.partial_cmp(&y.depth).unwrap());

        for i in &triangles {
            i.draw_face(&mut buffer, i.color);
        }

        line(&mut buffer, [WIDTH as i32/4,HEIGHT as i32/4], [WIDTH as i32/4,3*HEIGHT as i32/4], 0xff0000);
        line(&mut buffer, [WIDTH as i32/4,HEIGHT as i32/4], [3*WIDTH as i32/4,HEIGHT as i32/4], 0xff0000);
        line(&mut buffer, [3*WIDTH as i32/4,3*HEIGHT as i32/4], [WIDTH as i32/4,3*HEIGHT as i32/4], 0xff0000);
        line(&mut buffer, [3*WIDTH as i32/4,3*HEIGHT as i32/4], [3*WIDTH as i32/4,HEIGHT as i32/4], 0xff0000);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
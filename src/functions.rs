use crate::HEIGHT;
use crate::WIDTH;
use std::fs;

#[allow(dead_code)]
pub fn line(buffer: &mut [u32], [x1, y1]: [i32; 2], [x2, y2]: [i32; 2], color: u32)
{
    if (x1 >= WIDTH as i32 || x1 < 0)
        && (x2 >= WIDTH as i32 || x2 < 0)
    {
        return;
    }
    if (y1 >= HEIGHT as i32 || y1 < 0)
        && (y2 >= HEIGHT as i32 || y2 < 0)
    {
        return;
    }
    let mut x = x1;
    let mut y = y1;

    let dx = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    let dy = if y1 > y2 { y1 - y2 } else { y2 - y1 };

    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };

    let mut err = if dx > dy { dx } else { -dy } / 2;
    let mut err_tolerance;

    loop {
        if x < WIDTH as i32 && y < HEIGHT as i32 && x > 0 && y > 0 {
            buffer[((y as usize) * (WIDTH)) + x as usize] = color;
        };

        if x == x2 && y == y2 {
            break;
        };

        err_tolerance = err;

        if err_tolerance > -dx {
            err -= dy;
            x += sx;
        }
        if err_tolerance < dy {
            err += dx;
            y += sy;
        }
    }
}

#[allow(non_snake_case)]
pub fn triangle(
    buffer: &mut [u32],
    [x1, y1]: [i32; 2],
    [x2, y2]: [i32; 2],
    [x3, y3]: [i32; 2],
    color: u32)
{
    if (x1 >= WIDTH as i32 || x1 < 0)
        && (x2 >= WIDTH as i32 || x2 < 0)
        && (x3 >= WIDTH as i32 || x3 < 0)
    {
        return;
    }
    if (y1 >= HEIGHT as i32 || y1 < 0)
        && (y2 >= HEIGHT as i32 || y2 < 0)
        && (y3 >= HEIGHT as i32 || y3 < 0)
    {
        return;
    }

    let X1 = 16 * x1;
    let X2 = 16 * x2;
    let X3 = 16 * x3;

    let Y1 = 16 * y1;
    let Y2 = 16 * y2;
    let Y3 = 16 * y3;

    let DX12 = X1 - X2;
    let DX23 = X2 - X3;
    let DX31 = X3 - X1;

    let DY12 = Y1 - Y2;
    let DY23 = Y2 - Y3;
    let DY31 = Y3 - Y1;

    let FDX12 = DX12 << 4;
    let FDX23 = DX23 << 4;
    let FDX31 = DX31 << 4;

    let FDY12 = DY12 << 4;
    let FDY23 = DY23 << 4;
    let FDY31 = DY31 << 4;

    unsafe {
        let mut minx = (*[X1, X2, X3].iter().min().unwrap_unchecked() + 0xF) >> 4;
        let maxx = (*[X1, X2, X3].iter().max().unwrap_unchecked() + 0xF) >> 4;
        let mut miny = (*[Y1, Y2, Y3].iter().min().unwrap_unchecked() + 0xF) >> 4;
        let maxy = (*[Y1, Y2, Y3].iter().max().unwrap_unchecked() + 0xF) >> 4;

        let mut q = if (maxx - minx) > (maxy - miny) {
            maxx - minx + 1
        } else {
            maxy - miny + 1
        };
        q |= q >> 16;
        q |= q >> 8;
        q |= q >> 4;
        q |= q >> 2;
        q |= q >> 1;
        q ^= q >> 1;

        minx &= !(q - 1);
        miny &= !(q - 1);

        let mut C1 = DY12 * X1 - DX12 * Y1;
        let mut C2 = DY23 * X2 - DX23 * Y2;
        let mut C3 = DY31 * X3 - DX31 * Y3;

        if DY12 < 0 || (DY12 == 0 && DX12 > 0) {
            C1 += 1;
        }
        if DY23 < 0 || (DY23 == 0 && DX23 > 0) {
            C2 += 1;
        }
        if DY31 < 0 || (DY31 == 0 && DX31 > 0) {
            C3 += 1;
        }

        for y in (miny..maxy).step_by(q as usize) {
            for x in (minx..maxx).step_by(q as usize) {
                let x0 = x << 4;
                let x1 = (x + q - 1) << 4;
                let y0 = y << 4;
                let y1 = (y + q - 1) << 4;

                let a00 = (C1 + DX12 * y0 - DY12 * x0 > 0) as u8;
                let a10 = (C1 + DX12 * y0 - DY12 * x1 > 0) as u8;
                let a01 = (C1 + DX12 * y1 - DY12 * x0 > 0) as u8;
                let a11 = (C1 + DX12 * y1 - DY12 * x1 > 0) as u8;
                let a = a00 | (a10 << 1) | (a01 << 2) | (a11 << 3);

                let b00 = (C2 + DX23 * y0 - DY23 * x0 > 0) as u8;
                let b10 = (C2 + DX23 * y0 - DY23 * x1 > 0) as u8;
                let b01 = (C2 + DX23 * y1 - DY23 * x0 > 0) as u8;
                let b11 = (C2 + DX23 * y1 - DY23 * x1 > 0) as u8;
                let b = b00 | (b10 << 1) | (b01 << 2) | (b11 << 3);

                let c00 = (C3 + DX31 * y0 - DY31 * x0 > 0) as u8;
                let c10 = (C3 + DX31 * y0 - DY31 * x1 > 0) as u8;
                let c01 = (C3 + DX31 * y1 - DY31 * x0 > 0) as u8;
                let c11 = (C3 + DX31 * y1 - DY31 * x1 > 0) as u8;
                let c = c00 | (c10 << 1) | (c01 << 2) | (c11 << 3);

                if a == 0x0 || b == 0x0 || c == 0x0 {
                    continue;
                }

                if a == 0xF && b == 0xF && c == 0xF {
                    for iy in 0..q {
                        for ix in x..(x + q) {
                            if ix >= WIDTH as i32 || iy >= HEIGHT as i32 || ix < 0 || iy < 0 {
                                continue;
                            }
                            buffer[(ix + iy * WIDTH as i32) as usize] = color;
                        }
                    }
                } else {
                    let mut CY1 = C1 + DX12 * y0 - DY12 * x0;
                    let mut CY2 = C2 + DX23 * y0 - DY23 * x0;
                    let mut CY3 = C3 + DX31 * y0 - DY31 * x0;

                    for iy in y..(y + q) {
                        let mut CX1 = CY1;
                        let mut CX2 = CY2;
                        let mut CX3 = CY3;

                        for ix in x..(x + q) {
                            if CX1 > 0 && CX2 > 0 && CX3 > 0 {
                                if ix >= WIDTH as i32 || iy >= HEIGHT as i32 || ix < 0 || iy < 0 {
                                    continue;
                                }
                                buffer[(ix + iy * WIDTH as i32) as usize] = color;
                            }

                            CX1 -= FDY12;
                            CX2 -= FDY23;
                            CX3 -= FDY31;
                        }

                        CY1 += FDX12;
                        CY2 += FDX23;
                        CY3 += FDX31;
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn project(&self, f: f64) -> [i32; 2] {
        [
            (((self.x * f) / self.z + 1.0) * WIDTH as f64 / 2.0) as i32,
            (((self.y * f) / self.z + 1.0) * HEIGHT as f64 / 2.0) as i32,
        ]
    }

    pub fn normalise(&mut self) {
        let l = (self.x * self.x
            + self.y * self.y
            + self.z * self.z)
            .sqrt();

        self.x /= l;
        self.y /= l;
        self.z /= l;
    }

    pub fn rotate (&mut self, r: Vec3, fi: f64, axis: u8) -> Vec3{
        match axis % 3 {
            0 => {
                let (y, z) = (self.y - r.y, self.z - r.z);
                Vec3{
                    x: self.x,
                    y: z * fi.sin() + y * fi.cos() + r.y,
                    z: z * fi.cos() - y * fi.sin() + r.z,
                }
            }
            1 => {
                let (x, z) = (self.x - r.x, self.z - r.z);
                Vec3{
                    x: x * fi.cos() - z * fi.sin() + r.x,
                    y: self.y,
                    z: x * fi.sin() + z * fi.cos() + r.z,
                }
            }
            2 => {
                let (x, y) = (self.x - r.x, self.y - r.y);
                Vec3{
                    x: y * fi.sin() + x * fi.cos() + r.x,
                    y: y * fi.cos() - x * fi.sin() + r.y,
                    z: self.z,
                }
            }
            _ => Vec3{
                x: self.x,
                y: self.y,
                z: self.z,
            },
        }
    }
}

pub fn dot(a: &Vec3, b: &Vec3) -> f64 {a.x * b.x + a.y * b.y + a.z * b.z}

pub fn normal(a: &Vec3, b: &Vec3, c: &Vec3) -> Vec3{
    let line1 = Vec3 {
        x: b.x - a.x,
        y: b.y - a.y,
        z: b.z - a.z,
    };
    let line2 = Vec3 {
        x: c.x - a.x,
        y: c.y - a.y,
        z: c.z - a.z,
    };
    let mut normal = Vec3 {
        x: line1.y * line2.z - line1.z * line2.y,
        y: line1.z * line2.x - line1.x * line2.z,
        z: line1.x * line2.y - line1.y * line2.x,
    };

    normal.normalise();
    normal
}

pub struct Obj {pub mesh: Vec<Vec3>, pub faces: Vec<[usize; 3]>, pub projected_mesh: Vec<[i32; 2]>}

impl Obj {
    pub fn load_from_file(&mut self, path: &str) {
        let file = fs::read_to_string(path).unwrap();
        let split = file.split('\n');

        for s in split {
            match s.split_whitespace().next() {
                Some("v") => self.mesh.push(Vec3 {
                    x: s.split_whitespace().nth(1).unwrap().parse::<f64>().unwrap(),
                    y: s.split_whitespace().nth(2).unwrap().parse::<f64>().unwrap(),
                    z: s.split_whitespace().nth(3).unwrap().parse::<f64>().unwrap() + 6.0
                }),
                Some("f") => self.faces.push([
                    s.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap() - 1,
                    s.split_whitespace().nth(2).unwrap().parse::<usize>().unwrap() - 1,
                    s.split_whitespace().nth(3).unwrap().parse::<usize>().unwrap() - 1,
                ]),
                _ => ()
            }
        }
    }
}

#[derive(Clone)]
pub struct  Triangle{
    pub a: [i32;2],
    pub b: [i32;2],
    pub c: [i32;2],
    pub depth: f64,
    pub color: u32,
}

impl Triangle{
    pub fn draw_face(&self, buffer: &mut [u32], color: u32){
        triangle(buffer, self.a, self.b, self.c, color);
    }

    #[allow(dead_code)]
    pub fn draw_edges(&self, buffer: &mut [u32], color: u32){
        line(buffer, self.a, self.b, color);
        line(buffer, self.b, self.c, color);
        line(buffer, self.c, self.a, color);
    }
}

pub struct Camera{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
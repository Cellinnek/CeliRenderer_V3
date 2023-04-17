use std::fs;
use crate::WIDTH;
use crate::HEIGHT;
/*use std::mem::swap;*/

pub fn line(buffer: &mut [u32], [x1, y1]: [i32; 2], [x2, y2]: [i32; 2], color: u32) {
    let mut x = x1;
    let mut y = y1;

    let dx = if x1 > x2 {
        x1 - x2
    } else {
        x2 - x1
    };
    let dy = if y1 > y2 {
        y1 - y2
    } else {
        y2 - y1
    };

    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };

    let mut err = if dx > dy { dx } else { -dy } / 2;
    let mut err_tolerance;

    loop {
        if x<WIDTH as i32 && y<HEIGHT as i32 && x>0 && y>0{
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

/*pub fn triangle(buffer: &mut [u32], [mut x1,mut y1]: [i32; 2], [mut x2,mut y2]: [i32; 2], [mut x3,mut y3]: [i32; 2], color:u32){
    let height = HEIGHT as i32;
    let width = WIDTH as i32;

    if y2 > y3
    {
        swap(&mut x2,&mut x3);
        swap(&mut y2,&mut y3);
    }
    if y1 > y2
    {
        swap(&mut x1,&mut x2);
        swap(&mut y1,&mut y2);
    }
    if y2 > y3
    {
        swap(&mut x2,&mut x3);
        swap(&mut y2,&mut y3);
    }

    let dx_far = (x3 - x1) as f64/ (y3 - y1) as f64;
    let dx_upper = (x2 - x1) as f64 / (y2 - y1 + 1) as f64;
    let dx_low = (x3 - x2) as f64 / (y3 - y2) as f64;
    let mut xf = x1 as f64;
    let mut xt = x1 as f64 + dx_upper;
    for y in y1..(if y3<height-1{y3} else{height}) {
        if y >= 0 {
            for x in (if xf>0.0{xf as i32} else{0})..(if xt < (width-1) as f64{xt as i32} else{width-1}){
                buffer[(x+y*width) as usize] = color;
            }
            for x in (if xt > 0.0{xt as i32} else{0})..(if xf<width as f64{xf as i32} else{width-1}){
                buffer[(x+y*width) as usize] = color;
            }
        }
        xf += dx_far;
        if y < y2{xt += dx_upper;}
        else{ xt += dx_low;}
    }
}
*/

#[allow(non_snake_case)]
pub fn triangle(buffer: &mut [u32], [x1, y1]: [i32; 2], [x2, y2]: [i32; 2], [x3, y3]: [i32; 2], color:u32){
    let Y1 = (16.0*y1 as f64).round() as i64;
    let Y2 = (16.0*y2 as f64).round() as i64;
    let Y3 = (16.0*y3 as f64).round() as i64;

    let X1 = (16.0*x1 as f64).round() as i64;
    let X2 = (16.0*x2 as f64).round() as i64;
    let X3 = (16.0*x3 as f64).round() as i64;

    let DX12 = X1-X2;
    let DX23 = X2-X3;
    let DX31 = X3-X1;

    let DY12 = Y1-Y2;
    let DY23 = Y2-Y3;
    let DY31 = Y3-Y1;

    let FDX12 = DX12 << 4;
    let FDX23 = DX23 << 4;
    let FDX31 = DX31 << 4;

    let FDY12 = DY12 << 4;
    let FDY23 = DY23 << 4;
    let FDY31 = DY31 << 4;

    let mut minx = (*[X1,X2,X3].iter().min().unwrap() + 0xF) >> 4;
    let maxx = (*[X1,X2,X3].iter().max().unwrap() + 0xF) >> 4;
    let mut miny = (*[Y1,Y2,Y3].iter().min().unwrap() + 0xF) >> 4;
    let maxy = (*[Y1,Y2,Y3].iter().max().unwrap() + 0xF) >> 4;

    let q = 8;

    minx &= !(q-1);
    miny &= !(q-1);

    let mut C1 = DY12 * X1 - DX12 * Y1;
    let mut C2 = DY23 * X2 - DX23 * Y2;
    let mut C3 = DY31 * X3 - DX31 * Y3;

    if DY12 < 0 || (DY12 == 0 && DX12 > 0) {C1 += 1;}
    if DY23 < 0 || (DY23 == 0 && DX23 > 0) {C2 += 1;}
    if DY31 < 0 || (DY31 == 0 && DX31 > 0) {C3 += 1;}

    for y in (miny..maxy).step_by(q as usize){
        for x in (minx..maxx).step_by(q as usize){
            let x0 = x << 4;
            let x1 = (x + q - 1) << 4;
            let y0 = y << 4;
            let y1 = (y + q - 1) << 4;

            let a00 = (C1 + DX12 * y0 - DY12 * x0 > 0) as i32;
            let a10 = (C1 + DX12 * y0 - DY12 * x1 > 0) as i32;
            let a01 = (C1 + DX12 * y1 - DY12 * x0 > 0) as i32;
            let a11 = (C1 + DX12 * y1 - DY12 * x1 > 0) as i32;
            let a = a00 | (a10 << 1) | (a01 << 2) | (a11 << 3);

            let b00 = (C2 + DX23 * y0 - DY23 * x0 > 0) as i32;
            let b10 = (C2 + DX23 * y0 - DY23 * x1 > 0) as i32;
            let b01 = (C2 + DX23 * y1 - DY23 * x0 > 0) as i32;
            let b11 = (C2 + DX23 * y1 - DY23 * x1 > 0) as i32;
            let b = b00 | (b10 << 1) | (b01 << 2) | (b11 << 3);

            let c00 = (C3 + DX31 * y0 - DY31 * x0 > 0) as i32;
            let c10 = (C3 + DX31 * y0 - DY31 * x1 > 0) as i32;
            let c01 = (C3 + DX31 * y1 - DY31 * x0 > 0) as i32;
            let c11 = (C3 + DX31 * y1 - DY31 * x1 > 0) as i32;
            let c = c00 | (c10 << 1) | (c01 << 2) | (c11 << 3);

            if a == 0x0 || b == 0x0 || c == 0x0 {continue;}

            if a == 0xF && b == 0xF && c == 0xF {
                for iy in 0..q{
                    for ix in x..(x+q){
                        buffer[(ix+iy*WIDTH as i64) as usize] = color;
                    }
                }
            }
            else{
                let mut CY1 = C1 + DX12 * y0 - DY12 * x0;
                let mut CY2 = C2 + DX23 * y0 - DY23 * x0;
                let mut CY3 = C3 + DX31 * y0 - DY31 * x0;

                for iy in y..(y+q){
                    let mut CX1 = CY1;
                    let mut CX2 = CY2;
                    let mut CX3 = CY3;

                    for ix in x..(x+q){
                        if CX1 > 0 && CX2 > 0 && CX3 > 0 {
                            buffer[(ix+iy*WIDTH as i64) as usize] = color;
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

pub struct Vec3{
    pub x: f64,
    pub y: f64,
    pub z: f64
}
impl Vec3{
    pub fn project(&self, f: f64) -> [i32;2]{
        [(((self.x*f)/self.z+1.0)*WIDTH as f64/2.0) as i32,(((self.y*f)/self.z+1.0)*HEIGHT as f64/2.0) as i32]
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    a.x*b.x+a.y*b.y+a.x*b.y
}

pub struct Obj{
    pub mesh: Vec<Vec3>,
    pub faces: Vec<[usize;3]>,
}

impl Obj{
    pub fn load_from_file(&mut self, path: &str){
        let file = fs::read_to_string(path).unwrap();
        let split = file.split('\n');

        for s in split {
            if s.split_whitespace().next().unwrap() == "v" {
                self.mesh.push(Vec3 {
                    x: s.split_whitespace().nth(1).unwrap().parse::<f64>().unwrap()*-1.0,
                    y: s.split_whitespace().nth(2).unwrap().parse::<f64>().unwrap()*-1.0,
                    z: s.split_whitespace().nth(3).unwrap().parse::<f64>().unwrap()+4.0
                });
            }

            if s.split_whitespace().next().unwrap() == "f" {
                self.faces.push([
                    s.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap() - 1,
                    s.split_whitespace().nth(2).unwrap().parse::<usize>().unwrap() - 1,
                    s.split_whitespace().nth(3).unwrap().parse::<usize>().unwrap() - 1
                ]);
            }
        }
    }

}
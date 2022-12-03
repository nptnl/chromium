use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use ferrum::{ch::Comp, alg::ixp};

static DIM: i16 = 32;
const INDX: usize = 32;

#[derive(Debug, Copy, Clone)]
pub struct Co2D {
    pub x: i16,
    pub y: i16,
}
impl Co2D {
    pub fn new(x: i16, y: i16) -> Co2D {
        Co2D { x, y }
    }
    pub fn domain(self) -> bool {
        if self.x > -DIM && self.x < DIM
        && self.y > -DIM && self.y < DIM
        { true } else { false }
    }
    pub fn swap(self) -> Co2D {
        Co2D { x: self.y, y: self.x }
    }
    pub fn rotate(self, angle: f32) -> Co2D {
        let value = Comp::new(self.x as f32, self.y as f32) * ixp(Comp::new(angle, 0.0));
        Co2D { x: (value.r + 0.5) as i16, y: (value.i + 0.5) as i16 }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Co3D {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}
impl Co3D {
    pub fn new(x: i16, y: i16, z: i16) -> Co3D {
        Co3D { x, y, z }
    }
    pub fn domain(self) -> bool {
        if self.x > -DIM && self.x < DIM
        && self.y > -DIM && self.y < DIM
        && self.z > -DIM && self.z < DIM 
        { true } else { false }
    }
    pub fn project(self, focal: i16) -> Co2D {
        let px: i16 = ((self.x * focal) as f32 / (self.z + focal) as f32) as i16;
        let py: i16 = ((self.z * focal) as f32 / (self.z + focal) as f32) as i16;
        Co2D { x: px, y: py }
    }
}

fn line_grad(c1: Co2D, c2: Co2D) -> Vec<Co2D> {
    let slope: f32 = (c2.y - c1.y) as f32 / (c2.x - c1.x) as f32;
    let mut current: Co2D = c1;
    let mut outlist: Vec<Co2D> = Vec::new();
    let mut error: f32 = if c1.y < c2.y {0.5} else {-0.5};
    while current.x != c2.x {
        outlist.push(current);
        current.x += 1;
        error += slope;
        if error >= 1.0 {
            error -= 1.0;
            current.y += 1;
        } else if error <= -1.0 {
            error += 1.0;
            current.y -= 1;
        }
    }
    outlist.push(c2);
    outlist
}
fn line_steep(c1: Co2D, c2: Co2D) -> Vec<Co2D> {
    let slope: f32 = (c2.x - c1.x) as f32 / (c2.y - c1.y) as f32;
    let mut current: Co2D = c1.swap();
    let mut outlist: Vec<Co2D> = Vec::new();
    let mut error: f32 = if c1.y < c2.y {0.5} else {-0.5};
    while current.y != c2.y {
        outlist.push(current);
        current.y += 1;
        error += slope;
        if error >= 1.0 {
            error -= 1.0;
            current.x += 1;
        } else if error <= -1.0 {
            error += 1.0;
            current.y -= 1;
        }
    }
    outlist.push(c2);
    outlist
}
fn line_horiz(c1: Co2D, c2: Co2D) -> Vec<Co2D> {
    let mut outlist: Vec<Co2D> = Vec::new();
    for x in c1.x..c2.x {
        outlist.push(Co2D { x, y: c1.y });
    }
    outlist.push(c2);
    outlist
}
fn line_vert(c1: Co2D, c2: Co2D) -> Vec<Co2D> {
    let mut outlist: Vec<Co2D> = Vec::new();
    for y in c1.y..c2.y {
        outlist.push(Co2D { x: c1.x, y });
    }
    outlist.push(c2);
    outlist
}
pub fn line(c1: Co2D, c2: Co2D) -> Vec<Co2D> {
    if c1.y == c2.y {
        if c1.x < c2.x { return line_horiz(c1, c2) } else { return line_horiz(c2, c1) }
    } else if c1.x == c2.x {
        if c1.y < c2.y { return line_vert(c1, c2) } else { return line_vert(c2, c1) }
    } else if (c2.x - c1.x).abs() >= (c2.y - c1.y).abs() {
        if c1.x < c2.x { return line_grad(c1, c2) } else { return line_grad(c2, c1) }
    } else {
        if c1.y < c2.y { return line_steep(c1, c2) } else { return line_steep(c2, c1) }
    }
}

pub struct Wire2D {
    pub vtx: Vec<Co2D>,
    pub cnx: Vec<(usize, usize)>,
}
impl Wire2D {
    pub fn lines(self) -> Vec<Co2D> {
        let mut outlist: Vec<Co2D> = Vec::new();
        for connect in self.cnx {
            outlist.append( &mut line(self.vtx[connect.0], self.vtx[connect.1]) )
        }
        outlist
    }
    pub fn rotate(mut self, angle: f32) -> Wire2D {
        for indx in 0..self.vtx.len() {
            self.vtx[indx] = self.vtx[indx].rotate(angle);
        };
        self
    }
}

pub fn plot(colist: Vec<Co2D>) {
    let path = Path::new("./plots/current.npxl");
    let mut file = File::create(&path).unwrap();
    let first = format!("{} {}\n", DIM*2, DIM*2) + "2 1\n";
    file.write_all(first.as_bytes()).unwrap();
    let mut outplot: [[u8; 2*INDX]; 2*INDX] = [[0; 2*INDX]; 2*INDX];
    for co in colist {
        if co.domain() {
            outplot[(co.y + DIM) as usize][(co.x + DIM) as usize] = 1;
        }
    }
    for pln in outplot {
        let mut working: String = String::new();
        for bol in pln {
            working += &bol.to_string();
        }
        file.write_all(working.as_bytes()).unwrap();
    }
}
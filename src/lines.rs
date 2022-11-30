use ferrum::{ch::Comp, alg::ixp};

static DIM: i16 = 32;

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

fn line_grad(c1: Co2D, c2: Co2D) -> Vec<Co2D> {
    let slope: f32 = (c2.y - c1.y) as f32 / (c2.x - c1.x) as f32;
    let mut current: Co2D = c1;
    let mut outlist: Vec<Co2D> = Vec::new();
    let mut error: f32 = 0.5;
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
    let mut error: f32 = 0.5;
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
        outlist.push(Co2D { x: c1.y, y });
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
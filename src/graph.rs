use crate::vertex::{Co2D, DIM};

static CENTER: (i16, i16) = (0, 0);
static STEP: (f32, f32) = (0.005, 0.005);

pub fn explicit() -> Vec<Co2D> {
    let y1 = | x: f32 | x*x*x*x/24.0 - x*x/2.0 + 1.0;
    let mut covec: Vec<Co2D> = Vec::new();
    let mut y: i16;
    let mut previous: i16;
    let mut x: i16 = CENTER.0 - DIM;
    previous = y_round(y1(x_unround(x)));
    while x < CENTER.0 + DIM {
        y = y_round(y1(x_unround(x)));
        while y - previous > 1 {
            previous += 1;
            covec.push(Co2D { x, y: -previous as i16 });
        }
        while y - previous < -1 {
            previous -= 1;
            covec.push(Co2D { x, y: -previous as i16 });
        }
        covec.push(Co2D { x, y: -y as i16 });
        x += 1;
        previous = y;
    }
    covec
}
pub fn parametric() -> Vec<Co2D> {
    let x1 = | t: f32 | t - 2.0;
    let y1 = | t: f32 | t*t + 3.0;
    let mut covec: Vec<Co2D> = Vec::new();
    let mut x: i16;
    let mut y: i16;
    unimplemented!()
}
fn y_round(raw: f32) -> i16 {
    ((raw - CENTER.1 as f32) / STEP.1) as i16
}
fn x_unround(raw: i16) -> f32 {
    (raw - CENTER.0) as f32 * STEP.0
}
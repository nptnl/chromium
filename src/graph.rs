use crate::vertex::{plot, term_plot, Co2D, DIM};

static CENTER: (i16, i16) = (0, 0);
static STEP: (i16, i16) = (1, 1);

pub fn explicit() -> Vec<Co2D> {
    let y1 = | x: f32 | x * (8.0 - x);
    let mut covec: Vec<Co2D> = Vec::new();
    let mut y: i16;
    let mut previous: i16;
    let mut x: i16 = CENTER.0 - DIM;
    previous = y_round(y1(x_unround(x)));
    while x < CENTER.0 + DIM {
        y = y_round(y1(x_unround(x)));
        println!("y = {}", y1(((x - CENTER.0) * STEP.0) as f32));
        while y - previous > 1 {
            previous += 1;
            covec.push(Co2D { x, y: -previous as i16 });
            println!("going up one");
        }
        while y - previous < -1 {
            previous -= 1;
            covec.push(Co2D { x, y: -previous as i16 });
            println!("going down one ({}, {})", x, y);
        }
        covec.push(Co2D { x, y: -y as i16 });
        x += 1;
        previous = y;
    }
    covec
}
fn y_round(raw: f32) -> i16 {
    (raw - CENTER.1 as f32) as i16 / STEP.1
}
fn x_unround(raw: i16) -> f32 {
    ((raw - CENTER.0) * STEP.0) as f32
}
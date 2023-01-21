use crate::vertex::{Co2D, DIM, term_plot};

static CENTER: (i16, i16) = (0, 0);
static STEP: (f32, f32) = (0.1, 0.1);

pub fn explicit() -> Vec<Co2D> {
    let y1 = | x: f32 | x * x * x - x;
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
fn y_round(raw: f32) -> i16 {
    ((raw - CENTER.1 as f32) / STEP.1) as i16
}
fn x_unround(raw: i16) -> f32 {
    (raw - CENTER.0) as f32 * STEP.0
}


use ferrum::ch::Comp;
use ferrum::trig::sin;
pub fn cool() {
    let y1 = | x: f32, multiplier: f32 | x * x * x + (2.0 * multiplier * x);
    let mut angle: Comp = Comp::new(-3.14, 0.0);
    loop {
        angle += 0.001;
        if angle.r >= 3.14 { angle -= 6.28 };
        let mut covec: Vec<Co2D> = Vec::new();
        let mut y: i16;
        let mut previous: i16;
        let mut x: i16 = CENTER.0 - DIM;
        previous = y_round(y1(x_unround(x), sin(angle).r));
        while x < CENTER.0 + DIM {
            y = y_round(y1(x_unround(x), sin(angle).r));
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
        term_plot(&covec);
    }
}
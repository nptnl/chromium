use crate::vertex::{plot, term_plot, Co2D};

static CENTER: (i16, i16) = (0, 0);

pub fn explicit() -> Vec<Co2D> {
    let y1 = | x: f32 | x * x;
    let mut covec: Vec<Co2D> = Vec::new();
    for x in -10..10 {
        covec.push(Co2D { x: x - 1, y: -(y1(x as f32) + 0.5) as i16 });
    }
    covec
}
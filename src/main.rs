pub mod vertex;
pub mod fractal;

use vertex::*;

fn main() {
    let square: Wire2D = Wire2D {
        vtx: vec![ Co2D { x: -16, y: -16 }, Co2D { x: -16, y: 16 }, Co2D { x: 16, y: -16 }, Co2D { x: 16, y: 16 } ],
        cnx: vec![ (0, 1), (0, 2), (1, 3), (2, 3) ],
    };
    plot(
        square.lines()
    )
}

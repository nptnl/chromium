pub mod vertex;
pub mod fractal;

use vertex::*;

fn main() {
    let _square: Wire2D = Wire2D {
        vtx: vec![ Co2D { x: -16, y: -16 }, Co2D { x: 16, y: -16 }, Co2D { x: -16, y: 16 }, Co2D { x: 16, y: 16 } ],
        cnx: vec![ (0, 1), (0, 2), (1, 3), (2, 3) ],
    };
    let cube: Wire3D = Wire3D {
        vtx: vec![
        Co3D::new(-16, -16, -16), Co3D::new( 16, -16, -16),
        Co3D::new( 16,  16, -16), Co3D::new(-16,  16, -16),
        Co3D::new(-16, -16,  16), Co3D::new( 16, -16,  16),
        Co3D::new( 16,  16,  16), Co3D::new(-16,  16,  16),
        ],
        cnx: vec![
        (0, 1), (1, 2), (2, 3), (3, 0),
        (4, 5), (5, 6), (6, 7), (7, 4),
        (0, 4), (1, 5), (2, 6), (3, 7),
        ],
    };
    plot(
        cube.rotate('x', 0.5).rotate('y', 0.5).lines()
    )
}

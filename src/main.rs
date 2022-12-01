pub mod lines;
pub mod fractal;

fn main() {
    lines::plot(
        lines::line( lines::Co2D::new(-16, -16), lines::Co2D::new(0, 16) )
    );
}

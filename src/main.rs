pub mod vertex;
pub mod fractal;
pub mod graph;

use ferrum::ch::Comp;

fn main() {
    fractal::ispace(Comp::new(0.46, 0.0), 8192, 128);
}
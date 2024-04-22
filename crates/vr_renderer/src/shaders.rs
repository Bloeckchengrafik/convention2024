pub mod multiplier {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "shaders/mandelbrot.glsl"
    }
}
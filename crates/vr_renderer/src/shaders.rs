pub mod deform {
    pub mod vertex {
        use std::sync::Arc;
        use vulkano::device::Device;
        use vulkano::shader::EntryPoint;

        vulkano_shaders::shader! {
            ty: "vertex",
            path: "shaders/deform.vert"
        }

        pub fn entrypoint(device: Arc<Device>) -> EntryPoint {
            return load(device).unwrap().entry_point("main").unwrap();
        }
    }

    pub mod fragment {
        use std::sync::Arc;
        use vulkano::device::Device;
        use vulkano::shader::EntryPoint;
        vulkano_shaders::shader! {
            ty: "fragment",
            path: "shaders/deform.frag"
        }

        pub fn entrypoint(device: Arc<Device>) -> EntryPoint {
            return load(device).unwrap().entry_point("main").unwrap();
        }
    }
}
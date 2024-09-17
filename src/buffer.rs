// use the Copy trait so a buffer can be created
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> { //static lifetime
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, // page size
            step_mode: wgpu::VertexStepMode::Vertex, // whether each element in array is per vertex or instance
            attributes: &[ //parts of the vertex. can also use the macro for this
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0, // like the index of the field in the struct,
                    format: wgpu::VertexFormat::Float32x3 //
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1, // like the index of the field in the struct,
                    format: wgpu::VertexFormat::Float32x3 //
                }
            ]
        }
    }
}

//counter clockwise order (compat w wgpu::FrontFace::Ccw)
pub const VERTICES: &[Vertex] = &[
    Vertex { position : [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position : [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position : [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];


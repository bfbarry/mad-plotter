// use the Copy trait so a buffer can be created
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    // shader needs descriptor to read data
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
    Vertex { position: [0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [0.5, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0] },
];

pub const INDICES: &[u16] = &[
    0, 1, 2, 
    0, 2, 3
];

pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position) 
                    * cgmath::Matrix4::from(self.rotation) 
                    * cgmath::Matrix4::from_scale(0.02)
                ).into(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub model: [[f32; 4]; 4]
}

impl InstanceRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ]
        }
    }
}
pub const NUM_INSTANCES_PER_ROW: u32 = 10;
pub const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(NUM_INSTANCES_PER_ROW as f32 * 0.5, NUM_INSTANCES_PER_ROW as f32 * 0.5, 0.0);
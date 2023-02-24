#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0], }, // A
    Vertex { position: [-0.5, 0.5, 0.0], tex_coords: [0.0, 0.0], }, // B
    Vertex { position: [0.5, 0.5, 0.0], tex_coords: [1.0, 0.0], }, // C
    Vertex { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 1.0], }, // D
];

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2, 
                },
            ],
        }
    }
}

use crate::graphics::geometries::Geometry;
use crate::graphics::pipelines::Pipeline;
use crate::graphics::vertices::Vertices;
use fj_interop::mesh::Color;
use fj_interop::mesh::Mesh;
use fj_math::Point;
use fj_math::Triangle;

#[derive(Debug)]
pub struct ReferenceAxes {
    arrow_geometry: Geometry,
}

impl ReferenceAxes {
    pub fn new(device: &wgpu::Device) -> Self {
        let mut mesh: Mesh<Point<3>> = Mesh::new();
        let triangle: Triangle<3> = Triangle::from_points([
            [0.0, 0.0, 0.0],
            [0.0, 10.0, 0.0],
            [10.0, 0.0, 0.0],
        ])
        .unwrap();

        mesh.push_triangle(triangle, Color::default());

        let vertices = Vertices::from(&mesh);
        let indices: Vec<_> = mesh.indices().collect();

        let arrow_geometry =
            Geometry::new(device, vertices.vertices(), &indices);

        Self { arrow_geometry }
    }

    pub fn draw(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        color_view: &wgpu::TextureView,
        depth_view: &wgpu::TextureView,
        bind_group: &wgpu::BindGroup,
        model_pipeline: &Pipeline,
    ) {
        let mut render_pass =
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: color_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: depth_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        }),
                        stencil_ops: None,
                    },
                ),
            });

        render_pass.set_pipeline(&model_pipeline.0);
        render_pass.set_bind_group(0, bind_group, &[]);
        render_pass
            .set_vertex_buffer(0, self.arrow_geometry.vertex_buffer.slice(..));
        render_pass.set_index_buffer(
            self.arrow_geometry.index_buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );

        render_pass.set_viewport(3712.0, 1926.0, 128.0, 128.0, 0.0, 1.0);

        render_pass.draw_indexed(0..self.arrow_geometry.num_indices, 0, 0..1);
    }
}

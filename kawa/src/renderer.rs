use std::{mem, unimplemented};

use anyhow::Result;

use glow::HasContext;
use glutin::dpi::PhysicalSize;
use imgui::DrawIdx;

#[derive(Debug, Copy, Clone)]
struct Uniforms {
    texture: glow::UniformLocation,
    projection_matrix: glow::UniformLocation,
}

#[derive(Debug, Copy, Clone)]
struct Attribs {
    position: u32,
    uv: u32,
    color: u32,
}

#[derive(Copy, Clone, Debug)]
pub struct Renderer {
    program: glow::Program,
    ebo: glow::Buffer,
    vbo: glow::Buffer,
    font_texture: glow::Texture,
    uniforms: Uniforms,
    attribs: Attribs,
}

impl Renderer {
    pub unsafe fn init(gl: &glow::Context, imgui: &mut imgui::Context) -> Result<Self> {
        let vertex_array = gl
            .create_vertex_array()
            .expect("could not create vertex array");
        gl.bind_vertex_array(Some(vertex_array));

        let program = gl.create_program().expect("could not create program");

        let shader_sources = [
            (
                glow::VERTEX_SHADER,
                include_str!("../assets/shaders/window.vert.glsl"),
            ),
            (
                glow::FRAGMENT_SHADER,
                include_str!("../assets/shaders/window.frag.glsl"),
            ),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());
        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            gl.shader_source(shader, shader_source);
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!(gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            println!("{:?}", gl.get_program_info_log(program));
            panic!(gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        gl.use_program(Some(program));

        let uniforms = Uniforms {
            texture: gl.get_uniform_location(program, "u_texture").unwrap(),
            projection_matrix: gl.get_uniform_location(program, "u_prj_matrix").unwrap(),
        };

        let attribs = Attribs {
            position: gl.get_attrib_location(program, "position").unwrap(),
            uv: gl.get_attrib_location(program, "uv").unwrap(),
            color: gl.get_attrib_location(program, "color").unwrap(),
        };

        let vbo = gl.create_buffer().unwrap();
        let ebo = gl.create_buffer().unwrap();

        let current_texture = gl.get_parameter_i32(glow::TEXTURE_BINDING_2D);

        let font_texture = gl.create_texture().unwrap();
        gl.bind_texture(glow::TEXTURE_2D, Some(font_texture));
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MIN_FILTER,
            glow::NEAREST as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MAG_FILTER,
            glow::NEAREST as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_S,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_T,
            glow::CLAMP_TO_EDGE as i32,
        );

        gl.pixel_store_i32(glow::UNPACK_ROW_LENGTH, 0);

        {
            let mut atlas = imgui.fonts();
            let texture = atlas.build_rgba32_texture();

            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as _,
                texture.width as _,
                texture.height as _,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(texture.data),
            );

            atlas.tex_id = (font_texture as usize).into();
        }

        gl.bind_texture(glow::TEXTURE_2D, Some(current_texture as u32));

        Ok(Self {
            program,
            vbo,
            ebo,
            font_texture,
            uniforms,
            attribs,
        })
    }

    pub unsafe fn render_ui(
        &self,
        gl: &glow::Context,
        draw_data: &imgui::DrawData,
        display_size: (f32, f32),
        fb_scale: (f32, f32),
    ) -> Result<()> {
        gl.enable(glow::BLEND);
        gl.blend_equation(glow::FUNC_ADD);
        gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        gl.disable(glow::CULL_FACE);
        gl.disable(glow::DEPTH_TEST);
        gl.enable(glow::SCISSOR_TEST);
        gl.polygon_mode(glow::FRONT_AND_BACK, glow::FILL);

        let (width, height) = display_size;
        let (scale_w, scale_h) = fb_scale;

        let fb_width = width * scale_w;
        let fb_height = height * scale_h;

        gl.viewport(0, 0, fb_width as _, fb_height as _);

        let matrix = [
            2.0 / width as f32,
            0.0,
            0.0,
            0.0,
            0.0,
            2.0 / -(height as f32),
            0.0,
            0.0,
            0.0,
            0.0,
            -1.0,
            0.0,
            -1.0,
            1.0,
            0.0,
            1.0,
        ];

        gl.use_program(Some(self.program));
        gl.uniform_1_i32(Some(&self.uniforms.texture), 0);
        gl.uniform_matrix_4_f32_slice(Some(&self.uniforms.projection_matrix), true, &matrix);

        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vao));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
        gl.enable_vertex_attrib_array(self.attribs.position);
        gl.enable_vertex_attrib_array(self.attribs.uv);
        gl.enable_vertex_attrib_array(self.attribs.color);
        gl.vertex_attrib_pointer_f32(
            self.attribs.position,
            2,
            glow::FLOAT,
            false,
            std::mem::size_of::<imgui::DrawVert>() as _,
            field_offset::<imgui::DrawVert, _, _>(|v| &v.pos) as _,
        );
        gl.vertex_attrib_pointer_f32(
            self.attribs.uv,
            2,
            glow::FLOAT,
            false,
            std::mem::size_of::<imgui::DrawVert>() as _,
            field_offset::<imgui::DrawVert, _, _>(|v| &v.uv) as _,
        );
        gl.vertex_attrib_pointer_f32(
            self.attribs.color,
            4,
            glow::UNSIGNED_BYTE,
            true,
            std::mem::size_of::<imgui::DrawVert>() as _,
            field_offset::<imgui::DrawVert, _, _>(|v| &v.col) as _,
        );

        for draw_list in draw_data.draw_lists() {
            let vtx_buffer = draw_list.vtx_buffer();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                std::slice::from_raw_parts(
                    vtx_buffer.as_ptr() as *const u8,
                    vtx_buffer.len() * std::mem::size_of::<imgui::DrawVert>(),
                ),
                glow::STREAM_DRAW,
            );

            let idx_buffer = draw_list.idx_buffer();
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                std::slice::from_raw_parts(
                    idx_buffer.as_ptr() as *const u8,
                    idx_buffer.len() * std::mem::size_of::<u16>(),
                ),
                glow::STREAM_DRAW,
            );

            for command in draw_list.commands() {
                match command {
                    imgui::DrawCmd::Elements {
                        count,
                        cmd_params:
                            imgui::DrawCmdParams {
                                clip_rect: [x, y, z, w],
                                texture_id,
                                idx_offset,
                                ..
                            },
                    } => {
                        gl.bind_texture(glow::TEXTURE_2D, Some(texture_id.id() as _));

                        gl.scissor(
                            (x * scale_w) as i32,
                            (fb_height - w * scale_h) as i32,
                            ((z - x) * scale_w) as i32,
                            ((w - y) * scale_h) as i32,
                        );

                        let idx_size = if std::mem::size_of::<DrawIdx>() == 2 {
                            glow::UNSIGNED_SHORT
                        } else {
                            glow::UNSIGNED_INT
                        };

                        gl.draw_elements(
                            glow::TRIANGLES,
                            count as _,
                            idx_size,
                            (idx_offset * mem::size_of::<DrawIdx>()) as _,
                        );
                    }
                    imgui::DrawCmd::ResetRenderState => {
                        unimplemented!();
                    }
                    imgui::DrawCmd::RawCallback { .. } => {
                        unimplemented!();
                    }
                }
            }
        }

        gl.delete_vertex_array(vao);

        Ok(())
    }

    pub unsafe fn resize(self, gl: &glow::Context, size: PhysicalSize<u32>) -> Result<()> {
        gl.viewport(0, 0, size.width as i32, size.height as i32);
        gl.scissor(0, 0, size.width as i32, size.height as i32);

        Ok(())
    }
}

fn field_offset<T, U, F: for<'a> FnOnce(&'a T) -> &'a U>(f: F) -> usize {
    unsafe {
        let instance = std::mem::zeroed::<T>();

        let offset = {
            let field: &U = f(&instance);
            field as *const U as usize - &instance as *const T as usize
        };

        std::mem::forget(instance);

        offset
    }
}

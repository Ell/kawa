use anyhow::Result;
use glow::{HasContext, COLOR_BUFFER_BIT, TRIANGLES};
use glutin::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder, PossiblyCurrent, WindowedContext,
};

use crate::renderer::{self, Renderer};

pub struct Window {
    renderer: Renderer,
    gl_context: glow::Context,
    window_context: WindowedContext<PossiblyCurrent>,
    event_loop: EventLoop<()>,
    imgui: imgui::Context,
}

impl Window {
    pub unsafe fn create(title: &str) -> Result<Self> {
        let window_size = LogicalSize::new(1280.0, 720.0);

        let event_loop = EventLoop::new();

        let window_builder = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(window_size);

        let window_context = ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap();

        let window_context = window_context.make_current().unwrap();

        let gl_context =
            glow::Context::from_loader_function(|s| window_context.get_proc_address(s) as *const _);

        let mut imgui = imgui::Context::create();
        let renderer = Renderer::init(&gl_context, &mut imgui).unwrap();

        Ok(Window {
            renderer,
            gl_context,
            window_context,
            event_loop,
            imgui,
        })
    }

    pub fn run_event_loop(self) {
        let gl_context = self.gl_context;
        let window_context = self.window_context;
        let event_loop = self.event_loop;
        let renderer = self.renderer;
        let mut imgui = self.imgui;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::LoopDestroyed => {
                    return;
                }
                Event::MainEventsCleared => {
                    window_context.window().request_redraw();
                }
                Event::RedrawRequested(_) => unsafe {
                    gl_context.clear(COLOR_BUFFER_BIT);
                    gl_context.draw_arrays(TRIANGLES, 0, 6);

                    {
                        let frame = imgui.frame();
                        let display_size = frame.io().display_size;
                        let fb_scale = frame.io().display_framebuffer_scale;

                        let draw_data = frame.render();
                        let [width, height] = display_size;
                        let [fb_width, fb_height] = fb_scale;

                        renderer
                            .render_ui(
                                &gl_context,
                                draw_data,
                                (width, height),
                                (fb_width, fb_height),
                            )
                            .unwrap();
                    }

                    window_context.swap_buffers().unwrap();
                },
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(physical_size) => unsafe {
                        window_context.resize(*physical_size);
                        renderer.resize(&gl_context, *physical_size).unwrap();
                    },
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }
}

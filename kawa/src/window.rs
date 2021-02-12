use anyhow::Result;
use glow::HasContext;
use glutin::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder, PossiblyCurrent, WindowedContext,
};
use imgui::{FontConfig, FontSource};
use imgui_winit_support::WinitPlatform;

use crate::renderer::Renderer;

pub struct Window {
    renderer: Renderer,
    gl_context: glow::Context,
    window_context: WindowedContext<PossiblyCurrent>,
    event_loop: EventLoop<()>,
    platform: WinitPlatform,
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
        imgui.set_ini_filename(None);

        let mut platform = WinitPlatform::init(&mut imgui);
        {
            let window = window_context.window();
            platform.attach_window(
                imgui.io_mut(),
                window,
                imgui_winit_support::HiDpiMode::Rounded,
            );
        }

        let hidpi_factor = platform.hidpi_factor();
        let font_size = (16.0 * hidpi_factor) as f32;
        imgui.fonts().add_font(&[FontSource::TtfData {
            data: include_bytes!("../assets/fonts/Roboto-Regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                ..FontConfig::default()
            }),
        }]);

        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        let renderer = Renderer::init(&gl_context, &mut imgui).unwrap();

        Ok(Window {
            renderer,
            gl_context,
            window_context,
            event_loop,
            imgui,
            platform,
        })
    }

    pub fn run_event_loop(self) {
        let gl_context = self.gl_context;
        let window_context = self.window_context;
        let event_loop = self.event_loop;
        let renderer = self.renderer;
        let mut platform = self.platform;
        let mut imgui = self.imgui;

        let mut last_frame = std::time::Instant::now();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(_) => {
                    let now = std::time::Instant::now();
                    imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;
                }
                Event::LoopDestroyed => {
                    return;
                }
                Event::MainEventsCleared => {
                    let window = window_context.window();

                    platform
                        .prepare_frame(imgui.io_mut(), window)
                        .expect("failed to prepare frame");

                    window.request_redraw();
                }
                Event::RedrawRequested(_) => unsafe {
                    let window = window_context.window();

                    gl_context.clear(glow::COLOR_BUFFER_BIT);
                    gl_context.clear_color(0.2, 0.2, 0.2, 1.0);

                    {
                        let frame = imgui.frame();

                        let [width, height] = frame.io().display_size;
                        let [fb_width, fb_height] = frame.io().display_framebuffer_scale;

                        let mut run = true;
                        frame.show_demo_window(&mut run);

                        if !run {
                            *control_flow = ControlFlow::Exit;
                        }


                        platform.prepare_render(&frame, window);
                        let draw_data = frame.render();

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
                event => {
                    let window = window_context.window();

                    platform.handle_event(imgui.io_mut(), window, &event);
                }
            }
        });
    }
}

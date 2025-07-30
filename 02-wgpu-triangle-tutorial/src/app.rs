use std::sync::Arc;

use winit::{application::ApplicationHandler, window::Window};

use crate::gpu::{Gpu, Renderer};

struct WindowSurface {
    window: Arc<winit::window::Window>,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
}

impl WindowSurface {
    fn new(window: winit::window::Window, gpu: &Gpu) -> Self {
        let window = Arc::new(window);

        let surface = gpu.instance().create_surface(Arc::clone(&window)).unwrap();

        let surface_format = surface
            .get_capabilities(gpu.adapter())
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .cloned()
            .unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        surface.configure(gpu.device(), &surface_config);

        WindowSurface {
            window,
            surface,
            surface_config,
        }
    }

    fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(device, &self.surface_config);
        }
    }

    fn surface_texture(&self) -> wgpu::SurfaceTexture {
        self.surface.get_current_texture().unwrap()
    }

    fn surface_format(&self) -> wgpu::TextureFormat {
        self.surface_config.format
    }
}

pub struct App {
    gpu: Gpu,
    renderer: Renderer,
    window_surface: Option<WindowSurface>,
}

impl App {
    pub fn new() -> Self {
        let gpu = pollster::block_on(Gpu::new());

        let renderer = Renderer::new(gpu.device());

        App {
            gpu,
            renderer,
            window_surface: None,
        }
    }

    pub fn render(&mut self) {
        if let Some(window_surface) = &self.window_surface {
            let surface_texture = window_surface.surface_texture();
            let surface_view = surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let surface_format = window_surface.surface_format();

            self.renderer.render(
                self.gpu.device(),
                self.gpu.queue(),
                surface_view,
                surface_format,
            );

            surface_texture.present();
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();

        self.window_surface = Some(WindowSurface::new(window, &self.gpu));

        self.render();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::Resized(size) => {
                if let Some(window_surface) = &mut self.window_surface {
                    window_surface.resize(self.gpu.device(), size.width, size.height);
                }
                self.render();
            }
            _ => {}
        }
    }
}

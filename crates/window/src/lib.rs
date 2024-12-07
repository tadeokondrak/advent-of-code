pub extern crate tiny_skia;
pub extern crate winit;
use softbuffer::{Context as SoftbufferContext, Surface as SoftbufferSurface};
use std::cell::{OnceCell, RefCell};
use std::num::NonZeroU32;
use std::rc::Rc;
use tiny_skia::{Color, Paint, PixmapMut, Rect, Shader, Transform};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::raw_window_handle::{DisplayHandle, HasDisplayHandle};
use winit::window::{Theme, Window as WinitWindow, WindowId as WinitWindowId};

#[derive(Default)]
struct App {
    initial_width: f32,
    initial_height: f32,
    scale: f32,
    drawcb: Option<Box<dyn Fn() -> Vec<(Rect, Color)>>>,
    keycb: Option<Box<dyn Fn(winit::keyboard::Key)>>,
    softbuffer_context: OnceCell<SoftbufferContext<DisplayHandle<'static>>>,
    winit_window: OnceCell<WinitWindowState>,
}

struct WinitWindowState {
    window: Rc<WinitWindow>,
    surface: RefCell<SoftbufferSurface<DisplayHandle<'static>, Rc<WinitWindow>>>,
}

impl App {
    fn new() -> App {
        App::default()
    }

    fn handle_redraw_requested(&self) {
        let window = self.winit_window.get().unwrap();
        let PhysicalSize { width, height } = window.window.inner_size();
        let (Some(nzwidth), Some(nzheight)) = (NonZeroU32::new(width), NonZeroU32::new(height))
        else {
            return;
        };
        window
            .surface
            .borrow_mut()
            .resize(nzwidth, nzheight)
            .unwrap();
        let mut surface = window.surface.borrow_mut();
        let mut buffer = surface.buffer_mut().unwrap();
        let data = bytemuck::cast_slice_mut(&mut buffer);
        let mut pixmap = PixmapMut::from_bytes(data, width, height).unwrap();
        pixmap.fill(Color::BLACK);
        let cmds = (self.drawcb.as_ref().unwrap())();
        let scale_transform =
            Transform::from_scale(self.scale as f32 * 2.0, self.scale as f32 * 2.0);
        let mut paint = Paint::default();
        for (rect, color) in cmds {
            paint.shader = Shader::SolidColor(color);
            pixmap.fill_rect(rect, &paint, scale_transform, None);
        }
        buffer.present().unwrap();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.winit_window.get().is_some() {
            return;
        }
        // unsound but I don't care
        let display =
            unsafe { DisplayHandle::borrow_raw(event_loop.display_handle().unwrap().as_raw()) };
        self.softbuffer_context
            .set(SoftbufferContext::new(display).unwrap())
            .ok()
            .unwrap();
        let window_attributes = WinitWindow::default_attributes()
            .with_title("advent-of-code")
            .with_theme(Some(Theme::Dark))
            .with_inner_size(LogicalSize {
                width: self.initial_width,
                height: self.initial_height,
            });
        let winit_window = Rc::new(event_loop.create_window(window_attributes).unwrap());
        let context = self.softbuffer_context.get().unwrap();
        let mut surface = SoftbufferSurface::new(context, winit_window.clone()).unwrap();
        let PhysicalSize { width, height } = winit_window.inner_size();
        if let (Some(w), Some(h)) = (NonZeroU32::new(width), NonZeroU32::new(height)) {
            surface.resize(w, h).unwrap();
        }
        self.winit_window
            .set(WinitWindowState {
                window: winit_window.clone(),
                surface: RefCell::new(surface),
            })
            .ok()
            .unwrap();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WinitWindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                if let (Some(nzwidth), Some(nzheight)) =
                    (NonZeroU32::new(width), NonZeroU32::new(height))
                {
                    self.winit_window
                        .get_mut()
                        .unwrap()
                        .surface
                        .borrow_mut()
                        .resize(nzwidth, nzheight)
                        .unwrap();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state.is_pressed() {
                    (self.keycb.as_ref().unwrap())(event.logical_key);
                    self.winit_window.get().unwrap().window.request_redraw();
                }
            }
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::MouseInput { .. } => {}
            WindowEvent::RedrawRequested => self.handle_redraw_requested(),
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {}
}

pub enum Event<'a> {
    Draw(&'a mut Vec<(Rect, Color)>),
    Key(winit::keyboard::Key),
}

pub struct Context<'a> {
    pub pixmap: &'a mut PixmapMut<'a>,
}

pub fn run(
    width: f32,
    height: f32,
    scale: f32,
    drawcb: impl Fn() -> Vec<(Rect, Color)> + 'static,
    keycb: impl Fn(winit::keyboard::Key) + 'static,
) {
    let mut app = App::new();
    app.initial_width = width * scale;
    app.initial_height = height * scale;
    app.scale = scale;
    app.drawcb = Some(Box::new(drawcb));
    app.keycb = Some(Box::new(keycb));
    EventLoop::new().unwrap().run_app(&mut app).unwrap();
}

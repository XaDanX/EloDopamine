
use glium::glutin;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::{Fullscreen, WindowBuilder};
use glium::{Display, Surface};
use imgui::{Context, FontConfig, FontSource, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::path::Path;
use std::time::Instant;
use glium::glutin::platform::windows::{EventLoopExtWindows, WindowExtWindows};
use winapi::shared::windef::HWND;
use winapi::um::dwmapi::DwmExtendFrameIntoClientArea;
use winapi::um::winuser::{GetWindowRect, GWL_EXSTYLE, MoveWindow, SetActiveWindow, SetFocus, SetWindowLongA, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOPMOST, WS_EX_TRANSPARENT};
use crate::memory::get_hwnd;
use winapi::um::uxtheme::MARGINS;
use user32::{GetAsyncKeyState, GetWindowLongA};
use winapi::um::winnt::LONG;

pub struct System {
    pub event_loop: EventLoop<()>,
    pub display: glium::Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub font_size: f32,
    pub hwnd: HWND
}

pub static mut SHOW_MENU: bool = false;

pub fn init(title: &str, width: f32, height: f32) -> System {
    let title = match Path::new(&title).file_name() {
        Some(file_name) => file_name.to_str().unwrap(),
        None => title,
    };

    let builder = WindowBuilder::new()
        .with_title(title.to_owned())
        .with_inner_size(glutin::dpi::LogicalSize::new(width as f64, height as f64))
        .with_transparent(true)
        .with_decorations(false)
        .with_always_on_top(true);


    let event_loop = EventLoop::new_any_thread();
    let context = glutin::ContextBuilder::new().with_vsync(true);





    let display =
        Display::new(builder, context, &event_loop).expect("Failed to initialize display");

    let monitor_handle = display.gl_window().window().primary_monitor().unwrap();
    let fs = Fullscreen::Borderless(Some(monitor_handle));
    display.gl_window().window().set_fullscreen(Some(fs));

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let hwnd: HWND;
    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();
        hwnd = window.hwnd() as HWND;


        let dpi_mode = if let Ok(factor) = std::env::var("IMGUI_EXAMPLE_FORCE_DPI_FACTOR") {
            // Allow forcing of HiDPI factor for debugging purposes
            match factor.parse::<f64>() {
                Ok(f) => HiDpiMode::Locked(f),
                Err(e) => panic!("Invalid scaling factor: {}", e),
            }
        } else {
            HiDpiMode::Default
        };

        platform.attach_window(imgui.io_mut(), window, dpi_mode);
    }

    unsafe {
        SetWindowLongA(hwnd, GWL_EXSTYLE, (WS_EX_TRANSPARENT | WS_EX_LAYERED | WS_EX_NOACTIVATE) as _);

        DwmExtendFrameIntoClientArea(hwnd, &MARGINS {
            cxLeftWidth: -1,
            cxRightWidth: -1,
            cyBottomHeight: -1,
            cyTopHeight: -1,
        });

        SetActiveWindow(get_hwnd("League of Legends (TM) Client"));

        let mut rect = std::mem::zeroed();
        GetWindowRect(get_hwnd("League of Legends (TM) Client"), &mut rect);
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        MoveWindow(hwnd, rect.left + 1, rect.top + 1, width - 1, height - 1, 1);


    }

    let font_size = 16.0;

    imgui.fonts().add_font(&[
        FontSource::TtfData {
            data: include_bytes!("../resources/quick.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                rasterizer_multiply: 1.5,
                oversample_h: 4,
                oversample_v: 4,
                ..FontConfig::default()
            }),
        }]);


    let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");




    System {
        event_loop,
        display,
        imgui,
        platform,
        renderer,
        font_size,
        hwnd
    }
}

impl System {
    pub fn main_loop<F: FnMut(&mut bool, &mut Ui) + 'static>(self, mut run_ui: F) {
        let System {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            ..
        } = self;
        let mut last_frame = Instant::now();

        //let mut style = imgui.style_mut();
        //style.colors["TitleBg"] = [1.0, 0.0, 0.0, 1.0];


        event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(_) => unsafe {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
                SetActiveWindow(get_hwnd("League of Legends (TM) Client"));
                SetFocus(get_hwnd("League of Legends (TM) Client"));

            }
            Event::MainEventsCleared => {
                let gl_window = display.gl_window();
                platform
                    .prepare_frame(imgui.io_mut(), gl_window.window())
                    .expect("Failed to prepare frame");
                gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                let ui = imgui.frame();

                let mut run = true;
                run_ui(&mut run, ui);
                if !run {
                    *control_flow = ControlFlow::Exit;
                }

                let gl_window = display.gl_window();
                let mut target = display.draw();//nibba
                target.clear_color_srgb(0.0, 0.0, 0.0, 0.0);

                unsafe {
                    let key_state = GetAsyncKeyState(0xA1) & 0x0001;

                    if key_state > 0 {
                        let win_log = GetWindowLongA(self.hwnd as _, GWL_EXSTYLE);

                        if !SHOW_MENU {
                            if win_log != (WS_EX_LAYERED | WS_EX_TOPMOST) as LONG {
                                SetWindowLongA(self.hwnd, GWL_EXSTYLE, (WS_EX_LAYERED | WS_EX_LAYERED) as LONG);
                            }
                        }
                        if SHOW_MENU {
                            if win_log != (WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_TRANSPARENT) as LONG {
                                SetWindowLongA(self.hwnd, GWL_EXSTYLE, (WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_TRANSPARENT) as LONG);
                            }
                        }
                        SHOW_MENU= !SHOW_MENU;

                    }
                }

                platform.prepare_render(ui, gl_window.window());
                let draw_data = imgui.render();
                renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event => {
                let gl_window = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
            }
        })
    }
}

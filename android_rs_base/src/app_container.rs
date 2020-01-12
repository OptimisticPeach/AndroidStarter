use crate::app_implementor::*;
use crate::app_config::*;
use crate::InputEvent;
use piston::window::{WindowSettings, OpenGLWindow};
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::thread::JoinHandle;
use android_glue;
use crate::storage::{ShaderStorage, ShaderContext};

/// A utility struct for running an android application, to not have to worry about the minor
/// android-specific details when running and rendering an app with piston
pub struct AppContainer<T: AppImpl> {
    native_event_reciever: std::sync::mpsc::Receiver<android_glue::Event>,
    window: GlutinWindow,
    app: Option<T>,
    events: Events,
    window_size: (usize, usize),
    gl: GlGraphics,
    config: AppConfig,
    thread: Option<(JoinHandle<()>, std::sync::mpsc::Sender<InputEvent>)>,
    storage: ShaderStorage,

}

impl<T: AppImpl> AppContainer<T> {
    /// Creates an `AppContainer` with appropriate settings.
    /// Unsure what happens if you create two of them at the same time
    /// `app: T`: an instance of your struct which implements `AppImpl`
    /// `config: AppConfig`: a configuration setting with which to run your app like number of frames or reset options
    /// In more detail:
    /// 1. Creates a `GlutinWindow`
    /// 2. Loads Opengl pointers using the window's address
    /// 3. Prepares channels for use with `android_glue`
    /// 4. Creates an instance of `AppContainer` and fills in some other members
    pub fn init(config: AppConfig, data: T::InitializationData) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        android_glue::add_sender(sender);
        let mut window: GlutinWindow = WindowSettings::new(
                "rust app", (200.0, 200.0)
            )
            .fullscreen(true)
            .graphics_api(OpenGL::V3_2)
            .build()
            .unwrap();
        opengl_graphics::gl::load_with(|x| window.get_proc_address(x) as *const _);
        let mut gl = GlGraphics::new(OpenGL::V3_2);
        let events = Events::new(EventSettings::new());
        let mut shaders = ShaderStorage::new();
        let app = T::new(&mut gl, data, &mut shaders);
        Self {
            native_event_reciever: receiver,
            window,
            app: Some(app),
            events,
            window_size: (0, 0),
            gl,
            config,
            thread: None,
            storage: shaders,
        }
    }

    /// Prepares for draw, and then calls `self.app.draw` with the parameters it prepared
    fn draw(&mut self, rargs: RenderArgs) {
        let app_ref = self.app.as_mut().unwrap();
        let ws_ref = &mut self.window_size;
        let sh_ref = &mut self.storage;
        self.config.passed_frames += 1;
        let cfg_ref = &mut self.config;
        if *ws_ref != (rargs.draw_size[0] as usize, rargs.draw_size[1] as usize) {
            let size_new = (rargs.draw_size[0] as usize, rargs.draw_size[1] as usize);
            app_ref.on_size_change(&size_new, ws_ref, sh_ref);
            *ws_ref = size_new;
        }

        self.gl.draw(rargs.viewport(), |c, gl| {
            app_ref.draw_2d(c, gl, rargs.clone(), cfg_ref);
            app_ref.draw_shaded(ShaderContext::new(sh_ref, gl, c, rargs));
        });
    }

    /// Suspends thread until we get a GainedFocus
    /// A bit of a hack, but not using this leads to:
    /// calling `self.events.next()` which at some point tries to swap buffers crashing egl -- it's ugly
    fn wait_until_gain_focus(&mut self) {
        use android_glue::Event;
        loop{
            let recieved = self.native_event_reciever.recv();
            match recieved {
                Ok(x) => match x {
                    Event::GainedFocus => { break; },
                    _ => {}
                },
                Err(_) => {
                    let app = self.app.take().unwrap();
                    app.on_die();
                    break;
                }
            }
        }
    }

    /// Tries to recieve android events, and manages focus changes
    fn poll_android_events(&mut self) {
        use android_glue::Event;
        let mut flag = false;
        for event in self.native_event_reciever.try_iter(){
            match event {
                Event::LostFocus => {
                    flag = true;
                    break;
                },
                Event::EventMotion(_) => {/*These are already passed in by piston*/},
                misc => {
                    self.app.as_mut().map(move |app| app.handle_android_event(misc));
                }
            }
        }
        if flag {
            self.app.as_mut().map(|app| app.signal_pause());
            self.wait_until_gain_focus();
            self.app.as_mut().map(|app| app.refresh());
        }
    }

    fn poll_events(&mut self) -> bool {
        while let Some(e) = self.events.next(&mut self.window) {
            match e {
                Event::Loop(loopargs) => match loopargs {
                    Loop::Render(r_args) => {
                        self.draw(r_args);
                    },
                    Loop::Update(u_args) => {
                        self.poll_android_events();
                        let cfg_ref = &mut self.config;
                        self.app.as_mut().map(|app| app.update(u_args, cfg_ref));
                    },
                    Loop::AfterRender(a_args) => {
                        self.app.as_mut().map(|app| app.after_draw(a_args));
                        return true;
                    },
                    _ => {}
                },
                Event::Custom(id, event, time) => {
                    if let Some((_, send)) = &mut self.thread {
                        send.send(InputEvent::Custom(id, event)).expect("Could not send event");
                    } else {
                        self.app.as_mut().map(|app| app.handle_custom_event(id, event, time));
                    }
                },
                Event::Input(input, time) => {
                    if let Some((_, send)) = &mut self.thread {
                        send.send(InputEvent::Piston(input)).expect("Could not send event");
                    } else {
                        self.app.as_mut().map(|app| app.input(input, time));
                    }
                }
            }
        }
        false
    }

    pub fn spawn_user_thread(&mut self, mut f: impl FnMut(InputEvent) + Send + 'static) {
        let (sender, receiver) = std::sync::mpsc::channel();
        self.thread = Some((
            std::thread::spawn(move || {
                loop {
                    match receiver.recv() {
                        Ok(x) => {
                            f(x);
                        },
                        Err(e) => {
                            panic!("User thread panicked! {:?}", e);
                        }
                    }
                }
            }),
            sender,
        ));
    }

    /// Runs the application as per the configuration provided when `init` was called
    pub fn run(&mut self) {
        if self.config.reset_on_start {
            self.app.as_mut().map(|app| app.reset_on_start());
        }

        if let Some(frames) = self.config.num_frames {
            for _ in 0..frames {
                if self.app.is_none() {
                    break;
                }
                while !self.poll_events() {}
            }
        } else {
            loop {
                if self.app.is_none() {
                    break;
                }
                self.poll_events();
            }
        }
    }
}

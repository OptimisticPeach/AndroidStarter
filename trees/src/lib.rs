use android_base::{AppImpl, UpdateArgs, enable_backtrace, AppContainer, AppConfig, ShaderStorage, ShaderContext};
use graphics::{Context, clear};
use opengl_graphics::{GlGraphics, GLSL};
use piston::input::RenderArgs;

pub struct App {
    time: f64,
}

impl AppImpl for App {
    type InitializationData = ();
    fn new(gl: &mut GlGraphics, _data: Self::InitializationData, _shaders: &mut ShaderStorage) -> Self {
        Self {
            time: 0.0,
        }
    }

    fn on_size_change(&mut self, new: &(usize, usize), _old: &(usize, usize), shaders: &mut ShaderStorage) {
        println!("Size changed to {:?} as width, height", new);
    }

    fn update(&mut self, args: UpdateArgs, _cfg: &mut AppConfig) {
        self.time += args.dt;
    }

    fn draw_shaded(&mut self, mut context: ShaderContext) {
        // For shader calls
    }

    fn draw_2d(&mut self, _c: Context, gl: &mut GlGraphics, args: RenderArgs, _cfg: &mut AppConfig) {
        self.time += args.ext_dt;
        clear([163.0 / 255.0, 250.0 / 255.0, 255.0 / 255.0, 1.], gl);
        
    }

    fn on_die(self) {
        println!("Dieing!");
    }
    fn cancel_poll(&self) -> bool {
        false
    }
}

pub fn main() {
    enable_backtrace();
    let mut container = AppContainer::<App>::init(AppConfig::new(), ());
    container.run();
}

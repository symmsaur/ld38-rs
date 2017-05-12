extern crate sdl2;

use game;


pub struct Engine {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    //window: sdl2::video::Window,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    // refactor this out!
    event_pump: sdl2::EventPump,
}

impl Engine {
    // refactor this out!
    pub fn quit(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => return true,
                _ => (),
            }
        }
        false
    }

    pub fn render(&mut self, game: &game::Game) {
        self.canvas.clear();
        self.canvas.present();
    }
}

pub fn create() -> Result<Engine, String> {
    let sdl_context = try!(sdl2::init());
    let video_subsystem = try!(sdl_context.video());
    let window = match video_subsystem.window("Pond of Doom", 800, 600)
        .position_centered().opengl().build() {
            Ok(w) => w,
            Err(_) => return Err("Could not create window".to_string()),
        };
    let canvas = match window.into_canvas().build() {
        Ok(c) => c,
        Err(_) => return Err("Could not create canvas".to_string())
    };
    // refactor this out
    let event_pump = try!(sdl_context.event_pump());

    Ok(Engine {
        sdl_context: sdl_context,
        video_subsystem: video_subsystem,
        //window: window,
        canvas: canvas,
        // refactor this out
        event_pump: event_pump,
    })
}

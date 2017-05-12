mod engine;
mod game;

fn main() {
    let game = game::Game {};
    //let director = Director.create(&game);
    let mut engine = match engine::create() {
        Ok(e) => e,
        Err(s) => panic!(s)
    };
    //let input = Input.create();
    loop {
        if engine.quit() {
            break;
        }
        // Clock?
        //input.handle();
        //director.update_enemies();
        //game.tick();
        engine.render(&game);
    }
}

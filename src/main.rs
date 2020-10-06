use ggez::event;
use rand::Rng;
use rand::rngs::ThreadRng;
use kawakudari_ggez::Direction;
use kawakudari_ggez::Std15;

struct MainState {
    std15: Std15,
    rng: ThreadRng,
    x: i32,
    running : bool,
}

impl MainState {
    fn new() -> MainState {
        MainState {
            std15 : Std15::new(512, 384, 32, 24),
            rng : rand::thread_rng(),
            x : 15,
            running : true,
        }
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("kawakudari_ggez", "mitsuji")
        .window_setup(ggez::conf::WindowSetup::default()
                      .title("kawakudari"))
        .window_mode(ggez::conf::WindowMode::default()
                     .dimensions(512.0,384.0));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new();
    event::run(ctx, event_loop, state)
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        while ggez::timer::check_update_time(ctx, 60) {
            if self.running {
                let tick = ggez::timer::ticks(ctx) as usize;
                if tick % 5 == 0 {
                    self.std15.locate(self.x,5);
                    self.std15.putc('0');
                    self.std15.locate(self.rng.gen_range(0, 32),23);
                    self.std15.putc('*');
                    self.std15.scroll(Direction::Up);
                    if self.std15.scr(self.x,5) != '\0' {
                        self.std15.locate(0,23);
                        self.std15.putstr("Game Over...");
                        self.std15.putnum(tick as i32);
                        self.running = false;
                    }
                }
            }
        }
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut ggez::Context, keycode: event::KeyCode, _keymod: event::KeyMods, _repeat: bool) {
        if keycode == event::KeyCode::Left {
            self.x -= 1
        }
        if keycode == event::KeyCode::Right {
            self.x += 1
        }
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.std15.draw_screen(ctx)
    }

}



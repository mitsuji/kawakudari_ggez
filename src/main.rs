use ggez::event;
use rand::Rng;
use kawakudari_ggez::Std15;

struct MainState {
    std15: Std15,
    x: i32,
    running : bool,
}

impl MainState {
    fn new() -> MainState {
        MainState {
            std15 : Std15::new(512, 384, 32, 24),
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
                if ggez::timer::ticks(ctx) % 5 == 0 {
                    let mut rng = rand::thread_rng();
                    self.std15.locate(self.x,5);
                    self.std15.putc('0');
                    self.std15.locate(rng.gen_range(0, 32),23);
                    self.std15.putc('*');
                    self.std15.scroll();
                    if self.std15.scr(self.x,5) != '\0' {
                        self.running = false;
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.std15.papplet_draw(ctx)?;
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

}



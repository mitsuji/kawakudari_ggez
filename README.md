# kawakudari-ggez

This project implements part of the [std15.h](https://github.com/IchigoJam/c4ij/blob/master/src/std15.h) API (from [c4ij](https://github.com/IchigoJam/c4ij)) with [ggez](https://ggez.rs), and [Kawakudari Game](https://ichigojam.github.io/print/en/KAWAKUDARI.html) on top of it.

It will allow programming for [IchigoJam](https://ichigojam.net/index-en.html)-like targets that display [IchigoJam FONT](https://mitsuji.github.io/ichigojam-font.json/) on screen using a Rust programming language.
```
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
```

## Prerequisite

* This project using programming language Rust, so you need [rustup](https://rustup.rs) build tool properly installd to run example code.


## How to use

To just run example
```
$ cargo run
```

To build executeble and run example
```
$ cargo build
$ target/debug/kawakudari_ggez
```


## License
[![Creative Commons License](https://i.creativecommons.org/l/by/4.0/88x31.png)](http://creativecommons.org/licenses/by/4.0/)
[CC BY](https://creativecommons.org/licenses/by/4.0/) [mitsuji.org](https://mitsuji.org)

This work is licensed under a [Creative Commons Attribution 4.0 International License](http://creativecommons.org/licenses/by/4.0/).

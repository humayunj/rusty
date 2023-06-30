use ggez::{
    conf::WindowMode,
    glam::{vec2, Vec2},
    graphics::Color,
    winit::window,
    *,
};

struct State {
    pos_x: f32,
    pos_y: f32,
    circle: graphics::Mesh,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            100.0,
            2.0,
            Color::WHITE,
        )?;
        Ok(State {
            pos_x: 0.,
            pos_y: 0.,
            circle,
        })
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        self.pos_y = self.pos_y % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));
        canvas.draw(&self.circle, Vec2::new(self.pos_x, self.pos_y));
        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let c = conf::Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .default_conf(c)
        .build()
        .unwrap();
    let state = State::new(&mut ctx)?;
    ctx.gfx.window().set_resizable(true);
    event::run(ctx, event_loop, state);
}

use orbtk::prelude::*;

mod config;

fn main() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("Primary")
                .resizeable(true)
                .child(TextBlock::new().text("Hello World!").build(ctx))
                .build(ctx)
        })
        .run();
}

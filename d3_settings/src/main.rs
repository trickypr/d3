use std::{fs, path::Path};

use orbtk::prelude::*;

mod config;
mod view;

use config::{compile, Config, ConfigTree};

const CONFIG: &[&'static str] = &["/etc/i3/config", "~/.i3/config", "~/.config/i3/config"];

fn main() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("d3 Settings")
                .resizeable(true)
                .child(MainSystem::new().build(ctx))
                .build(ctx)
        })
        .run();
}

widget!(MainSystem { i3_config: Config });

const STARTUP_APPS_CONTAINER: &str = "startup_apps_container";

impl Template for MainSystem {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            TabWidget::new()
                .tab(
                    "Startup Applications",
                    view::LaunchApps::new().i3_config(id).build(ctx),
                )
                .build(ctx),
        )
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        DefaultRenderObject.into()
    }

    fn layout(&self) -> Box<dyn Layout> {
        GridLayout::new().into()
    }
}

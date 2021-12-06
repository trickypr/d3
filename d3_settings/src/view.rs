use orbtk::prelude::*;

use crate::config::Config;

widget!(LaunchApps { i3_config: Config });

impl Template for LaunchApps {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(ListView::new().build(ctx))
    }
}

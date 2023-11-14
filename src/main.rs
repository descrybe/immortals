pub mod camera;

use camera::CameraPlugin;

fn main() {
  App::new()
    .add_plugin(CameraPlugin)
    .run();
}

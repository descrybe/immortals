use crate::ApplicationState;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(spawnCamera);
  }
}

fn spawnCamera() {
  println!("spawn camera");
}

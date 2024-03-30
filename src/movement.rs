use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Veclocity {
    pub value: Vec3,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

fn update_position(mut query: Query<(&Veclocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut tranform) in query.iter_mut() {
        tranform.translation += velocity.value * time.delta_seconds();
    }
}

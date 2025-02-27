use bevy::prelude::*;
use bevy_mod_sysfail::prelude::*;
use bevy_mod_sysfail::Dedup;

use thiserror::Error;

#[derive(Component)]
struct Foo;

#[derive(Error, Debug)]
enum GizmoError {
    #[error("A Gizmo error")]
    Error,
}

impl Dedup for GizmoError {
    type ID = ();

    fn identify(&self) {}
}

fn main() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::log::LogPlugin::default()))
        .add_systems(
            Update,
            (drag_gizmo, exclusive_system, (delete_gizmo, place_gizmo)).chain(),
        );
    app.update();
    app.update();
}

#[sysfail(Log<GizmoError>)]
fn drag_gizmo(time: Res<Time>) {
    println!("drag time is: {}", time.elapsed_secs());
    let _ = Err(GizmoError::Error)?;
    println!("This will never print");
}

#[sysfail(Log<&'static str, Info>)]
fn place_gizmo() {
    let () = Result::<(), &'static str>::Ok(())?;
    println!("this line should actually show up");
    let _ = Err("Ah, some creative use of info logging I see")?;
}

#[exclusive_sysfail(LogSimply<anyhow::Error, Error>)]
fn exclusive_system(_: &mut World, mut has_printed: Local<bool>) {
    if *has_printed {
        return Ok(());
    }
    *has_printed = true;
    let _ = Err(anyhow::anyhow!("We simply logged this error"))?;
}

/// This also has some doc
#[sysfail(Ignore)]
fn delete_gizmo(time: Res<Time>, mut query: Query<&mut Transform>, foos: Query<Entity, With<Foo>>) {
    println!("delete time is: {}", time.elapsed_secs());
    for foo in &foos {
        let mut trans = query.get_mut(foo)?;
        trans.translation += Vec3::Y;
    }
    let _ = Err(())?;
    println!("This will never print");
}

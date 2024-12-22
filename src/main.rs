use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::linear_rgb(0.4, 0.4, 0.4)))
        .insert_resource(AmbientLight{brightness: 1000.0, ..default()})
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy game".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, load_gltfs)
        .run();
}



fn load_gltfs(
    mut commands: Commands,
    ass: Res<AssetServer>
){

    // Camera
    let mut camera_transform = Transform::from_translation(Vec3::new(0.0, 400.0, 400.0));
    camera_transform.look_at(Vec3::new(0.0, 50.0, 0.0), Vec3::Y);

    commands.spawn(
        (
            Camera3d::default(),
            camera_transform
        )
    );

    // Mesh only character
    let mesh0 = Mesh3d(ass.load(
        GltfAssetLabel::Primitive{mesh:0, primitive:0}.from_asset("mesh_only_viking.glb")
    ));
    let material0 = MeshMaterial3d::<StandardMaterial>(ass.load(
            GltfAssetLabel::Material{index: 0, is_scale_inverted: false}.from_asset("mesh_only_viking.glb")
    ));

    let _character0 = commands.spawn((
        mesh0,
        material0,
        Transform::from_translation(Vec3::splat(0.0))
                  .with_scale(Vec3::splat(1.0))
                  .with_rotation(Quat::from_euler(EulerRot::XYZ, 90.0_f32.to_radians(), 0.0, 0.0)),
    )).id();


    // Character with armature
    let mesh1 = Mesh3d(ass.load(
        GltfAssetLabel::Primitive{mesh:0, primitive:0}.from_asset("mesh_with_armature_viking.glb")
    ));
    let material1 = MeshMaterial3d::<StandardMaterial>(ass.load(
            GltfAssetLabel::Material{index: 0, is_scale_inverted: false}.from_asset("mesh_with_armature_viking.glb")
    ));

    let _character1 = commands.spawn((
        mesh1,
        material1,
        Transform::from_translation(Vec3::splat(0.0))
                  .with_scale(Vec3::splat(1.0))
                  .with_rotation(Quat::from_euler(EulerRot::XYZ, 90.0_f32.to_radians(), 0.0, 0.0)),
    )).id();


    // Character with armature but loaded as scene
    // let scene = SceneRoot(ass.load(GltfAssetLabel::Scene(0).from_asset("mesh_with_armature_viking.glb")));
    // let _scene_character = commands.spawn(
    //     (
    //         scene,
    //         Transform::from_translation(Vec3::splat(0.0)).with_scale(Vec3::splat(100.0))
    //     ));

}

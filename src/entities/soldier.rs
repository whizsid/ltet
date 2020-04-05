use amethyst::prelude::World;
use amethyst::prelude::Builder;
use amethyst::prelude::WorldExt;
use amethyst::ecs::Entity;
use amethyst::assets::Handle;
use amethyst::core::transform::Transform;
use amethyst_gltf::GltfSceneAsset;

pub struct Soldier {
    entity: Entity,
    trans_x: f32,
    trans_y: f32,
    trans_z: f32,
}

impl Soldier {
    pub fn new( world: &mut World, soldier: Handle<GltfSceneAsset>)->Soldier{
        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, 0.0, 0.0);

        let entity = world.create_entity()
            .with(soldier)
            .with(transform)
            .build();

        Soldier {
            entity,
            trans_x: 0.0,
            trans_y: 0.0,
            trans_z: 0.0,
        }
    }

    pub fn set_position(&mut self,world: &mut World, x: f32, y: f32, z: f32){
        self.trans_x = x;
        self.trans_y = y;
        self.trans_z = z;

        let mut storage = world.write_storage::<Transform>();
        let transform = storage.get_mut(self.entity).expect("Failed to get transform for camera");

        transform.set_translation_xyz(x, y, z);
    }


}
use amethyst::{
    prelude::{
        World,
        Builder,
        WorldExt
    },
    ecs::Entity,
    assets::{
        Handle
    },
    core::transform::Transform
};
use amethyst_gltf::GltfSceneAsset;
use super::behaviour::{CanTransform,RotateDirection};

pub struct Soldier {
    entity: Entity,
    position: (f32, f32, f32),
    rotation: (f32, f32)
}

impl Soldier {
    pub fn new( world: &mut World, soldier: Handle<GltfSceneAsset>)->Soldier{
        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, 0.0, 10.0);

        let entity = world.create_entity()
            .with(soldier)
            .with(transform)
            .build();

        Soldier {
            entity,
            position:(0.0,0.0,0.0),
            rotation: (0.0,0.0)
        }
    }
}

impl CanTransform for Soldier {
    fn set_position(&mut self, pos: (f32, f32, f32)){
        self.position = pos;
    }

    fn set_rotation(&mut self, dir: RotateDirection, rad: f32){
        match dir {
            RotateDirection::Horizontal=>{
                self.rotation = (rad,self.rotation.1);
            }
            RotateDirection::Vertical=>{
                self.rotation = (self.rotation.1,rad);
            }
        };
    }

    fn get_entity(&self)-> Entity{
        self.entity
    }
}
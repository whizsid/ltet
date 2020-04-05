use amethyst::core::transform::Transform;
use amethyst::prelude::World;
use amethyst::prelude::WorldExt;
use amethyst::ecs::Entity;

pub trait CanTransform {
    fn set_position(&mut self,position:(f32,f32,f32));

    fn get_entity(&self)-> Entity;

    fn set_rotation(&mut self, dir: RotateDirection, rad: f32);
}

#[derive(Copy, Clone)]
pub enum RotateDirection {
    Vertical,
    Horizontal
}

pub fn r#move(world: &mut World, movable: &mut impl CanTransform, position:(f32, f32, f32)){
    let mut storage = world.write_storage::<Transform>();
    let transform = storage.get_mut(movable.get_entity()).expect("Failed to get transform for the entity");

    movable.set_position(position);
    transform.set_translation_xyz(position.0,position.1,position.2);
}

pub fn rotate(world: &mut World, rotatable: &mut impl CanTransform, dir: RotateDirection, rad: f32 ){
    let mut storage = world.write_storage::<Transform>();
    let transform = storage.get_mut(rotatable.get_entity()).expect("Failed to get transform for the entity");

    rotatable.set_rotation(dir, rad);

    match dir {
        RotateDirection::Horizontal=>{
            transform.append_rotation_y_axis(rad);
        }
        RotateDirection::Vertical=>{
            transform.append_rotation_x_axis(rad);
        }
    };
}
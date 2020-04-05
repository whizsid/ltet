use amethyst::prelude::World;
use amethyst::prelude::WorldExt;
use amethyst::window::ScreenDimensions;
use amethyst::renderer::{Camera as ACamera};
use amethyst::core::transform::Transform;
use amethyst::prelude::Builder;
use amethyst::ecs::Entity;
use amethyst::controls::FlyControlTag;
use super::behaviour::{
    CanTransform,
    RotateDirection
};

pub struct Camera {
    entity: Entity,
    position: (f32, f32, f32),
    rotation: (f32, f32)
}

impl Camera {
    pub fn new (world: &mut World)->Camera{
        let (width,height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(),dim.height())
        };

        let camera = ACamera::standard_3d(width, height);

        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, 0.0, 0.0);

        let fly = FlyControlTag;

        let entity = world
            .create_entity()
            .with(camera)
            .with(transform)
            .with(fly)
            .build();

        Camera {
            position: (0.0,0.0,0.0),
            rotation: (0.0,0.0),
            entity
        }
    }
}

impl CanTransform for Camera {
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

    fn set_position(&mut self, pos:(f32, f32, f32)){
        self.position = pos;
    }

    fn get_entity(&self)-> Entity{
        self.entity
    }
}
use amethyst::prelude::World;
use amethyst::prelude::WorldExt;
use amethyst::window::ScreenDimensions;
use amethyst::renderer::{Camera as ACamera};
use amethyst::core::transform::Transform;
use amethyst::prelude::Builder;
use amethyst::ecs::Entity;
use amethyst::controls::FlyControlTag;

pub struct Camera {
    camera_entity: Entity,
    trans_x: f32,
    trans_y: f32,
    trans_z: f32,
    rotate_v: f32,
    rotate_h: f32
}

impl Camera {
    pub fn new (world: &mut World)->Camera{
        let (width,height) = {
                let dim = world.read_resource::<ScreenDimensions>();
                (dim.width(),dim.height())
            };

        let camera = ACamera::standard_3d(width, height);

        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, 0.0, -200.0);
        transform.prepend_rotation_y_axis(std::f32::consts::PI);

        let fly = FlyControlTag;

        let camera_entity = world
            .create_entity()
            .with(camera)
            .with(transform)
            .with(fly)
            .build();



        Camera {
            trans_x: 0.0,
            trans_y: 0.0,
            trans_z: -12.0,
            rotate_v: std::f32::consts::PI,
            rotate_h: 0.,
            camera_entity
        }
    }

    pub fn set_position(&mut self,world: &mut World, x: f32, y: f32, z: f32){
        self.trans_x = x;
        self.trans_y = y;
        self.trans_z = z;

        let mut storage = world.write_storage::<Transform>();
        let transform = storage.get_mut(self.camera_entity).expect("Failed to get transform for camera");

        transform.set_translation_xyz(x, y, z);
    }

    pub fn set_rotation_h(&mut self, world: &mut World, rad: f32){
        self.rotate_h = rad;

        let mut storage = world.write_storage::<Transform>();
        let transform = storage.get_mut(self.camera_entity).expect("Failed to get transform for camera");

        transform.append_rotation_y_axis(rad);
    }

    pub fn set_rotation_v(&mut self, world: &mut World, rad: f32){
        self.rotate_v = rad;

        let mut storage = world.write_storage::<Transform>();
        let transform = storage.get_mut(self.camera_entity).expect("Failed to get transform for camera");

        transform.append_rotation_x_axis(rad);
    }
}
use amethyst::renderer::rendy::mesh::Position;
use amethyst::ecs::{Component, DenseVecStorage};

struct Wound {
    pub create_sound : &'static str,
    pub after_create_sound: &'static str,
    pub after_create_sound_delay: &'static str,
    pub wounds: Vec<Position>
}

impl Wound {
    pub fn new(&mut self,position: Position){
        self.wounds.push(position);
    }
}

impl Component for Wound {
    type Storage = DenseVecStorage<Self>;
}
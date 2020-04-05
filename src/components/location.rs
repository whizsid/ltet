use amethyst::renderer::rendy::mesh::Position;
use amethyst::ecs::{Component, DenseVecStorage};

struct Location {
    pub current_position: Position
}

impl Location {
    pub fn move_to(&mut self, pos: Position){
        self.current_position = pos;
    }
}

impl Component for Location {
    type Storage = DenseVecStorage<Self>;
}
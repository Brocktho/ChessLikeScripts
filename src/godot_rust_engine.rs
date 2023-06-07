use godot::{
    engine::{node::InternalMode, packed_scene::GenEditState},
    prelude::*,
};

pub trait PackedSceneExt {
    fn instantiate_as<T>(&self) -> Gd<T>
    where
        T: GodotClass + Inherits<Node>,
    {
        self.try_instantiate_as().unwrap()
    }

    fn try_instantiate_as<T>(&self) -> Option<Gd<T>>
    where
        T: GodotClass + Inherits<Node>;
}

impl PackedSceneExt for PackedScene {
    fn try_instantiate_as<T>(&self) -> Option<Gd<T>>
    where
        T: GodotClass + Inherits<Node>,
    {
        self.instantiate(GenEditState::GEN_EDIT_STATE_DISABLED)
            .map(|gd| gd.cast::<T>())
    }
}

pub trait NodeExt2 {
    fn add_child2<T>(&mut self, child: Gd<T>)
    where
        T: GodotClass + Inherits<Node>;
}

impl NodeExt2 for Node {
    fn add_child2<T>(&mut self, child: Gd<T>)
    where
        T: GodotClass + Inherits<Node>,
    {
        self.add_child(child.upcast(), false, InternalMode::INTERNAL_MODE_DISABLED);
    }
}

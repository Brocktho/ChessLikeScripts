use godot::engine::global::Side;
use godot::engine::{ColorRect, ColorRectVirtual};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=ColorRect)]
pub struct Tile {
    interactable: bool,
    current_pos: Vector2,

    #[base]
    base: Base<ColorRect>,
}

#[godot_api]
impl ColorRectVirtual for Tile {
    fn init(base: Base<ColorRect>) -> Self {
        Tile {
            interactable: false,
            current_pos: Vector2::new(0., 0.),
            base,
        }
    }
}

#[godot_api]
impl Tile {
    #[func]
    pub fn create(&mut self, position: Vector2, color: Color) -> Gd<Tile> {
        self.base.set_position(position * 32.0, false);
        self.base.set_color(color.with_alpha(0.5));
        self.base.share().cast()
    }

    #[func]
    pub fn center_self(&mut self) -> Gd<Tile> {
        self.base.set_anchor(Side::SIDE_BOTTOM, 0.5, false, false);
        self.base.set_anchor(Side::SIDE_LEFT, 0.5, false, false);
        self.base.set_anchor(Side::SIDE_RIGHT, 0.5, false, false);
        self.base.set_anchor(Side::SIDE_TOP, 0.5, false, false);
        self.base.share().cast()
    }
}

/*
class_name Tile
extends ColorRect

signal pressed(current_pos : Vector2)

var interactable = true
var current_pos = Vector2(0,0)

func center_self():
    anchor_left = 0.5
    anchor_top = 0.5
    anchor_right = 0.5
    anchor_bottom = 0.5
    return self

func set_opacity(opacity: float):
    self.modulate.a = opacity

func set_interact(can_interact: bool):
    interactable = can_interact
    if not interactable:
        mouse_filter = Control.MOUSE_FILTER_IGNORE
    else:
        set_opacity(0.5)
        mouse_filter = Control.MOUSE_FILTER_PASS
        mouse_default_cursor_shape = Control.CURSOR_POINTING_HAND
    return self

func add_border():
    var ref = ReferenceRect.new()
    ref.mouse_filter = Control.MOUSE_FILTER_IGNORE
    ref.z_index = 2
    ref.border_color = Color("BLACK")
    ref.border_width = 1
    ref.editor_only = false
    ref.get_parent_area_size()
    add_child(ref)

func init(pos: Vector2, can_interact: bool, border: bool = true):
    current_pos = pos
    if border:
        add_border()
    position = pos * 32

    set_interact(can_interact)
    return self

func _gui_input(event):
    if interactable:
        if event is InputEventMouseButton:
            if event.is_pressed() && event.button_index == MOUSE_BUTTON_LEFT:
                pressed.emit(current_pos)
 */

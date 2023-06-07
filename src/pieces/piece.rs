use godot::engine::{Sprite2D, Sprite2DVirtual};
use godot::prelude::meta::VariantMetadata;
use godot::prelude::*;
use godot::sys::GodotFfi;
use godot::sys::GodotFuncMarshal;

#[derive(Debug, FromVariant)]
pub enum PieceTypes {
    Horse,
    Queen,
    Pawn,
    King,
    Bishop,
    Rook,
}

impl From<i64> for PieceTypes {
    fn from(v: i64) -> Self {
        match v {
            0 => PieceTypes::Horse,
            1 => PieceTypes::Queen,
            2 => PieceTypes::Pawn,
            3 => PieceTypes::King,
            4 => PieceTypes::Bishop,
            5 => PieceTypes::Rook,
            _ => panic!("Invalid piece type"),
        }
    }
}

impl VariantMetadata for PieceTypes {
    fn variant_type() -> VariantType {
        VariantType::Int
    }
}

impl GodotFuncMarshal for PieceTypes {
    type Via = i64;

    unsafe fn try_from_arg(
        ptr: godot::sys::GDExtensionTypePtr,
        call_type: godot::sys::PtrcallType,
    ) -> Result<Self, Self::Via> {
        let via = i64::from_arg_ptr(ptr, call_type);
        Ok(PieceTypes::from(via))
    }

    unsafe fn try_return(
        self,
        dst: godot::sys::GDExtensionTypePtr,
        call_type: godot::sys::PtrcallType,
    ) -> Result<(), Self> {
        (self as i64).move_return_ptr(dst, call_type);
        Ok(())
    }
}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Piece {
    //moves: Vec<Vector2>,
    position: Vector2,
    alignment: i8,
    piece_type: PieceTypes,

    #[base]
    base: Base<Sprite2D>,
}

#[godot_api]
impl Sprite2DVirtual for Piece {
    fn init(base: Base<Sprite2D>) -> Self {
        Piece {
            //      moves: vec![],
            position: Vector2::new(0., 0.),
            piece_type: PieceTypes::Pawn,
            alignment: 1,
            base,
        }
    }
}

#[godot_api]
impl Piece {
    #[func]
    fn move_me(&mut self, new_pos: Vector2) -> Gd<Piece> {
        self.position = new_pos;
        self.base.set_position(new_pos * 32.0);
        self.base.share().cast::<Piece>()
    }

    #[func]
    fn create(&mut self, pos: Vector2, align: i8, piece_type: PieceTypes) -> Gd<Piece> {
        self.position = pos;
        self.alignment = align;
        self.piece_type = piece_type;
        self.base.set_position(pos * 32.0);
        self.base.share().cast::<Piece>()
    }
}

/* class_name Piece
extends Sprite2D

signal pressed(node: Piece)

var available_moves: Array
var current_pos = Vector2(3, 3)
var board : Board
var alignment = 1;
var exhausted = false;
var type = "Piece"

const shader = preload("res://src/Pieces/Piece.gdshader")

var this_mat : ShaderMaterial

func check_collision(key : String):
    if key in self.board.grid:
        var found_piece = self.board.grid[key]
        if found_piece.alignment == alignment:
            return [true, true]
        return [true, false]
    return [false, false]

func rook_moves(pos: Vector2):
    for direction in Globals.directions:
        # [1,0] [0,1] [-1,0] [0,-1]
        var finished = false;
        var new_move = pos + direction
        while (new_move[1] >= 0 && new_move[1] <= Globals.l_edge && new_move[0] >= 0 && new_move[0] <= Globals.w_edge && !finished):
            var result = check_collision(str(new_move))
            finished = result[0]
            var own_piece = result[1]
            if own_piece: break
            available_moves.append(new_move)
            new_move += direction

func bishop_moves(pos: Vector2):
    for diag in Globals.diags:
        var finished = false;
        var new_move = pos + diag
        while (new_move[0] >= 0 && new_move[0] <= Globals.w_edge && new_move[1] >= 0 && new_move[1] <= Globals.l_edge && !finished):
            var result = check_collision(str(new_move))
            finished = result[0]
            var own_piece = result[1]
            if own_piece: break
            available_moves.append(new_move)
            new_move += diag

# To be used by non long-boy moves
func valid_move(move_vec: Vector2, invalid_on_collision: bool = false, valid_on_collision: bool = false):
    if move_vec[0] >= 0 && move_vec[0] <= Globals.w_edge && move_vec[1] >= 0 && move_vec[1] <= Globals.l_edge:
        var result = check_collision(str(move_vec))
        var hit_piece = result[0]
        var own_piece = result[1]
        if valid_on_collision:
            if hit_piece && !own_piece:
                available_moves.append(move_vec)
            return
        if invalid_on_collision:
            if hit_piece:
                return
        if not own_piece:
            available_moves.append(move_vec)


func horse_moves(pos: Vector2):
    for dir in Globals.directions:
        var next = pos + (dir * 2)
        if dir[0] != 0:
            var dy = Vector2(0, 1)
            valid_move(next + dy)
            valid_move(next + (dy * -1))
        if dir[1] != 0:
            var dx = Vector2(1, 0)
            valid_move(next + dx)
            valid_move(next + (dx * -1))

func pawn_moves(pos: Vector2):
    var dir = Vector2(0, alignment)
    var diags = [Vector2(1, alignment), Vector2(-1, alignment)]
    valid_move(pos + dir, true)
    for diag in diags:
        valid_move(pos + diag, false, true)

func king_moves(pos: Vector2):
    for dir in Globals.directions:
        valid_move(pos + dir)
    for diag in Globals.diags:
        valid_move(pos + diag)

func generate_moves(pos: Vector2 = current_pos):
    print("Please implement this method for your piece. It should make use of current_pos and add to available_moves")

func create_shader():
    var this_material = ShaderMaterial.new()
    this_material.set_shader(shader)
    set_material(this_material)
    this_mat = this_material

func set_shader():
    var bool_val = alignment == 1
    if this_mat:
        this_mat.set_shader_parameter("toggle", bool_val)

func set_outline(on: bool):
    if on:
        this_mat.set_shader_parameter("outline_color", Vector4(0.0, 1.0, 0.0, 1.0))
    else:
        this_mat.set_shader_parameter("outline_color", Vector4(0.0, 0.0, 0.0, 1.0))

func init(pos: Vector2, align: int = 1, piece_type: String = "Piece"):
    type = piece_type
    current_pos = pos
    alignment = align
    create_shader()
    set_shader()
    return self

func create_click_box():
    var block : Tile = board.tile.instantiate().init(Vector2(0,0), true, false).center_self()
    block.scale = Vector2(1/Globals.scale, 1/Globals.scale)
    block.color = Color(Globals.clear)
    block.connect("pressed", focused)
    add_child(block)
    block.position = -0.5 * block.get_parent_area_size()


# Called when the node enters the scene tree for the first time.
func _ready():
    board = get_parent()
    z_index = 1
    scale = Vector2(Globals.scale, Globals.scale)
    create_click_box()

func focused(pos: Vector2):
    pressed.emit(self)

func exhaust(force: bool = false):
    exhausted = true

func unexhaust(force: bool = false):
    exhausted = false

func move(pos:Vector2, interaction: int = 0):
    current_pos = pos
    position = pos * 32 + board.center
    if interaction != 0:
        exhaust()
 */

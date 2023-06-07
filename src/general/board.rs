//use std::collections::HashMap;

use godot::builtin::Color;
use godot::engine::{Node2D, Node2DVirtual};
use godot::prelude::*;

use crate::godot_rust_engine::{NodeExt2, PackedSceneExt};
//use crate::pieces::piece::Piece;

use super::tile::Tile;

//const HORSE: &str = "Horse";
//const QUEEN: &str = "Queen";
//const PAWN: &str = "Pawn";
//const KING: &str = "King";
//const BISHOP: &str = "Bishop";
//const ROOK: &str = "Rook";

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Board {
    pub white: Color,        //Color::from_string("DARK_SLATE_GRAY")  // Square color
    pub grey: Color,         // Color::from_string("DARK_SLATE_GRAY")  // Square color
    pub mod_color: Color,    // Color::from_string("YELLOW")  // For highlighting squares
    pub select_color: Color, // Color::from_string("GREEN") // For highlighting selected piece
    tile: Gd<PackedScene>,   // load("res://src/Tile.tscn")
    //horses: Gd<PackedScene>,  // load("res://src/Pieces/Knight.tscn")
    //rooks: Gd<PackedScene>,   // load("res://src/Pieces/Rook.tscn")
    //bishops: Gd<PackedScene>, // load("res://src/Pieces/Bishop.tscn")
    //queens: Gd<PackedScene>,  // load("res://src/Pieces/Queen.tscn")
    //pawns: Gd<PackedScene>,   // load("res://src/Pieces/Pawn.tscn")
    //kings: Gd<PackedScene>, // load("res://src/Pieces/King.tscn")
    //center: Vector2,
    //grid: HashMap<i8, Gd<Piece>>, // Map of what pieces are placed on the board
    //player_turn: i8,
    //max_energy: i8,
    //energy: i8,
    //initializing: bool,
    //player_pieces: HashMap<PieceTypes, Gd<Piece>>,
    //highlights: Vec<Gd<Tile>>,
    //selected_piece: Option<Gd<Piece>>,
    //progress_tile: Option<Gd<Tile>>,
    //progress_text: Option<Gd<Label>>,
    //energy_ui: Option<Gd<Label>>,
    board_width: i8,
    board_length: i8,

    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl Node2DVirtual for Board {
    fn init(base: Base<Node2D>) -> Self {
        Board {
            base,

            tile: load("res://src/Tile.tscn"),
            //horses: load("res://src/Pieces/Knight.tscn"),
            //rooks: load("res://src/Pieces/Rook.tscn"),
            //bishops: load("res://src/Pieces/Bishop.tscn"),
            //queens: load("res://src/Pieces/Queen.tscn"),
            //pawns: load("res://src/Pieces/Pawn.tscn"),
            //kings: load("res://src/Pieces/King.tscn"),
            white: Color::from_string("DARK_SLATE_GRAY").unwrap(),
            grey: Color::from_string("DARK_SLATE_GRAY").unwrap(),
            mod_color: Color::from_string("YELLOW").unwrap(),
            select_color: Color::from_string("GREEN").unwrap(),
            //center: Vector2::new(16., 16.),
            //max_energy: 5,
            board_length: 8,
            board_width: 6,
            //grid: HashMap::new(),
            //player_pieces: HashMap::new(),

            //highlights: Vec::new(),

            //initializing: true,
            //energy: 5,
            //player_turn: 1,

            //selected_piece: None,
            //progress_tile: None,
            //progress_text: None,
            //energy_ui: None,
        }
    }

    fn ready(&mut self) {
        self.draw_board();
    }
}

#[godot_api]
impl Board {
    #[signal]
    fn no_energy();

    #[signal]
    fn d_turn();

    #[func]
    fn w_edge(&self) -> i8 {
        self.board_width - 1
    }

    #[func]
    fn l_edge(&self) -> i8 {
        self.board_length - 1
    }

    /*     #[func]
       fn place_player_pieces(&self) {
           for y in 0 as i8..2 as i8 {
               for x in 0 as i8..4 as i8 {
                   let pos = Vector2::new(x as f32, (self.l_edge() - y) as f32);
                   match y {
                       0 => {
                           match x {
                               0 => {
                                   let scene = &self.horses;
                                   let piece = PieceTypes::Horse;
                                   //create_piece(this_piece[0], this_piece[1], pos, 1)
                               }
                               1 => {
                                   let scene = &self.kings;
                                   let piece = PieceTypes::King;
                                   //create_piece(this_piece[0], this_piece[1], pos, 1)
                               }
                               2 => {
                                   let scene = &self.bishops;
                                   let piece = PieceTypes::Bishop;
                                   //create_piece(this_piece[0], this_piece[1], pos, 1)
                               }
                               3 => {
                                   let scene = &self.rooks;
                                   let piece = PieceTypes::Rook;
                                   //create_piece(this_piece[0], this_piece[1], pos, 1)
                               }
                               _ => {
                                   godot_print!("This shouldn't happen...");
                               }
                           }
                       }
                       1 => {
                           //create_piece(self.pawns, PAWN, pos, 1)
                       }
                       _ => {
                           godot_print!("This shouldn't happen...");
                       }
                   }
               }
           }
       }
    */
    #[func]
    fn create_tile(&mut self, position: Vector2, color: Color) {
        let mut tile: Gd<Tile> = self.tile.instantiate_as::<Tile>();
        tile.bind_mut().create(position, color);
        self.add_child2(tile);
    }

    #[func]
    fn draw_board(&mut self) {
        for y in 0..self.board_length {
            for x in 0..self.board_width {
                let pos = Vector2::new(x as f32, y as f32);
                self.create_tile(pos, self.mod_color);
            }
        }
    }
}

/*



var key_help = ["none", 0]



var rng = RandomNumberGenerator.new()








# We want to have the grid squares that are targeted as the key
# and have the pieces location on the grid as an entry in another dictionary
var player_moves = {}
# We want to store all the moves the enemy can make with a similar schema.
var enemy_moves = {}

func rng_x():
    return rng.randi_range(0, Globals.w_edge)

func rng_y(back: int = 0, front: int = 3):
    return rng.randi_range(back, front)

func draw_init_tiles():
    clear_highlights()
    for x in Globals.board_width:
        for y in 3:
            var pos = Vector2(x, Globals.l_edge - y)
            if str(pos) not in grid:
                create_tile(pos, mod_color, true)

#func cleanup_enemy_moves(piece: Piece):


func add_enemy_moves(piece: Piece):
    for moves in piece.available_moves:
        pass
        #var entry = enemy_moves.get(str(move), {})
        #entry[str(piece.current_pos)] = piece

func must_create(type: PackedScene, dict_key, back: int = 0, front: int = 3, alignment : int = 1):
    var success = false
    while not success:
        var pos = Vector2(rng_x(), rng_y(back, front))
        var new_piece = create_piece(type, dict_key, pos, alignment)
        success = new_piece != null
        new_piece.generate_moves()

        enemy_moves[str(pos)] = new_piece.moves



func create_label(node: Node, text: String = "End Turn"):
    var label = Label.new()
    label.text = text
    label.position = Vector2(0, 0)
    node.add_child(label)
    return label

func init():
    draw_board()
    if initializing:
        var en = create_tile(Vector2(-4, 0), Gd<Color>::new(Globals.clear), false)
        energy_UI = create_label(en, "Energy: " + str(energy))
        var t = create_tile(Vector2(Globals.board_width + 2, 0), Gd<Color>::new("BLUE"), false).set_interact(true)
        progress_text = create_label(t)
        t.connect("pressed", handle_progress)
        progress_tile = t
    place_player_pieces()
    must_create(rooks, rook, 0, 1)
    must_create(bishops, bishop, 0, 1)
    must_create(horses, horse, 0, 1)
    must_create(kings, king, 0, 1)
    for p in 4:
        must_create(pawns, pawn, 1, 2)

func respawn():
    grid = {}
    player_pieces = {}
    var children = get_children()
    for child in children:
        if child is Camera2D:
            pass
        else:
            destroy_node(child)
    init()

func destroy():
    player_pieces = {}
    var children = get_children()
    for child in children:
        if child is Tile:
            var x = child.current_pos[0]
            var y = child.current_pos[1]
            if (x > Globals.w_edge || y > Globals.l_edge || not child.interactable):
                destroy_node(child)

func reload(reverse: bool):
    if(reverse):
        Globals.board_length -= 1
        Globals.w_edge -= 1
        Globals.board_width -= 1
        Globals.l_edge -= 1
    else:
        Globals.board_length += 1
        Globals.w_edge += 1
        Globals.board_width += 1
        Globals.l_edge += 1
    destroy()
    draw_board()

# Called when the node enters the scene tree for the first time.
func _ready():
    init()
    pass

func clear_highlights():
    for highlight in highlights:
        destroy_node(highlight)
    highlights = []

func destroy_node(node: Node):
    remove_child(node)
    node.queue_free()

func update_energy(new_val):
    if new_val is Callable:
        energy = new_val.call(energy)
    else:
        energy = new_val
    energy_UI.text = "Energy: " + str(energy)
    if energy <= 0:
        no_energy.emit(false, player_turn)

func move_piece(piece: Piece, pos: Vector2, interaction: int = 0):
    if piece.alignment != player_turn:
        return false
    if str(pos) in grid:
        var other_piece = grid[str(pos)]
        if other_piece.alignment != piece.alignment:
            if other_piece.type == king:
                end_game()
            destroy_node(other_piece)
        else:
            return false
    clear_highlights()
    grid.erase(str(piece.current_pos))
    grid[str(pos)] = piece
    piece.move(pos, interaction)
    if interaction != 0:
        update_energy(func(x): return x - 1)

func add_player_piece(piece: Sprite2D, type: String):
    var prev_pieces = []
    if type in player_pieces:
        prev_pieces = player_pieces[type]

    prev_pieces.append(piece)
    player_pieces[type] = prev_pieces

func create_piece(type: PackedScene, dict_key: String, pos: Vector2 = Vector2(0,0), alignment : int = 1):
    if(str(pos) in grid):
        return null;
    var new_piece = type.instantiate().init(pos, alignment, dict_key)
    new_piece.move(pos)
    self.connect("no_energy", new_piece.exhaust)
    self.connect("d_turn", new_piece.unexhaust)
    new_piece.connect("pressed", handle_piece_click)
    grid[str(pos)] = new_piece
    add_child(new_piece)
    if alignment != 1:
        add_player_piece(new_piece, dict_key)
    return new_piece;

func draw_board():
    var odd = true
    for y in Globals.board_length:
        odd = ! odd
        for x in Globals.board_width:
            odd = ! odd
            if odd:
                create_tile(Vector2(x, y), white, false)
            else:
                create_tile(Vector2(x, y), grey, false)

func create_tile(pos: Vector2, color: Gd<Color>, temp: bool):
    var t = tile.instantiate().init(pos, temp, temp)
    t.color = color
    add_child(t)
    if (temp):
        t.connect("pressed", handle_highlight_click)
        highlights.append(t)
    return t

func can_move(piece: Piece):
    return piece.alignment == player_turn

func draw_moves(piece: Piece, keyboard: bool):
    clear_highlights()
    if keyboard:
        create_tile(piece.current_pos, select_color, true).disconnect("pressed", handle_highlight_click)
    for move in piece.available_moves:
        if move is Vector2:
            if can_move(piece):
                create_tile(move, mod_color, true)
            else:
                create_tile(move, Gd<Color>::new("RED"), true)

func handle_key(type, reverse: bool):
    var last_type = key_help[0]
    var last_index = 0;
    if last_type == type:
        last_index = key_help[1]
        if reverse:
            last_index -= 1
        else:
            last_index += 1
    key_help = [type, last_index]
    if type in player_pieces:
        var pieces = player_pieces[type]
        if(pieces.size() > 0):
            var next_piece = pieces[last_index % pieces.size()]
            if next_piece:
                handle_moves(next_piece)

func handle_moves(piece: Piece):
    piece.generate_moves()
    draw_moves(piece, false)

func handle_piece_click(piece: Piece):
    if selected_piece:
        selected_piece.set_outline(false)
    selected_piece = piece
    selected_piece.set_outline(true)
    if initializing && selected_piece.alignment == player_turn:
        draw_init_tiles()
    else:
        handle_moves(piece)

func change_turn():
    player_turn *= -1
    update_energy(max_energy)
    d_turn.emit(false)

func end_game():
    if player_turn == 1:
        print("Black wins!")
    else:
        print("White wins!")

func handle_progress(pos: Vector2):
    print(player_turn)
    if initializing:
        initializing = false
    change_turn()

func handle_highlight_click(pos: Vector2):
    var type = 1
    if initializing:
        type = 0
    if selected_piece:
        move_piece(selected_piece, pos, type)
 */

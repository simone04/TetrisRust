mod tetris_engine;
mod user_controls;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use tetris_engine::*;
use user_controls::*;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

use sdl2::rect::Rect;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::ttf::Font;

const SQUARE :u32 = 30;
const COLUMNS : u32 = 10;
const ROWS : u32 = 20;
const PADDING : u32 = 20;

const LEFT_AREA_WIDTH : u32 = 150;
const CENTER_AREA_WIDTH : u32 = PADDING*2 + BOARD_WIDTH;

const BOARD_WIDTH  : u32 = SQUARE * COLUMNS;
const BOARD_HEIGHT : u32 = SQUARE * ROWS;

const RIGHT_AREA_WIDTH : u32 = 150;

const WIDTH : u32 = LEFT_AREA_WIDTH + BOARD_WIDTH + 2 * PADDING + RIGHT_AREA_WIDTH;
const HEIGHT : u32 = BOARD_HEIGHT + 2 * PADDING;

const HOLD_SQUARE_SIZE : u32 = 20;
const HOLD_WIDTH: u32 = HOLD_SQUARE_SIZE * 4;
const HOLD_HEIGHT: u32 = HOLD_SQUARE_SIZE * 3;
const HOLD_MARGIN: u32 = 20;
const HOLD_PADDING: u32 = 10;

const NEXTPIECE_HEIGHT : u32 = HOLD_SQUARE_SIZE * 15;

const GHOST_COLOR : Color = Color::RGBA(255,255,255, 128);


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video.window("Tetris", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let font_path = "./src/DejaVuSans.ttf";
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut events = sdl_context.event_pump().unwrap();
    let font = ttf_context.load_font(font_path, 24).unwrap();

    loop{
        let mut game: Game = Game::new();
        let mut userControl = UserControl::new();

        let mut offset = (0,  0);
        
        let stop = 'running : loop  {
            for event in events.poll_iter() {
                match event {
                    Event::Quit {..} | 
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running true,
                    Event::KeyDown { keycode: Some(Keycode::R), .. } => break 'running false,

                    Event::KeyDown { keycode: Some(key), .. } => {
                        if userControl.action(&mut game, key, true){
                            break 'running false
                        }
                    },
                    Event::KeyUp { keycode: Some(key), .. } => {
                        if userControl.action(&mut game, key, false){
                            break 'running false
                        }
                    },
                    _ => {}
                }
            }
            if userControl.update(&mut game){
                break 'running false;
            }

            canvas.set_draw_color(Color::RGB(100, 100, 100));
            canvas.clear();

            renderBoard(&mut canvas, &game, offset);
            drawHold(&mut canvas, &game);
            drawNexts(&mut canvas, &game);
            drawLines(&mut canvas, &game, &font);

            canvas.present();
            std::thread::sleep(Duration::from_millis(16)); // 60 FPS
        };

        if stop{
            break;
        }
    }
}



fn draw_square(canvas: &mut Canvas<Window>, (x, y) : (i16, i16), color: Color, square : i16, position : (i16, i16)) {
    let offset : i16 = 1;
    let sq : i16 = square as i16 - offset*2;
    let corner : i16 = 3;
    let mut vx = [corner, sq - corner, sq, sq, sq - corner, corner, 0, 0];
    let mut vy= [0, 0, corner, sq - corner, sq, sq, sq-corner, corner];

    for i in 0..8{
        vx[i] += position.0 + square*x+offset;
        vy[i] += position.1 + square*y+offset;
    }

    let _ = canvas.filled_polygon(&vx, &vy, color);

    //canvas.set_draw_color(Color::RGB(color.r, color.g, color.b));
    //canvas.fill_rect(Rect::new(x * (SQUARE as i32) + PADDING as i32, y * (SQUARE as i32) + PADDING as i32, SQUARE, SQUARE)).unwrap();
    
}

fn renderPiece(canvas: &mut Canvas<Window>, (pos_x,pos_y) : (i8, i8), tetromino : &Tetromino, color : Color, position : (i16, i16)){
    for (x, y) in tetromino{
        let pos  = ((x + pos_x) as i16, (pos_y - y) as i16);
        draw_square(canvas, pos, color, SQUARE as i16, position);
    }
}



fn renderBoard(canvas: &mut Canvas<Window>, game : &Game, offset : (i32, i32)){
    let rect = Rect::new(
        (PADDING + LEFT_AREA_WIDTH) as i32 + offset.0, 
        PADDING as i32 + offset.1, 
        BOARD_WIDTH, BOARD_HEIGHT
    );
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_rect(rect);

    let position = (rect.x as i16, rect.y as i16);

    for y in 0..20{
        for x in 0..10{
            if game.board[y][x] != 0 {
                let color = getColor(game.board[y][x]);
                draw_square(
                    canvas, 
                    (x as i16, y as i16), 
                    color, 
                    SQUARE as i16, 
                    position
                );
            }
        }
    }
    let tetromino = &game.current_tetromino();

    renderPiece(canvas, game.get_ghost(), tetromino, GHOST_COLOR, position);
    renderPiece(canvas, game.current_position, tetromino, getColor(game.current_piece.index), position);    
}

fn getColor(square : u8) -> Color{
    match square {
        1 => Color::RGB(0, 255, 255),
        2 => Color::RGB(255, 255, 0),
        3 => Color::RGB(128, 0, 128),
        4 => Color::RGB(0, 255, 0),
        5 => Color::RGB(255, 0, 0),
        6 => Color::RGB(0, 0, 255),
        7 => Color::RGB(255, 165, 0),
        _ => Color::RGB(255, 255, 255),
    }
}


fn drawHold(canvas: &mut Canvas<Window>, game : &Game){
    let rect = Rect::new(
        (LEFT_AREA_WIDTH-HOLD_WIDTH-HOLD_MARGIN-HOLD_PADDING) as i32, 
        (HOLD_MARGIN-HOLD_PADDING) as i32, 
        HOLD_WIDTH + 2*HOLD_PADDING, HOLD_HEIGHT + 2*HOLD_PADDING);
    
    match game.hold_piece{
        Some(piece) => {
            let tetromino = piece.rotations[0];
            let color = getColor(piece.index);

            let pos  = ((LEFT_AREA_WIDTH-HOLD_WIDTH-HOLD_MARGIN) as i16, HOLD_MARGIN as i16);
            for (x, y) in tetromino{
                draw_square(canvas, (1 + x as i16, 1 - y as i16), color, HOLD_SQUARE_SIZE as i16, pos);
            }
        },
        None => {
            
        }
    }


    canvas.set_draw_color(Color::WHITE);
    canvas.draw_rect(rect);
}

fn drawNexts(canvas: &mut Canvas<Window>, game : &Game){
    let rect = Rect::new(
        (WIDTH-RIGHT_AREA_WIDTH + HOLD_MARGIN - HOLD_PADDING) as i32, 
        (HOLD_MARGIN - HOLD_PADDING) as i32, 
        HOLD_WIDTH + 2*HOLD_PADDING, NEXTPIECE_HEIGHT + 2*HOLD_PADDING);
    
    
    let pos  = ((WIDTH-RIGHT_AREA_WIDTH + HOLD_MARGIN) as i16, HOLD_MARGIN as i16);
    
    let pieces = game.get_nexts();
    for i in 0..5 {
        let piece = pieces[i];
        let tetromino = piece.rotations[0];
        let color = getColor(piece.index);
        for (x, y) in tetromino{
            draw_square(canvas, (1 + x as i16, 1 + (i as i16)*3 - y as i16), color, HOLD_SQUARE_SIZE as i16, pos);
        }
    }

    canvas.set_draw_color(Color::WHITE);
    canvas.draw_rect(rect);
}

fn draw_text(canvas: &mut Canvas<Window>, font : &Font<'_, 'static>, text : &str, (x, y): (i32, i32), color : Color){
    let surface = font
        .render(text)
        .blended(color)
        .unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let target = Rect::new(x, y, surface.width(), surface.height());

    canvas.copy(&texture, None, Some(target)).unwrap();
}

fn drawLines(canvas: &mut Canvas<Window>, game : &Game, font : &Font<'_, 'static>){

    draw_text(canvas, font, "SCORE", ((WIDTH - RIGHT_AREA_WIDTH )as i32, HEIGHT as i32 - 200), Color::WHITE);
    draw_text(canvas, font, &game.score.to_string(), ((WIDTH - RIGHT_AREA_WIDTH )as i32, HEIGHT as i32 - 175), Color::WHITE);

    draw_text(canvas, font, "LINES", ((WIDTH - RIGHT_AREA_WIDTH )as i32, HEIGHT as i32 - 100), Color::WHITE);
    draw_text(canvas, font, &game.lines_cleared.to_string(), ((WIDTH - RIGHT_AREA_WIDTH )as i32, HEIGHT as i32 - 75), Color::WHITE);
}
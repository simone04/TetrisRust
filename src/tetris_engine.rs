use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;

const HEIGHT : usize = 20;
const WIDTH : usize = 10;

pub type Tetromino = [(i8,i8); 4];

pub struct Piece {
    pub rotations : [Tetromino; 4],
    pub index : u8,
    name : char, 
}

pub static PIECES : [Piece; 7]   = [
    Piece {
        rotations : [
            [(-1, 0), (0, 0), (1, 0), (2, 0)],
            [(0, 1), (0, 0), (0, -1), (0, -2)],
            [(-2, 0), (-1, 0), (0, 0), (1, 0)],
            [(0, 2), (0, 1), (0, 0), (0, -1)]
        ],
        index : 1,
        name : 'I',
    },
    Piece {
        rotations : [
            [(-1, 1), (-1, 0), (0, 0), (1, 0)],
            [(0, 1), (1, 1), (0, 0), (0, -1)],
            [(-1, 0), (0, 0), (1, 0), (1, -1)],
            [(0, 1), (0, 0), (0, -1), (-1, -1)]
        ],
        index : 6,
        name : 'J',
    },
    Piece {
        rotations : [
            [(-1, 0), (0, 0), (1, 0), (1, 1)],
            [(0, 1), (0, 0), (0, -1), (1, -1)],
            [(-1, 0), (0, 0), (1, 0), (-1, -1)],
            [(-1, 1), (0, 1), (0, 0), (0, -1)]
        ],
        index : 7,
        name : 'L',
    },
    Piece {
        rotations : [
            [(0, 1), (1, 1), (0, 0), (1, 0)],
            [(0, 0), (1, 0), (0, -1), (1, -1)],
            [(-1, 0), (0, 0), (-1, -1), (0, -1)],
            [(-1, 1), (0, 1), (-1, 0), (0, 0)]
        ],
        index : 2,
        name : 'O',
    },
    Piece {
        rotations : [
            [(0, 1), (1, 1), (-1, 0), (0, 0)],
            [(0, 1), (0, 0), (1, 0), (1, -1)],
            [(0, 0), (1, 0), (-1, -1), (0, -1)],
            [(-1, 1), (-1, 0), (0, 0), (0, -1)]
        ],
        index : 4,
        name : 'S',
    },
    Piece {
        rotations : [
            [(0, 1), (-1, 0), (0, 0), (1, 0)],
            [(0, 1), (0, 0), (1, 0), (0, -1)],
            [(-1, 0), (0, 0), (1, 0), (0, -1)],
            [(0, 1), (-1, 0), (0, 0), (0, -1)]
        ],
        index : 3,
        name : 'T',
    },
    Piece {
        rotations : [
            [(-1, 1), (0, 1), (0, 0), (1, 0)],
            [(1, 1), (0, 0), (1, 0), (0, -1)],
            [(-1, 0), (0, 0), (0, -1), (1, -1)],
            [(0, 1), (-1, 0), (0, 0), (-1, -1)]
        ],
        index : 5,
        name : 'Z',
    }
];

static OFFSET_DATA : [[(i8,i8); 5]; 4] = [
    [(0,0), (0,0), (0,0), (0,0), (0,0)],
    [(0,0), (1,0), (1, -1), (0, 2), (1,2)],
    [(0,0), (0,0), (0,0), (0,0), (0,0)],
    [(0,0), (-1,0), (-1,-1), (0,2), (-1,2)]
];

static I_OFFSET_DATA : [[(i8,i8); 5]; 4] = [
    [(0,0), (-1,0), (2,0), (-1,0), (2,0)],
    [(-1,0), (0,0), (0,0), (0,1), (0,-2)],
    [(-1,1), (1,1), (-2,1), (1,0), (-2,0)],
    [(0,1), (0,1), (0,1), (0,-1), (0,2)] 
];

static O_OFFESET_DATA : [(i8,i8); 4] = [
    (0,0), (0,-1), (-1,-1), (-1,0)
];


fn calc_kicks(piece : char, start : usize, end : usize) -> [(i8, i8); 5] {
    let mut kicks : [(i8, i8); 5] = [(0,0); 5];
    for k in 0..5{
        let array = if piece == 'I' {&I_OFFSET_DATA} else {&OFFSET_DATA};
        let (start_x,start_y) = array[start][k];
        let (end_x, end_y) = array[end][k];

        kicks[k] = (start_x - end_x, start_y - end_y);
    }
    return kicks;
}

fn calc_o_kick(start : usize, end : usize) -> (i8, i8) {
    let array = O_OFFESET_DATA;
    let (start_x,start_y) = array[start];
    let (end_x, end_y) = array[end];

    return (start_x - end_x, start_y - end_y);
}

struct Bag {
    queue: Vec<usize>, 
    next_pieces: Vec<usize>
}

impl Bag {
    pub fn new() -> Self {
        let mut bag = Bag { 
            queue: Vec::new(), 
            next_pieces: Vec::new() 
        };
        bag.refill();
        for _ in 0..5{
            let piece = bag.queue.remove(0);
            bag.next_pieces.push(piece);
        }
        return bag;
    }

    fn refill(&mut self) {
        let mut indices: Vec<usize> = (0..7).collect();
        indices.shuffle(&mut thread_rng());
        self.queue.extend(indices);
    }

    pub fn next(&mut self) -> usize {
        if self.queue.is_empty() {
            self.refill();
        }
        let piece = self.queue.remove(0);
        self.next_pieces.push(piece);
        self.next_pieces.remove(0)

    }
}



pub struct Game<'a>{
    pub board : [[u8; WIDTH]; HEIGHT],
    pub current_piece : &'a Piece,

    pub current_rotation : usize,
    pub current_position : (i8, i8),

    game_over : bool,

    pub score : u32,
    level : u32,
    pub lines_cleared : u32,

    pub hold_piece : Option<&'a Piece>,
    pub already_switched : bool,

    bag : Bag,
}

impl<'a> Game<'a>{

    pub fn new() -> Self{
        let mut bag = Bag::new();
        let piece_1 = &PIECES[bag.next()];

        Self{
            board : [[0; WIDTH]; HEIGHT],
            current_piece : piece_1,
            current_rotation : 0,
            current_position : (4,0), 
            game_over : false,
            score : 0,
            level : 0,
            lines_cleared : 0,
            hold_piece : None,
            already_switched : false,
            bag : bag,
        }
    }

    fn printBoard(&self){
        for y in 0..20{
            for x in 0..10{
                print!("{} ", self.board[y][x]);
            }
            println!("");
        }
        println!("");
    }

    fn printTetromino(&self){
        let tetromino = self.current_tetromino();
        let (x, y) = self.current_position;
        
        println!("current tetromino  {}, {}", self.current_position.0, self.current_position.1);
        for (t_x,t_y) in tetromino{
            let (x1, y1) = (x + t_x, y - t_y);
            println!("{} {}", x1, y1);
        }
        println!("");
    }

    pub fn current_tetromino(&self) -> Tetromino{
        self.current_piece.rotations[self.current_rotation]
    }

    pub fn check_tetromino(&self, position : (i8, i8), tetromino : &Tetromino) -> bool{
        //self.printTetromino();
        //println!("---");
        for (t_x,t_y) in tetromino{
            let (x, y) = (position.0 + t_x, position.1 - t_y);
            //println!("{} {}", x, y);
            if x < 0 || x>=10 || y>=20{return false;}
            if y>=0{
                if self.board[y as usize][x as usize] != 0 {return false;}
            }
 
        }
        //println!("---");
        return true;
    }

    pub fn hold_piece(&mut self) -> bool{
        if self.already_switched{
            return false;
        }
        self.already_switched = true;
        match self.hold_piece{
            Some(piece) => {
                let hold = self.current_piece;
                self.summon_piece(piece);
                self.hold_piece = Some(hold);
                
            },
            None => {
                self.hold_piece = Some(self.current_piece);
                let x = self.get_next();
                self.summon_piece(x);
            },
        }   
        return true;
    }

    pub fn get_next(&mut self) -> &'a Piece{
        return &PIECES[self.bag.next()];
    }

    pub fn get_nexts(&self) -> [&Piece; 5]{
        self.bag.next_pieces
            .iter()
            .take(5)
            .map(|&idx| &PIECES[idx])
            .collect::<Vec<&Piece>>()
            .try_into()
            .unwrap_or_else(|v: _| panic!("Tua mamma puttana"))
    }

    pub fn move_piece(&mut self, dir : i8) -> bool{
        let tetromino = &self.current_tetromino();
        let (x, y) = (self.current_position.0 + dir, self.current_position.1);
        if self.check_tetromino((x,y), tetromino){ 
            self.current_position.0 += dir;
            return true
        }
        return false
    }

    fn summon_piece(&mut self, piece : &'a Piece){
        self.current_piece = piece;
        self.current_rotation = 0;
        self.current_position = (4,0);
    }

    pub fn hard_drop(&mut self) -> bool{
        while self.drop(){

        }
        return self.place();
    }

    pub fn soft_drop(&mut self){

    }

    pub fn drop(&mut self) -> bool{
        let (x, y) = (self.current_position.0, self.current_position.1 + 1);
        if self.check_tetromino((x, y), &self.current_piece.rotations[self.current_rotation]){
            self.current_position.1 += 1;
            return true;
        }
        return false;
    }

    pub fn hard_move(&mut self, dir : i8){
        while self.move_piece(dir){
            
        }
    }

    pub fn rotate(&mut self, offset : i32){
        let new_rotation = (self.current_rotation + offset as usize) % 4;
        
        if self.current_piece.name == 'O'{
            let kick = calc_o_kick(self.current_rotation, new_rotation);
            self.current_position.0 += kick.0;
            self.current_position.1 -= kick.1;
            self.current_rotation = new_rotation;
            return
        }

        let kicks = calc_kicks(
            self.current_piece.name, 
            self.current_rotation, 
            new_rotation
        );
        let mut last_kick = (0,0);
        let mut found = false;
        for kick in kicks{
            let (x, y) = (self.current_position.0 + kick.0, self.current_position.1 - kick.1);
            if self.check_tetromino((x,y), &self.current_piece.rotations[new_rotation]){
                last_kick = kick;
                found = true;
                break
            }
        }
        if found {
            self.current_position.0 += last_kick.0;
            self.current_position.1 -= last_kick.1;
            self.current_rotation = new_rotation;
        }
        
    }

    fn clear_lines(&mut self) -> u32{
        let mut line: u32 = 20;

        for y in (0..20).rev(){
            let mut full = true;
            for x in 0..10{

                if self.board[y][x] == 0{
                    full = false;
                    break;
                }
            }
            if full {continue}
            self.board[line as usize - 1] = self.board[y];
            line -= 1;
        }
        for y in (0..line).rev(){
            self.board[y as usize] = [0; 10];
        }

        return line
    }

    pub fn get_level(&self) -> u32{
        return self.lines_cleared / 10;
    }

    fn update_score(&mut self, lines : u32){
        let l = self.get_level();
        let score =  match lines {
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => 0,
        };
        self.score += score * (l+1);
    }

    pub fn place(&mut self) -> bool{
        for (t_x,t_y) in self.current_piece.rotations[self.current_rotation]{
            let (x, y) = (self.current_position.0 + t_x, self.current_position.1 - t_y);
            if y<0{
                return true;
            }

            self.board[y as usize][x as usize] = self.current_piece.index
        }

        let lines= self.clear_lines();
        self.update_score(lines);
        self.lines_cleared += lines;

        let x = self.get_next();
        self.summon_piece(x);
        self.already_switched = false;
        return false;
        //self.printBoard();
    }

    pub fn get_ghost(&self) -> (i8, i8){
        let (x, mut y) = (self.current_position.0, self.current_position.1 + 1);
        loop {
            if self.check_tetromino((x, y), &self.current_piece.rotations[self.current_rotation]){
                y += 1;
                continue;
            }
            break;
        }
        return (x, y-1);
    }   
    
}
    


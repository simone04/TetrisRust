use std::{cmp::max, collections::HashMap};

use crate::tetris_engine::Game;
use sdl2::{keyboard::Keycode};



#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Action {NONE, LEFT, RIGHT, DOWN, ROTATE_C, ROTATE_A, ROTATE_H, DROP, HOLD}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {LEFT, RIGHT, NONE}

struct KeyMap {
    map: HashMap<Keycode, Action>,
}

impl KeyMap {
    fn new() -> Self {
        let mut map = HashMap::new();
        // Esempio di inizializzazione
        map.insert(Keycode::A, Action::LEFT);
        map.insert(Keycode::D, Action::RIGHT);
        map.insert(Keycode::S, Action::DOWN);
        map.insert(Keycode::Space, Action::DROP);
        map.insert(Keycode::L, Action::ROTATE_C);
        map.insert(Keycode::J, Action::ROTATE_A);
        map.insert(Keycode::K, Action::ROTATE_H);
        map.insert(Keycode::LShift, Action::HOLD);


        Self { map }
    }

    fn get(&self, key: &Keycode) -> Option<&Action> {
        self.map.get(key)
    }
}

struct ActionMap {
    map: HashMap<Action, bool>,
}

impl ActionMap {
    fn new() -> Self {
        let mut map = HashMap::new();
        // Esempio di inizializzazione
        map.insert(Action::LEFT, false);
        map.insert(Action::RIGHT, false);
        map.insert(Action::DOWN, false);
        map.insert(Action::DROP, false);
        map.insert(Action::ROTATE_C, false);
        map.insert(Action::ROTATE_A, false);
        map.insert(Action::ROTATE_H, false);
        map.insert(Action::HOLD, false);


        Self { map }
    }

    fn get(&self, key: &Action) -> Option<&bool> {
        self.map.get(key)
    }

    fn set(&mut self, key: Action, value: bool) {
        self.map.insert(key, value);
    }
}

pub struct Handling{
    gravity_frame: u32,
    das_delay: u32,
    arr: u32,
    sdf : u32,
    lock_delay: u32,
}

impl Handling {
    pub fn new() -> Self{
        Self{
            gravity_frame: 40,
            das_delay: 10,
            arr: 1,
            lock_delay: 60,
            sdf : 30,
        }
    }
}

pub struct UserControl{
    key_map: KeyMap,
    action_map: ActionMap,
    handling : Handling,

    frame : u32,

    
    direction : Direction,
    dropping : bool,
    gravity_frame: u32,
    hold: u32,
    arr : u32,
    lock_delay: u32,
    touching : bool,
}

impl UserControl{
    pub fn new() -> Self{
        Self{
            key_map: KeyMap::new(),
            action_map : ActionMap::new(),
            handling : Handling::new(),
            frame : 0,
        
            dropping : false,
            gravity_frame: 0,
            hold: 0,
            direction : Direction::NONE,
            arr : 0,
            lock_delay: 0,
            touching : false,
        }
    }

    pub fn action(&mut self, game : &mut Game, key : Keycode, pressed : bool){
        let action_option = self.key_map.get(&key);
        match action_option {
            Some(val) => {

                let action = *val;
                if pressed{ 
                    self.action_map.set(action, true);  

                    match action {
                        Action::LEFT | Action::RIGHT => {
                            let direction = if action == Action::LEFT {Direction::LEFT} else {Direction::RIGHT};
                            if self.direction != direction{
                                self.hold = 0;
                                self.arr = self.frame;
                                self.direction = direction;
                                game.move_piece(if Direction::LEFT == direction {-1} else {1});
                            }
                        },
                        Action::ROTATE_C | Action::ROTATE_A | Action::ROTATE_H => {
                            game.rotate(
                                if action == Action::ROTATE_C {1} else if action == Action::ROTATE_A {3} else {2}
                            );
                        },
                        Action::DROP => {
                            game.hard_drop();
                        },
                        Action::DOWN => {
                            self.dropping = true;
                        },
                        Action::HOLD => {
                            if game.hold_piece(){
                                self.touching = false;
                                self.lock_delay = 0;
                            } 
                        },
                        _ => (),       
                    }
                }else{
                    self.action_map.set(action, false);  
                    match action {
                        Action::LEFT | Action::RIGHT => {
                            self.hold = 0;
                            self.arr = self.frame;
                            self.direction = Direction::NONE;
                        },
                        Action::DOWN => {
                            self.dropping = false;
                        },
                        _ => (),       
                    }  
                }
            }
            None => (),
        }

    }

    pub fn update(&mut self, game : &mut Game){
        self.frame += 1;

        self.hold += 1;

        if self.touching{
            self.lock_delay += 1;
            if self.lock_delay >= self.handling.lock_delay{
                game.place();
                self.lock_delay = 0;
            }
        }

        //println!("{} {}", self.frame, self._gravity_frame);
        let calculated_gravity = max(1, 48 - (game.get_level() * 5));
        let gravity_frame = if self.dropping { calculated_gravity / self.handling.sdf} else {calculated_gravity}; //self.handling.gravity_frame
        if self.frame - self.gravity_frame >= gravity_frame{
            self.gravity_frame = self.frame;

            
            if game.drop(){
                self.lock_delay = 0;
                self.touching = false;
            }else{
                self.touching = true
            }
        }

        match self.direction {
            Direction::LEFT | Direction::RIGHT => {
                //println!("{} {} - {} {}", self._hold, self.das_delay, self.frame, self._arr);
                if self.hold >= self.handling.das_delay {
                    if self.frame - self.arr >= self.handling.arr{
                        game.move_piece(if self.direction == Direction::LEFT {-1} else {1});

                        self.arr = self.frame;
                    }
                }
            },
            _ => (),
        }
    }

}
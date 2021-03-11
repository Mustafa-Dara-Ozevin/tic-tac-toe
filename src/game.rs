use std::{cmp::max, usize};
use text_io::read;
use Mark::Cross;
use Mark::Empty;
use Mark::Circle;

#[derive(PartialEq,Copy,Clone,Debug)]
pub enum Mark {
    Circle=-1,
    Empty,
    Cross
    
}

impl Mark {
    
}


pub struct Game{
    pub board: [Mark;9],
    pub turn: Mark
}

impl Game {
    pub fn new() -> Game{
        Game{
            board :  [Empty;9],
            turn: Cross
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game{
    pub fn clear_board(&mut self){
        self.board = [Empty;9];
    }
    fn print(&self){
        println!("-------------");
        for i in 0..9{
            match  self.board[i]{
                Circle => print!("| O "),
                Cross => print!("| X "),
                Empty => print!("|   ")
            }

            if (i+1) % 3 == 0{
                println!("|\n-------------");
            }
        }
    }
    fn set_tile(&mut self,cord:usize){
        assert!(cord < 9);
        if Empty == self.board[cord]{
            self.board[cord] = self.turn;
            match self.turn {
                Circle => {self.turn = Cross;}
                Empty => {return;}
                Cross => {self.turn = Circle;}
            }
            
        }else{
            println!("This tile is already occupied try again!");
        }
    }
    fn is_game_ended(&self)->Option<Mark>{
        for i in 0..3{
            if self.board[i] == self.board[i+3] && self.board[i] == self.board[i+6] && self.board[i] != Empty {
                return Some(self.board[i]);
            }
        }
        let rows: [usize;3] = [0,3,6];
        for i in  rows.iter(){
            if self.board[*i] == self.board[*i+1] && self.board[*i] == self.board[*i+2] && self.board[*i] != Empty {
                return Some(self.board[*i]);
            }
        }
        if self.board[0] == self.board[4] && self.board[0] == self.board[8] && self.board[0] != Empty{
            return Some(self.board[0]);
        }
        if self.board[6] == self.board[4] && self.board[6] == self.board[2] && self.board[6] != Empty{
            return Some(self.board[6]);
        }
        for i in 0..9 {
            if self.board[i] == Empty {
                return None;
            }
        }
        return Some(Empty);
    }
    fn get_tile(&mut self){
        println!("Please enter a tile: ");
        let i: usize = read!();
        self.set_tile(i-1);
    }
    pub fn start_game(&mut self) {
        self.print();
        while self.is_game_ended() == None {
            self.get_tile();
            println!("\n");
            self.print();
        }
        let result = self.is_game_ended();
        match result {
            Some(x) => {
                match x {
                    Circle => {println!(r#"Circle Wins!"#)}
                    Empty => {println!(r#"Draw!"#)}
                    Cross => {println!(r#"Cross Wins!"#)}
                }
            }
            None => {println!("error!")}
        }
    }
}
impl Game{
    fn evaluate(&self)->i32{
        let result = self.is_game_ended();
        match result {
            Some(x) => {
                match x {
                    Circle => {return -5000}
                    Empty => {return 0}
                    Cross => {return 5000}
                }
            }
            None => {return 0;}
        }
    }
    fn get_legal_moves(&self)->Vec<usize>{
        let mut legal_moves: Vec<usize> = Vec::new();
        for i in 0..9{
            if self.board[i] == Empty{
                legal_moves.push(i);
            }
        }
        return legal_moves;
    }
    fn takeback(&mut self,cord:usize){
        if self.board[cord] != Empty{
            self.board[cord] = Empty;
            match self.turn {
                Circle => {self.turn = Cross;}
                Empty => {return;}
                Cross => {self.turn = Circle;}
            }
        }
        else{
            println!("Tile is already empty!")
        }
    }
    fn negamax(&mut self,)->i32{
        if self.is_game_ended() != None{
            return self.evaluate() * self.turn as i32;
        }
        let mut v = -1000000;
        let legal_moves = self.get_legal_moves();
        for legal_move in legal_moves.iter(){
            self.set_tile(*legal_move);
            v = max(v, -self.negamax());
            self.takeback(*legal_move);
        }
        return v;
    }
    fn ai_play(&mut self){
        let mut v = -1000000;
        let legal_moves = self.get_legal_moves();
        let mut best_move:usize =0;
        for legal_move in legal_moves.iter(){
            self.set_tile(*legal_move);
            let score = -self.negamax();
            self.takeback(*legal_move);
            if score > v{
                best_move = *legal_move;
                v = score
            }
        }
        self.set_tile(best_move);
    }
    pub fn start_game_vs_ai(&mut self){
        println!("Do you want to go first or second(type 1 or 2): ");
        let i: i8 = read!();
        self.print();
        while self.is_game_ended() == None {
            if i == 1{
                self.get_tile();
                println!("\n");
                self.print();
            }
            if self.is_game_ended() != None{
                break;
            }
            self.ai_play();
            println!("\n");
            self.print();
            if i == 2 && self.is_game_ended() == None{
                self.get_tile();
                println!("\n");
                self.print();
            }
        }
        let result = self.is_game_ended();
        match result {
            Some(x) => {
                match x {
                    Circle => {println!(r#"Circle Wins!"#)}
                    Empty => {println!(r#"Draw!"#)}
                    Cross => {println!(r#"Cross Wins!"#)}
                }
            }
            None => {println!("error!")}
        }

    }
}
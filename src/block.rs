extern crate chrono;
use chrono::{DateTime,Utc};
pub type Hash = Vec<u8>;

    use std::io;
    pub trait BlockChain{
        // fn new(&mut self)->Block;
        fn genesis_block() -> Block;
        fn block_instance(&mut self,id: u32,timestamp: String,hash: Hash,prev_hash: Hash,data: String) -> Block;
        fn mine_block(&self,block: &Block);
        fn input_data(&mut self);
        fn output_data(&self);
        fn output_block(block: &Block);
        // fn show_chain(self,blockchain: &[Block]);
        fn show_chain(&self,blockchain: &[Block]);
        fn combine_string(&self) -> String;
        // type Hash = Vec<u8>;
        // fn hash(data: String) -> Hash;
        // fn current_time() -> String;
        
        // fn hash(&self,file: &String) -> String;
        // fn 
    }
    pub struct Block{
        // pub_key:    String,
        // pri_key:    String,
        id:         u32,
        // timestamp:  u32,
        timestamp:  String,
        // hash:       String,
        hash: Hash,
        prev_hash:  Hash,
        data:       String,
    }

    impl BlockChain for Block{
        // #[test]
        // fn new() -> Block{
        //     Block{
        //         id: 0u32,
        //         hash: "none".to_string(),
        //         prev_hash: "none".to_string(),
        //         data: "none".to_string(),
        //     }
        // }

        // fn new(&mut self) -> Block{
        //     Block{
        //         id:     
        //     }
        // }

        // fn genesis_block()->Block{
        //     Block{
        //         id: 0u32,
        //         timestamp: current_time(),
        //         hash: "none".to_string(),
        //         prev_hash: "none".to_string(),
        //         data: "none".to_string(),
        //     }
        // }
        // use super::*;
        fn genesis_block()->Block{
            let mut s = String::new();
            let mut s2 = String::new();
            let time = current_time();
            let mut value_hash: Vec<u8> = Vec::new();
            io::stdin().read_line(&mut s).expect("Invalid input");
            // time = current_time();
            // let mut value_hash = String::new();
            // let mut value_hash = vec![u8;32];
            // let mut s1 = String::new();
            s2 = st(&s);
            let mut s1 = s2 + &time;
            // s1 = s.to_string() + time.to_string();
            // s1 = s + time;
            value_hash = hash(&s1);
            Block{
                id: 0u32,
                // timestamp: current_time(),
                timestamp: time,
                // hash: hash(id.to_string() + timestamp + data),
                // hash: vec![0;32],
                // hash: hash(&value_hash),
                hash: value_hash, 
                // hash: vec_to_str(&Hash),
                // prev_hash: "none".to_string(),
                prev_hash: vec![0;32],
                data: s,
                // println!("input data: ");
                // io::stdin().read_line(&mut self.data).expect("Input Error");
            }
        }


        fn block_instance(&mut self,id: u32,timestamp: String,hash: Hash,prev_hash: Hash,data: String) -> Block{
            Block{
                id,
                timestamp,
                hash,
                prev_hash,
                data,   
            }
        }

        fn mine_block(&self, block: &Block){
            // let blockchain: [Block; 0] = [];
            let mut blockchain: Vec<&Block> = Vec::new();
            blockchain.push(block);

        }
        fn input_data(&mut self){
            let mut id = String::new();
            println!("input id: ");
            io::stdin().read_line(&mut id).expect("Something went wrong!");
            self.id = id.trim().parse().unwrap();
            // println!("input hash: ");
            // io::stdin().read_line(&mut self.hash).expect("Something went wrong!");
            // println!("input prev_hash: ");
            // io::stdin().read_line(&mut self.prev_hash).expect("Something went wrong!");
            println!("input data: ");
            io::stdin().read_line(&mut self.data).expect("Something went wrong");
        }
        fn output_data(&self){
            println!("ID:           {}",&self.id);
            println!("Timestamp:    {}",&self.timestamp);
            println!("Prev_hash:    {}",vec_to_str(&self.prev_hash));
            println!("Hash:         {}",vec_to_str(&self.hash));
            println!("Data:         {}",&self.data);
        }
        fn output_block(block: &Block){
            println!("ID:           {}",block.id);
            println!("Timestamp:    {}",block.timestamp);
            println!("Prev_hash:    {}",vec_to_str(&block.prev_hash));
            // println!("Hash:         {:?}",block.hash);
            // let mut st = String::new();
            // st = vec_to_str(&block.hash);
            println!("Hash:         {}",vec_to_str(&block.hash));
            // println!("Hash:         {}",st);
            println!("Data:         {}",block.data);
        }
        // fn show_chain(&self,blockchain: &[Block]);
        fn show_chain(&self,blockchain: &[Block]){
            for block in blockchain{
                Block::output_block(&block);
                println!("----------*----------");
            }
        }

        fn combine_string(&self) -> String{
            let mut st = String::new();
            st = st + &self.id.to_string() + &self.timestamp + &vec_to_str(&self.hash) + &vec_to_str(&self.prev_hash) + &self.data;
            st
        }
        // type Hash = Vec<u8>;
        // fn hash (data: String) -> Hash {
        //     crypto_hash::digest(crypto_hash::Algorithm::SHA256, data.as_bytes())
        // }
        
    }

pub fn current_time() -> String{
        let now: DateTime<Utc> = Utc::now();
        let mut time = String::new();
        time = now.format("%a %e %b %Y %T.%f").to_string();
        time
}

fn hash (data: &String) -> Hash {
            crypto_hash::digest(crypto_hash::Algorithm::SHA256, data.as_bytes())
        }
// pub fn vec_to_str(vector: )

pub fn vec_to_str(vector: &Vec<u8>) -> String{
    let mut st = String::new();
    st = vector.into_iter().map(|i| i.to_string()).collect::<String>();
    st
}

fn st(s: &String) -> String{
    s.to_string()
}

// fn hash (data: bytes) -> Hash {
//         crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
// }


// pub fn combine_string() -> String{

// }



fn main(){
    
}

// use std::io;
// trait BlockChain{
//     // fn new(&mut self)->Block;
//     fn genesis_block() -> Block;
//     fn block_instance(&mut self,id: u32,hash: String,prev_hash: String,data: String) -> Block;
//     fn mine_block(&self,block1: &Block);
//     fn input_data(&mut self);
//     fn output_data(&self);
//     fn output_block(block1: &Block);
//     // fn show_chain(self,blockchain: &[Block]);
//     fn show_chain(&self,blockchain: &[Block]);
    
//     // fn hash(&self,file: &String) -> String;
//     // fn 
// }
// struct Block{
//     // pub_key:    String,
//     // pri_key:    String,
//     id:         u32,
//     // timestamp:  u32,
//     hash:       String,
//     prev_hash:  String,
//     data:       String,
// }

// impl BlockChain for Block{
//     // #[test]
//     // fn new() -> Block{
//     //     Block{
//     //         id: 0u32,
//     //         hash: "none".to_string(),
//     //         prev_hash: "none".to_string(),
//     //         data: "none".to_string(),
//     //     }
//     // }

//     // fn new(&mut self) -> Block{
//     //     Block{
//     //         id:     
//     //     }
//     // }

//     fn genesis_block()->Block{
//         Block{
//             id: 0u32,
//             hash: "none".to_string(),
//             prev_hash: "none".to_string(),
//             data: "none".to_string(),
//         }
//     }

//     fn block_instance(&mut self,id1: u32,hash1: String,prev_hash1: String,data1: String) -> Block{
//         Block{
//             id: id1,
//             hash: hash1,
//             prev_hash: prev_hash1,
//             data: data1,   
//         }
//     }

//     fn mine_block(&self, block1: &Block){
//         // let blockchain: [Block; 0] = [];
//         let mut blockchain: Vec<&Block> = Vec::new();
//         blockchain.push(block1);

//     }
//     fn input_data(&mut self){
//         let mut id = String::new();
//         println!("input id: ");
//         io::stdin().read_line(&mut id).expect("Something went wrong!");
//         self.id = id.trim().parse().unwrap();
//         println!("input hash: ");
//         io::stdin().read_line(&mut self.hash).expect("Something went wrong!");
//         println!("input prev_hash: ");
//         io::stdin().read_line(&mut self.prev_hash).expect("Something went wrong!");
//         println!("input data: ");
//         io::stdin().read_line(&mut self.data).expect("Something went wrong");
//     }
//     fn output_data(&self){
//         println!("ID:           {}",&self.id);
//         println!("Prev_hash:    {}",&self.prev_hash);
//         println!("Hash:         {}",&self.hash);
//         println!("Data:         {}",&self.data);
//     }
//     fn output_block(block1: &Block){
//         println!("ID:           {}",block1.id);
//         println!("Prev_hash:    {}",block1.prev_hash);
//         println!("Hash:         {}",block1.hash);
//         println!("Data:         {}",block1.data);
//     }
//     // fn show_chain(&self,blockchain: &[Block]);
//     fn show_chain(&self,blockchain: &[Block]){
//         for block1 in blockchain{
//             Block::output_block(&block1);
//             println!("----------*----------");
//         }
//     }
// }

/////////////////////////////////////////////////////

// trait Function{
//     fn input(&mut self);
//     fn output(a: &A);
//     // fn output();
//     fn output_all(self,array: &[A]);
// }
// struct A{
//     a: i32,
//     b: i32,
// }
// impl Function for A{
//     fn input(&mut self){
//         // a.clear();
//         // b.clear();
//         let mut a = String::new();
//         let mut b = String::new();
//         println!("input a: ");
//         io::stdin().read_line(&mut a).expect("Something went wrong");
//         println!("input b: ");
//         io::stdin().read_line(&mut b).expect("Something went wrong");
//         self.a = a.trim().parse().expect("Converting Error");
//         self.b = b.trim().parse().expect("Converting Error");
//     }
//     fn output(ax: &A){
//         println!("A:    {}",ax.a);
//         println!("B:    {}",ax.b);
//     }
//     // fn output(){

//     // }
//     fn output_all(self,array: &[A]){
//         for value in array{
//             // println!("A:    {}",value.a);
//             // println!("B:    {}",value.b);
//             // item.output(&value);
//             A::output(&value);
//         }
//     }
// }
// extern crate crypto-hash;
// use super::*;

fn vector_loop(vector: &[String]){
    for value in vector{
        println!("{}",value);
    }
}

// fn hash (data: String) {
//         println!("{}",crypto_hash::digest(crypto_hash::Algorithm::SHA256, data.as_bytes()));
// }

// type Hash = Vec<u8>;
// fn hash (data: String) -> Hash {
//         crypto_hash::digest(crypto_hash::Algorithm::SHA256, data.as_bytes())
// }

//Standard library
// use std::io;

// extern crate lib;
// mod all_file;
// use std::time::SystemTime;
// use std::time::{SystemTime, UNIX_EPOCH};
extern crate chrono;
use chrono::{DateTime,Utc};
use std::time::{SystemTime,UNIX_EPOCH};



mod block;
use block::Block;
use block::BlockChain;
use block::current_time;
// use block::BlockChain::hash;
// use block1::BlockChain::block_instance;

// extern crate block1::Block;
fn main() {
        let mut block1 = Block::genesis_block();
        block1.output_data();
        // println!("----------*-----------");
    // let vector: Vec<String> = vec!["block1".to_string(),
    //                                "block2".to_string(),
    //                                "block3".to_string(),
    //                                "block4".to_string()];
    //     let mut vector: Vec<block1::Block> = Vec::new();
    //     let block0 = block1.block_instance(0u32,"hash0".to_string(),"prev_hash0".to_string(),"data0".to_string());
    //     let block1 = block1.block_instance(1u32,"hash1".to_string(),"prev_hash1".to_string(),"data1".to_string());
    //     let block2 = block1.block_instance(2u32,"hash2".to_string(),"prev_hash2".to_string(),"data2".to_string());
    //     let block3 = block1.block_instance(3u32,"hash3".to_string(),"prev_hash3".to_string(),"data3".to_string());
    //     vector.push(block0);
    //     vector.push(block1);
    //     vector.push(block2);
    //     vector.push(block3);
    // vector.push("block1".to_string());
    // vector.push("block2".to_string());
    // vector.push("block3".to_string());
    // vector.push("block4".to_string());
    // vector.push("block5".to_string());
    //     block1.show_chain(&vector);

    
    // let mut vector: Vec<String> = Vec::new();
    // vector.push("block1".to_string());
    // vector.push("block2".to_string());
    // vector.push("block3".to_string());
    // vector.push("block4".to_string());
    // vector_loop(&vector);



//import module from other file

    // let mut blocka = Block::genesis_block();
    // blocka.output_data();
    // let mut bloc = Block::block_instance{
    //     id: 1,
    //     timestamp: current_time(),
    //     hash: 
    // };
    
    // let mut vector: Vec<Block> = Vec::new();
        // let block0 = blocka.block_instance(0u32,current_time(),"hash0".to_string(),"prev_hash0".to_string(),"data0".to_string());
        // let block1 = blocka.block_instance(1u32,current_time(),"hash1".to_string(),"prev_hash1".to_string(),"data1".to_string());
        // let block2 = blocka.block_instance(2u32,current_time(),"hash2".to_string(),"prev_hash2".to_string(),"data2".to_string());
        // let block3 = blocka.block_instance(3u32,current_time(),"hash3".to_string(),"prev_hash3".to_string(),"data3".to_string());
    //     let block0 = blocka.block_instance(0u32,current_time(),Block::hash(blocka.combine_string()).into_iter().map(|i| i.to_string()).collect::<String>(),"prev_hash0".to_string(),"data0".to_string());
    //     let block1 = blocka.block_instance(1u32,current_time(),Block::hash(blocka.combine_string()).into_iter().map(|i| i.to_string()).collect::<String>(),"prev_hash1".to_string(),"data1".to_string());
    //     let block2 = blocka.block_instance(2u32,current_time(),Block::hash(blocka.combine_string()).into_iter().map(|i| i.to_string()).collect::<String>(),"prev_hash2".to_string(),"data2".to_string());
    //     let block3 = blocka.block_instance(3u32,current_time(),Block::hash(blocka.combine_string()).into_iter().map(|i| i.to_string()).collect::<String>(),"prev_hash3".to_string(),"data3".to_string());
    //     vector.push(block0);
    //     vector.push(block1);
    //     vector.push(block2);
    //     vector.push(block3);
    // blocka.show_chain(&vector);

    // let mut st = "152212172301891686263161122041781561751991331871092974711731091361745212291171875".to_string();
    // println!("{}",st.len());

//Testing combine_string() function

    // let mut block = Block::genesis_block();
    // println!("combine string function: {}",block.combine_string());
    // let mut st = block.combine_string();
    // let mut hash1 = Block::hash(st);
    // println!("{:?}",hash1);

    // println!("");
    // block = Block::genesis_block();
    // println!("combine string function: {}",block.combine_string());
    // st = block.combine_string();
    // hash1 = Block::hash(st);
    // println!("{:?}",hash1);


    // let mut hash_vec = Block::hash("love".to_string());
    // let mut st = String::new();
    // st = hash_vec.into_iter().map(|i| i.to_string()).collect::<String>();
    // println!("Hash's result: {}",st);
    // println!("{:?}",st.as_bytes());


    // println!("Hash's result : {:?}",hash("love".to_string()));
    // let mut hash_vec = BlockChain::hash("love".to_string());
    

    // let mut hash_string = String::new();
    // hash_string = hash_vec.iter().collect().unwarp();
    // hash_string = hash_vec.iter().collect::<&u8>().to_string();
    
    // for value in &hash_vec{
    //     // println!("{}",value);
    //     st = st + &value.to_string();
    // }
    // println!("hash_string = {}",st);

    // st = hash_vec.iter().collect();
    
    // let value: u8 = 10;
    // let mut string = String::new();
    // string = value.to_string();
    // let time = SystemTime::now();

    // let time = SystemTime::now();
    // let time1 = time.duration_since(UNIX_EPOCH)
    //     .expect("something went wrong");
    // println!("Time: {:?}",time);
    // println!("Time1: {:?}",time1);
    

    // println!("UTC now is: {}", now);
    // println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    // println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    // println!("UTC now in a custom format is: {}", now.format("%a %b %e %T %Y"));
    // println!("UTC now in a custom format is: {}", now.format("%a %e %b %Y %T"));
    
    
    // time = now.format("%a %e %b %Y %T").parse::<String>().unwrap();


//WORK for get time
    // let now: DateTime<Utc> = Utc::now();
    // let mut time = String::new();
    // time = now.format("%a %e %b %Y %T .%f").to_string();
    // println!("{}",time);

    // println!("{}",now.format("%a"));
    // println!("{}",now.format("%b"));
    // println!("{}",now.format("%e"));
    // println!("{}",now.format("%T"));
    // println!("{}",now.format("%Y"));

    // let timestamp = "1524820690".parse::<i64>().unwrap();
    // println!("{}",timestamp);


    // let duration = SystemTime::now()
    //         .duration_since(UNIX_EPOCH)
    //         .unwrap();
    //     duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128;
    // println!("Duration: {}",duration.to_string());

    // let mut string = String::new();
    // string =  duration.to_string();
    // println!("Duration: {}",string);
    
    // let value = 10.0;
    // let mut string = String::new();
    // string = value.to_string();
    // println!("{}",string);

}

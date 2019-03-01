type Hash = Vec<u8>;
pub struct Block{
    prev_hash: Hash,
}
impl Block{
    fn new(prev_hash: Hash) -> Self{
        Block{
            prev_hash,
        }
    }
    fn output(&self){
        println!("Hash: {:?}",&self.prev_hash);
    }
}






struct MyBox<T>(T);
impl <T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

use std::ops::Deref;
impl <T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &T{
        &self.0
    }
}

pub fn vec_to_str(vector: &Vec<u8>) -> String{
    let mut st = String::new();
    st = vector.into_iter().map(|i| i.to_string()).collect::<String>();
    st
}
// pub fn vec_to_str1<V: std::iter::Iterator + std::string::ToString>(vector: V) -> String{
//     let mut st = String::new();
//     st = vector.into_iter().map(|i| i.to_string()).collect::<String>();
//     st
// }
// V :: std::iter::Iterator + std::string::ToString;
    // pub fn vec_to_str1<V: /*IntoIterator + */ToString>(vector: V) -> String
    // pub fn vec_to_str1<V: IntoIterator>(vector: V) -> String
    
    
    // pub fn vec_to_str1<V>(vector: &V) -> String
    //     where V: IntoIterator<Item = String>
    //     {
    //     let mut st = String::new();
    //     st = vector.into_iter().map(|i| i.to_string()).collect::<String>();
    //     st
    // }




// pub fn toString<T: ToString>(value: T) -> String{
//     value.to_string()
// }


fn main(){
    // let mut st = "152212172301891686263161122041781561751991331871092974711731091361745212291171875".to_string();
    // println!("{}",st.len());

    // let vector = vec![0; 32];
    // println!("{:?}",vector);
    // let mut st = String::new();
    // st = vector.into_iter().map(|i| i.to_string()).collect::<String>();
    // println!("{}",st);

    //Test Vector [0;32] -> Result
        // let mut vector = vec![1u8;32];
        // let mut block1 = Block::new(vector);
        // block1.output();

    //Test convert vec_to_str() function
        // let vector = vec![1;32];
        // let vector = vec![1;32];
        // let mut st = vec_to_str(&vector);
        // println!("String of vector:  {}",st);
        
        // let vector1 = vec![1.1;32];
        // let mut st1 = String::new();
        // st1 = vec_to_str1(&vector1);
        // println!("String of vecto1: {}",st1);
    //Test plus string
        let mut s1 = String::from("Hello ");
        // let mut s2 = String::from("World");
        // let mut s2 = "World";
        let mut s2 = String::from("World");
        let mut s = String::new();
        let mut s3 = st(&s1);
        s = s1 + &s2;
        println!("s = {}",s);   
        println!("s1 = {}",s3); 
        
        // let mut vector = MyBox::new(vector1);
        
        // println!("Vec<u8> : {:?}",vector);
        //------> use generic type to a function that can accepts all vector type 
}
fn st(s: &String) -> String{
    s.to_string()
}

// use std::thread;
// use std::time::Duration;


// struct Cacher<T>
// where T:Fn(i32)->i32
// {
//     calculation:T,
//     value:Option<i32>,

// }
// impl<T> Cacher<T>
// where T:Fn(i32)->i32 {
//     fn new(calculation:T)->Cacher<T>{
//         Cacher{
//             calculation,
//             value:None,
//         }
//     }

//     fn value(&mut self,arg:i32)->i32 {
//         match self.value {
//             Some(v)=> v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }

// fn main() {

//     fn an_other_func(fs_parameter:i32, sc_parameter:i32){
//         let mut result = Cacher::new(|num|{
//             println!("messsage from any_function");
//             thread::sleep(Duration::from_secs(2));
//             num
//         });
//         if fs_parameter < 30 {
//             println!("your value is {}",result.value(fs_parameter));
//             println!("your next Values is {}",result.value(fs_parameter));
//         }
//         else{
//             if sc_parameter > fs_parameter {
//                 println!("value is : {}", sc_parameter);
//             }
//             else {
//                 println!("Value is : {}", result.value(fs_parameter));
//             }
//         }
//         }

//         an_other_func(35, 45);
//         let  mut x=0;

//         let mut name = || {
//             if x==0 {
//                 x = 2;
//             }
//             else {
//             println!("printing Somthing {}",x);
//             }
//         };

//         name();
        
// }


use std::thread;
use std::time::Duration;

fn main(){
    
    let handle = thread::spawn(|| {for i in 1..10 {
        println!("first for loop {}",i);
        thread::sleep(Duration::from_millis(1));
    }
    });


    for i in 1..5 {
        println!("second for loop {}",i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();


}


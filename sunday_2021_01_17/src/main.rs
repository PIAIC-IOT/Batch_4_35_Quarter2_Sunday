//  2 = 10
//  3 = 11
//  4 = 100
//  7 = 111
// 2^8                           2^2 2^1 2^0
// 256 128 64  32   16    8    4   2   1           255
//     0   0   0    0     0    0   0   0
//                         0   1   0   0
//                             1   1
//                         1   0   0 
//                         1   1   1     
//                  1      0   1   1   1           23
//     1   0   1   1       0   1   1   1           183         


// OR AND NOT XOR

// 1  OR  1 = 1
// 1  OR  0 = 1
// 0  OR  1 = 1
// 0  or  0 = 0


// 2 OR 3 = 3

// 10 OR 11 =  11 = 3 

// 10 OR 11 = 11 

// 10 OR 12 =  14 

// 1010 OR 1100 = 1110 



// AND
// 1  AND  1 = 1
// 1  AND  0 = 0
// 0  AND  1 = 0
// 0  AND  0 = 0


// 8 AND 4 = 0
//     8421
//     1000 
// AND 0100 
//   = 0000



//   XOR

//   0 XOR 0 XOR 0=  0
//   0 XOR 1 XOR 1=  0
//   1 XOR 0 XOR 1=  0
//   1 XOR 1 XOR 1=  1



//   4 XOR 3 = 7
//        100 
//    XOR 011 
//      = 111


// fn main() {
//     let a:u32 = 3;
//     let b:u32 = 4;
//     let mut result:u32;
//     result = a|b;
//     println!("a or b is {}", result);
//     //0011
//     //0100
//     //0111 = 7
//     result = a&b;
//     println!("a and b is {}", result);
//     result = a^b;
//     println!("a xor b is {}", result);
//     result = !b;
//     println!("not b is {}", result);
//     result = a>>b;
//     println!("a>> b is {}", result);
//     result = a<<b;
//     println!("a << b is {}", result);
//     }

// 32 16 8 4 2 1
// 1  0  1 1 0 1  45 

// 0  1  1 0 1 0  26
// 0  1  0 1 1 0  22

// 1  1  1 1 1 1  63

// 0  1  1 1 1 1  31



///// chapter 19.1 unsafe rust ////////////
fn main(){
    let mut num  = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    unsafe{
    println!("r1 is {}", *r1);
    println!("r1 is {}", *r2);
    }

    unsafe fn dangerous(){}

    unsafe {
        dangerous();
    }

    static mut counter: u32=0;
        fn add_to (a:u32){
            unsafe{
            counter += a;
            }
        }

        add_to(2);
        unsafe{
        println!("Counter : {} ",counter)
        }
}



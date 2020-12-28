fn main(){
    fn name <'a>(y: &'a str, x:&'a str) -> &'a str{
        if x.len()> y.len() {
            x
        }
        else {
            y
        }
        // let result = String::from("String is too long");
        // result.as_str()
    }


    //     let y= "ejaz";    

    //     { 
    //     let x="hanzala";
    //     let output = name( y, x);
    //     }

    struct struct_name <'a>{
        name:& 'a str
    }
    let name = String::from("Orya");    
    let new_name = struct_name{
    name : &name,
    };

    impl<'a> struct_name<'a> {
        fn full_name(&self, last_name: &str) -> String {
        format!{"{} and {}", self.name, last_name}  
        }
        }


        use std::fmt::Display;
        fn longest_with_an_announcement<'a, T,D>(
            x: &'a str,
            y: &'a str,
            ann: T,
            att:D,
            ) -> &'a str
        where
            T:Display,
            D:Display,
            {
                 println!("Announcement! {}", ann);
                if x.len() > y.len() {
                     x
                } else {
                     y
                }
}

// struct Iotstudent {
//     name: String,
//     age: i32,
//     education: String,
// }

// struct IotInstructor {
//     name: String,
//     age: i32,
// }

// let student = Iotstudent {
//     name: String::from("Shaukat"),
//     age: 33,
//     education: String::from("“Electrical Engineer”"),
// };

// let student1 = Iotstudent {
//     name: String::from("Ali"),
//     age: 33,
//     education: String::from("“Electrical Engineer”"),
// };

// let teacher = IotInstructor {
//     name: String::from("“Sir”"),
//     age: 26,
// };
    
//    println!("{}",teacher.ask_Question(student.name));
//    println!("{}",student1.ask_Question(student1.name)));


}



    


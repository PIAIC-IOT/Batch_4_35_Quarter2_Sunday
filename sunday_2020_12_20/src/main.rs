
fn main() {
    // fn func_name<T> (x:T)->T{
    //     x
    // }
    // println!("{}",func_name(5.2));

    // struct koi_name<T> {
    //     name:T,
    //     age:T,
    // }

    // let first_str= koi_name {
    //     name:"Hanzala",
    //     age:"21",
    // };

    // println!("{}", first_str.name);
    // enum name<T>{
    //     ok(T),
    //     not_ok(T),
    // }
         trait name_trait{
            fn some_function(&self) -> String;

            fn func_complete_data(&self)->String{
                format! ("Student name and class is : {}", self.some_function())
            }
            }
         struct name_struct {
             name:String,
             class:String,
        }

        impl name_trait for name_struct {
            fn some_function(&self) -> String {
                format!("{} \n class : {} ",self.name, self.class)
            }
        }

        struct second_struct {
            name:String,
            class:String,
            batch:String,
       }

       impl name_trait for second_struct {
        fn some_function(&self) -> String {
            format!("{} \n class : {} Batch {} ",self.name, self.class, self.batch)
        }

        }
        let name_of_sruct = name_struct {
            name:String::from("saifullah"),
            class:String::from("Sunday"),
        };

        println!("{}",name_of_sruct.func_complete_data());


        let second_instance = second_struct {
            name:String::from("saifullah"),
            class:String::from("Sunday"),
            batch:String::from("Batch 4 to 35"),
        };

        println!("{}",second_instance.func_complete_data());





}

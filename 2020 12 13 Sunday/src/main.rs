fn main() {
    fn function_name(parameter:i32)-> i32 {
        parameter
    }
    let output_1= function_name(56);
       println!("{}", output_1);

    fn function_name_f32(parameter_f32:f32)-> f32 {
        parameter_f32
    }

    let output_2= function_name_f32(50.0);
       print!("{}", output_2);

        let a=vec![20,2,3,5,4];
        let b=vec![2,1,3,2,1];


       fn func<F:std::cmp::PartialOrd>(param:&[F]) ->&F{
           let mut largest=&param[0];
           for num in param {
               if num > largest  {
                   largest = num
               }
           }
           largest
       }
        
       let output_3 = func(&[a]);
       
       struct name_str<T> {
           x:T,
           y:T
       }

    impl<T,Y> name_str<T,Y>{

        fn func_struct (&self) -> &T {
            if (&self.x == 0){
                &self.y
            }
            else{
                &self.x
            }

        }

    }

    let struct_1 = name_str {
        x:5,
        y:7
    };
   let x= struct_1.func_struct();

    //  enum first_enum<T> {
    //     some,
    //     none
    //  }

     enum first_enum <T,Y>{
        some(T),
        another(Y)
    }

    enum first_enum_i32 {
        some(i32),
        another(i32)
    }

    enum first_enum_f32 {
        some(f32),
        another(f32)
    }

        

       

}

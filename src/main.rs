fn main() {
    // let mut num:u8 = 255;
    // println!("The value stored in num is {}",num);
    // num = 100;
    // println!("The value stored in num is {}",num)

    //String ---> Dynamic length Strings - Heap allocated
    //&str --> Fixed length Strings - Special memory
    let mut string_literal:String = String::from("Hii, Rajshri");
    string_literal.push_str(" Wassup...");
    print!("This is string literal {}", string_literal);

}


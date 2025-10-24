fn main() {
    // let mut num:u8 = 255;
    // println!("The value stored in num is {}",num);
    // num = 100;
    // println!("The value stored in num is {}",num)

    //String ---> Dynamic length Strings - Heap allocated
    //&str --> Fixed length Strings - Special memory
    // let mut string_literal:String = String::from("Hii, Rajshri");
    // string_literal.push_str(" Wassup...");
    // print!("This is string literal {}", string_literal);



    //Tuple
    let emp_info:(&str, u8) = ("Raj", 26);
    let emp_name:&str = emp_info.0;
    let emp_age:u8 = emp_info.1;
    println!("Name : {} and Age : {}",emp_name, emp_age);
    //destructuring
    let (employee_n, employee_a) = emp_info;
    println!("Employee Name : {} and Employee Age : {}", employee_n, employee_a);
    
}


use std::env;


#[allow(dead_code)]
fn print_typename<T>(_: T) { 
    println!("{}", std::any::type_name::<T>());
}

fn printstr(val: &str) {
    println!("{}", val);
}

fn printstatic_str_wrapper(_val: &str) {
    // printstatic_str(val); // staticがないから渡せない
}


fn printstatic_str(val: &'static str) {
    println!("{}", val);
}



fn hello_string(x: &str) -> String {
    if x.len() > 0 {
        String::from("hello world")
    }
    else { 
        "".to_string()
    }
}

fn hello_string_strliteral(x: &str) -> &str {
    // this is ok because  thease return values are literal (consistent static)
    if x.len() > 0 {
        "hello world"
    }
    else { 
        ""
    }
}

fn hello_string_str(x: &str) -> &str {
    // this also ok. may be caller already knowns `x`'s lifetime.
    if x.len() > 0 {
        x
    }
    else { 
        ""
    }
}


fn hello_string_str_ref(x: &str) -> &str {
    let s = "sample";
    let upper_string = s.to_uppercase();
    let upper_str = upper_string.as_str();
    print_typename(&upper_string);
    print_typename(upper_str);
    // print_typename(upper_string);  // move
    // print_typename(upper_str);     // error because uppser_string moved out.

    if x.len() > 0 {
        x
    }
    else { 
        ""
    }
}


fn unstatic(val: &'static str) -> String {
    String::from(val)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg0 = &args[0];

    let variable1 : &str = "string to be stored";
    let variable2 = "string inferred type";


    let x = hello_string("a");
    println!("{}", x);
    let x = hello_string_strliteral("a");
    println!("{}", x);

    let x = hello_string_str("a");
    println!("{}", x);
    let x = hello_string_str_ref("a");
    println!("{}", x);
    


    print_typename(variable1); // &str
    print_typename(variable2); // &str

    printstr(variable1);
    printstatic_str(variable1);  // staticと認められるのはリテラルだから。
    printstr(arg0);

    printstatic_str_wrapper(variable1);
    // printstatic_str(arg0); // リテラルでないので staticとは確定されない。

    let takeasstr = |v:&str| -> String { String::from(v) };
    let _var3 = takeasstr(variable1);

    printstatic_str("this is a literal");



    let strbase = String::from("this is a literal");

    
    let xxx = strbase.as_str();
    printstatic_str(xxx);




    println!("Hello");

}
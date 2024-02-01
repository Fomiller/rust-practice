fn main() {
    let a = "string a".to_string();
    let b = Some("string b".to_string());
    let c = Some("string c".to_string());
    let my_struct = MyStruct { a, b, c };
    println!("{:?}", my_struct);
    // let x = my_struct.b.take();
    // println!("{:?}", x);
    println!("{:?}", my_struct);
    let my_option = MyOption {
        x: my_struct.b.as_ref().and_then(|s| Some(s.to_string())),
    };
    println!("{:?}", my_option);
    println!("{:?}", my_struct);
}

#[derive(Debug)]
struct MyStruct {
    #[allow(dead_code)]
    a: String,
    b: Option<String>,
    #[allow(dead_code)]
    c: Option<String>,
}

#[derive(Debug)]
struct MyOption {
    #[allow(dead_code)]
    x: Option<String>,
}

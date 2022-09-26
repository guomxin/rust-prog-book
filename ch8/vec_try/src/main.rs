fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    if let Some(n) = third {
        println!("The third element is {}", n)
    }

    /*
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
     */
    
    for i in &v {
        println!("{}", i)
    }

    
}

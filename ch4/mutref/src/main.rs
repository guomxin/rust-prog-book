fn main() {
    let mut s = String::from("hello");
    let s1 = &s;
    println!("{}", s1); // 可在这
    change(&mut s);
    //println!("{}", s1); // 不可以在这，上面&mut s与s1有交叉生命周期
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

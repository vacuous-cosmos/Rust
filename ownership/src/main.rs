fn main() {
    let mut s=String::from("Hello");
    s.push_str(",world!");
    println!("{}",s);
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    let mut s = String::from("hello");
    change(&mut s);
    //no more than 1 mutable reference at a time

}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

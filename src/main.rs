fn main() {
    let string = String::from("Test");
    let reference = &string;
    let mut vec_string = vec![String::from("Test3")];
    vec_string.push(String::from("Salam les rhoya"));

    println!("{} {}2 {:?}", string, reference, vec_string);
}

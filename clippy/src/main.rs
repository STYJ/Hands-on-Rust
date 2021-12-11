fn main() {
    let my_list = ["one", "two", "three"];
    for i in &my_list {
        println!("{}", i);
    }
}

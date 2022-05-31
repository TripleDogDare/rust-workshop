use example::Anything;

fn main() {
    let anything = Anything {
        name: "anything".to_string(),
        something_else: 3.1423723728378237,
    };

    println!("{}", anything.to_string());
}

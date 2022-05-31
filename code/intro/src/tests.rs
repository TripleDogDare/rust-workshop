use crate::Anything;

#[test]
fn it_works() {
    let anything = Anything {
        name: "anything".to_string(),
        something_else: 1.,
    };

    println!("{}", anything.name);
}

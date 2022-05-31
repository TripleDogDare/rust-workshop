fn snippet() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
}

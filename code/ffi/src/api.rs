pub fn example1(x: u32) -> u32 {
    x + 1
}

pub fn example2_1(x: u32) -> Vec<u32> {
    vec![x]
}

pub fn example2_2(x: &[u32]) -> u32 {
    x[0]
}

pub struct Example4 {
    pub x: Vec<u32>,
}

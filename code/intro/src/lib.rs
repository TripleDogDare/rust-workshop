mod another;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Anything {
    pub name: String,
    pub something_else: f64,
}

// impl std::fmt::Display for Anything {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//     "The value of Anything includes the name: {}, and something_else: {}",
//     self.name, self.something_else
// )
//     }
// }

impl ToString for Anything {
    fn to_string(&self) -> String {
        format!(
            "The value of Anything includes the name: {}, and something_else: {}",
            self.name, self.something_else
        )
    }
}

use std::fmt;

#[derive(Debug)]
pub struct Check {
    expression: String
}

impl Check {
    pub fn new(constraint: &str) -> Self {
        Self {
            expression: constraint.to_owned()
        }
    }
}

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " CHECK({}) ", self.expression)?;
        Ok(())
    }
}
use std::fmt;

const INT: &str = "INTEGER";
const REAL: &str = "REAL";
const TEXT: &str = "TEXT";
const BLOB: &str = "BLOB";

#[derive(Debug, Clone, Copy)]
pub enum GeneratedColumnType {
    Virtual,
    Stored
}

impl fmt::Display for GeneratedColumnType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Stored => write!(f, "STORED")?,
            Self::Virtual => write!(f, "VIRTUAL")?
        }
        Ok(())
    }
    
}

#[derive(Debug)]
struct Generated {
    expr: String,
    type_: GeneratedColumnType
}

impl Generated {
    fn new(expr: &str, typ: GeneratedColumnType) -> Self {
        Self { expr: expr.to_owned(), type_: typ }
    }
}

#[derive(Debug)]
pub struct Column {
    name: String,
    typ_: String,
    not_null: bool,
    unique: bool,
    primary_key: bool,
    check: Option<String>,
    default_val: Option<String>,
    generated: Option<Generated> 
}

impl Column {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            typ_: INT.to_owned(),
            not_null: true,
            unique: false,
            primary_key: false,
            check: None,
            default_val: None,
            generated: None
        }
    }

    pub fn int(mut self) -> Self {
        self.typ_ = INT.to_string();
        self
    }

    pub fn real(mut self) -> Self {
        self.typ_ = REAL.to_string();
        self
    }

    pub fn text(mut self) -> Self {
        self.typ_ = TEXT.to_string();
        self
    }

    pub fn blob(mut self) -> Self {
        self.typ_ = BLOB.to_string();
        self
    }

    pub fn nullable(mut self) -> Self {        
        self.not_null = false;
        self.primary_key = false;
        self.unique = false;
        self
    }

    pub fn unique(mut self) -> Self {       
        self.not_null = true;       
        self.unique = true;
        self
    }

    pub fn primary_key(mut self) -> Self {
        self.not_null = true;
        self.primary_key = true;
        self
    }

    pub fn check(mut self, constraint: &str) -> Self {
        self.check = Some(constraint.to_string());
        self
    }

    pub fn default_value(mut self, value: &str) -> Self {
        self.default_val = Some(value.to_string());
        self
    }

    ///
    /// Generated column cannot be or be part of Primary Key and cannot have a default value.
    /// Additionally, only virtual columns can be added by ALTER TABLE ADD COLUMN
    pub fn generated(mut self, expr: &str, typ: GeneratedColumnType) -> Self {
        self.generated = Some(Generated::new(expr, typ));
        self.default_val = None;
        self.primary_key = false;
        self
    }

    pub fn is_generated(&self) -> bool {
        match self.generated {
            Some(_) => true,
            None => false
        }
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.name, self.typ_)?;
        if self.not_null {
            write!(f, " NOT NULL")?;
        }
        if self.unique {
            write!(f, " UNIQUE")?;
        }
        if self.primary_key {
            write!(f, " PRIMARY KEY")?;
        }
        if let Some(ref v) = self.default_val {
            write!(f, " DEFAULT {}", v)?;
        }
        if let Some(ref c) = self.check {
            write!(f, " CHECK({})", c)?;
        }
        if let Some(ref g) = self.generated {
            write!(f, " AS ({}) {}", g.expr, g.type_)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Column, GeneratedColumnType};

    #[test]
    fn primary_key() {
        let col = Column::new("Id").primary_key();
        assert_eq!(col.to_string(), "Id INTEGER NOT NULL PRIMARY KEY");
    }

    #[test]
    fn nullable() {
        let col = Column::new("Nullable").nullable();
        assert_eq!(col.to_string(), "Nullable INTEGER");
    }

    #[test]
    fn unique() {
        let col = Column::new("UniqueCol").unique();
        assert_eq!(col.to_string(), "UniqueCol INTEGER NOT NULL UNIQUE");
    }

    #[test]
    fn check() {
        let col = Column::new("Col").check("Col > 10");
        assert_eq!(col.to_string(), "Col INTEGER NOT NULL CHECK(Col > 10)");
    }

    #[test]
    fn derfault_value() {
        let col = Column::new("Col").default_value("21");
        assert_eq!(col.to_string(), "Col INTEGER NOT NULL DEFAULT 21");
    }

    #[test]
    fn generated() {
        let col1 = Column::new("Generated_1").primary_key().generated("x + y + 1", GeneratedColumnType::Virtual);
        let col2 = Column::new("Generated_2").default_value("10").generated("x + y + 1", GeneratedColumnType::Stored);
        assert_eq!(col1.primary_key, false);
        assert_eq!(col1.default_val, None);
        assert_eq!(col1.to_string(), "Generated_1 INTEGER NOT NULL AS (x + y + 1) VIRTUAL");
        assert_eq!(col2.primary_key, false);
        assert_eq!(col2.default_val, None);
        assert_eq!(col2.to_string(), "Generated_2 INTEGER NOT NULL AS (x + y + 1) STORED");
    }

    #[test]
    fn is_generated() {
        let col1 = Column::new("col1").generated("1 + x", GeneratedColumnType::Stored);
        let col2 = Column::new("col2");

        assert_eq!(col1.is_generated(), true);
        assert_eq!(col2.is_generated(), false);
    }
}
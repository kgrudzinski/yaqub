use std::fmt;

#[derive(Default)]
struct RawInsert {
    table: String,
    columns: String,
    values: String
}

impl fmt::Display for RawInsert {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "INSERT INTO {}({}) VALUES({})", self.table, self.columns, self.values)
    }
}

pub struct Insert(RawInsert);
pub struct InsertInto(RawInsert);
pub struct FinalInsert(RawInsert);


impl Insert {
    pub fn into(mut self, table: &str) -> InsertInto {
        self.0.table = table.to_string();
        InsertInto(self.0)
    }
}

impl InsertInto {
    pub fn values(mut self, values: &[&str]) -> FinalInsert {
        self.0.values = values.join(", ");
        FinalInsert(self.0)
    }
}

impl fmt::Display for FinalInsert {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn insert(columns: &[&str]) -> Insert {
    Insert(RawInsert {
        columns: columns.join(", "),
        ..Default::default()
    })
}

#[cfg(test)]
mod test {
    use super::insert;

    #[test]
    fn insert_item() {
        let sql = insert(&["login", "email", "passwd"]).
        into("users").
        values(&["'Winnie the pooh'", "'pooh@hundredacreforest.org'", "'honey!!!'"]).
        to_string();
        
        assert_eq!(sql, "INSERT INTO users(login, email, passwd) VALUES('Winnie the pooh', 'pooh@hundredacreforest.org', 'honey!!!')");
    }
}
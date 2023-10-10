use std::fmt;

#[derive(Default)]
struct RawUpdate {
    table: String,
    data: String,
    where_: String
}

impl fmt::Display for RawUpdate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UPDATE {} SET {}", self.table, self.data)?;
        if self.where_.len() > 0 {
            write!(f," WHERE {}", self.where_)?;
        }
        Ok(())
    }
}

pub struct Update(RawUpdate);
pub struct UpdateSet(RawUpdate);
pub struct UpdateWhere(RawUpdate);



impl Update {
    pub fn set(mut self, data: &[(&str, &str)]) -> UpdateSet {
        self.0.data = data.iter().map(|it| format!("{} = {}",it.0, it.1)).collect::<Vec<String>>().join(", ");
        UpdateSet(self.0)
    }
}

impl UpdateSet {
    pub fn where_(mut self, cond: &str) -> UpdateWhere {
        self.0.where_ = cond.to_string();
        UpdateWhere(self.0)
    }
}

impl fmt::Display for UpdateSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl UpdateWhere {
    pub fn and(mut self, cond: &str) -> Self {
        self.0.where_.push_str(&format!(" AND {}", cond));
        self
    }

    pub fn or(mut self, cond: &str) -> Self {
        self.0.where_.push_str(&format!(" OR {}", cond));
        self
    }
}

impl fmt::Display for UpdateWhere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn update(table: &str) -> Update {
    Update(RawUpdate{
        table: table.to_string(),
        ..Default::default()
    })
}
#[cfg(test)]
mod test {
    use super::update;

    #[test]
    fn update_item() {
        let sql = update("users").                
        set(&[("login","'a'"), ("email", "'a@a'"), ("passwd", "'c'")]).
        where_("userId = 4").
        to_string();        
        assert_eq!(sql, "UPDATE users SET login = 'a', email = 'a@a', passwd = 'c' WHERE userId = 4");
    }

    #[test]
    fn update_item_where() {
        let sql = update("transactions").        
        set(&[("active", "0")]).
        where_("amount > 100").
        and("amount < 1000").
        or("customerId = 7").
        to_string();
        
        assert_eq!(sql, "UPDATE transactions SET active = 0 WHERE amount > 100 AND amount < 1000 OR customerId = 7");
    }
}
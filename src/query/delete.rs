use std::fmt;

#[derive(Default)]
struct RawDelete {
    table: String,
    where_: String
}

impl fmt::Display for RawDelete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DELETE FROM {}", self.table)?;
        if !self.where_.is_empty() {
            write!(f," WHERE {}", self.where_)?;
        }
        Ok(())
    }
}

pub struct DeleteFrom(RawDelete);
pub struct DeleteWhere(RawDelete);

impl DeleteFrom {
    pub fn where_(mut self, cond: &str) -> DeleteWhere {
        self.0.where_ = cond.to_string();
        DeleteWhere(self.0)
    }
}

impl fmt::Display for DeleteFrom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl DeleteWhere {
    pub fn and(mut self, cond: &str) -> Self {
        self.0.where_.push_str(&format!(" AND {}", cond));
        self
    }

    pub fn or(mut self, cond: &str) -> Self {
        self.0.where_.push_str(&format!(" OR {}", cond));
        self
    }
}

impl fmt::Display for DeleteWhere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn delete_from(table: &str) -> DeleteFrom {
    let delete = RawDelete{
        table: table.to_string(),
        ..Default::default()
    };

    DeleteFrom(delete)
}

#[cfg(test)]
mod test {
    use super::delete_from;

    #[test]
    fn delete_all() {
        let sql = delete_from("users").to_string();
        assert_eq!(sql, "DELETE FROM users");
    }

    #[test]
    fn delete_where() {
        let sql = delete_from("users").
        where_("userId = 4").
        to_string();
        assert_eq!(sql, "DELETE FROM users WHERE userId = 4");
    }

    #[test]
    fn delete_where_multi() {
        let sql = delete_from("users").
        where_("age < 16").
        and("age > 65").
        or("userId = 77").
        to_string();
        assert_eq!(sql, "DELETE FROM users WHERE age < 16 AND age > 65 OR userId = 77");
    }

    #[test]
    fn delete_with_subquery() {
        use crate::query::select;

        let sql = delete_from("comments").
        where_(&format!("userId IN ({})", select(&["userId"]).from("users").where_("banned = TRUE"))).
        to_string();

        assert_eq!(sql, "DELETE FROM comments WHERE userId IN (SELECT userId FROM users WHERE banned = TRUE)");
    }
}
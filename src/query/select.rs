
use std::fmt;

#[derive(Debug, Clone, Copy)]
enum SortOrder {
    Asc,
    Desc
}

impl fmt::Display for SortOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Asc => write!(f, "ASC"),
            Self::Desc => write!(f, "DESC")
        }
    }
}

#[derive(Debug, Clone)]
struct SortBy(String, SortOrder);

impl fmt::Display for SortBy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

#[derive(Debug,Clone)]
struct RawQuery {
    select: String,
    from: String,
    sort_by: Vec<SortBy>,    
    group_by: String,
    having: String,
    where_: String,
    distinct: bool,
    limit: Option<String>,
    offset: Option<String>
}

#[derive(Debug,Clone)]
pub struct SelectQuery(RawQuery);

#[derive(Debug,Clone)]
pub struct OrderByQuery(RawQuery);

#[derive(Debug,Clone)]
pub struct WhereQuery(RawQuery);

#[derive(Debug,Clone)]
pub struct GroupQuery(RawQuery);

#[derive(Debug,Clone)]
pub struct HavingQuery(RawQuery);

#[derive(Debug,Clone)]
pub struct FinalQuery(RawQuery);

impl RawQuery {
    fn new() -> Self {
        Self {
            select: String::new(),
            from: String::new(),
            sort_by: Vec::new(),
            group_by: String::new(),
            having: String::new(),
            where_: String::new(),
            distinct: false,
            limit: None,
            offset: None
        }
    }    
}

impl SelectQuery {
    pub fn from(mut self, table: &str) -> Self {
        self.0.from = table.into();
        self
    }

    pub fn order_by(mut self, field: &str) -> OrderByQuery {      
        self.0.sort_by.push(SortBy(field.into(), SortOrder::Asc));
        OrderByQuery(self.0)        
    }

    pub fn limit(mut self, value: Option<u32>) -> Self {
        self.0.limit = if let Some(v) = value { Some(v.to_string()) } else { Some("?".to_string()) };
        self
    }

    pub fn distinct(mut self) -> Self {
        self.0.distinct = true;
        self
    }

    pub fn offset(mut self, value: Option<u32>) -> Self {
        self.0.offset = if let Some(v) = value { Some(v.to_string()) } else { Some("?".to_string()) };
        self
    }

    pub fn where_(mut self, cond: &str) -> WhereQuery {
        self.0.where_ = cond.into();
        WhereQuery(self.0)
    }

    pub fn group_by(mut self, fields: &[&str]) -> GroupQuery {
        self.0.group_by = fields.join(", ");
        GroupQuery(self.0)
    }
}

impl WhereQuery {
    pub fn and(mut self, cond: &str) -> Self {
        self.0.where_.push_str(&format!(" AND {}", cond));
        self
    }

    pub fn or(mut self, cond: &str) -> Self {
        self.0.where_.push_str(&format!(" OR {}", cond));
        self
    }

    pub fn order_by(mut self, field: &str) -> OrderByQuery {
        self.0.sort_by.push(SortBy(field.into(), SortOrder::Asc));
        OrderByQuery(self.0)
    }

    pub fn group_by(mut self, fields: &[&str]) -> GroupQuery {
        self.0.group_by = fields.join(", ");
        GroupQuery(self.0)
    }
}

impl OrderByQuery {
    pub fn asc(mut self) -> OrderByQuery {
        if let Some(value) = self.0.sort_by.last_mut() {
            value.1 = SortOrder::Asc;
        } 
        self
    }

    pub fn desc(mut self) -> OrderByQuery {
        if let Some(value) = self.0.sort_by.last_mut() {
            value.1 = SortOrder::Desc;
        } 
        self
    }

    pub fn order_by(mut self, field: &str) -> Self {
        self.0.sort_by.push(SortBy(field.into(), SortOrder::Asc));        
        self
    }    
}

impl GroupQuery {
    pub fn order_by(mut self, field: &str) -> OrderByQuery {
        self.0.sort_by.push(SortBy(field.into(), SortOrder::Asc));
        OrderByQuery(self.0)
    }

    pub fn having(mut self, cond: &str) -> HavingQuery {
        self.0.having = cond.into();
        HavingQuery(self.0)
    }
}

impl HavingQuery {
    pub fn and(mut self, cond: &str) -> Self {
        self.0.having.push_str(&format!(" AND {}", cond));
        self
    }

    pub fn or(mut self, cond: &str) -> Self {
        self.0.having.push_str(&format!(" OR {}", cond));
        self
    }

    pub fn order_by(mut self, field: &str) -> OrderByQuery {
        self.0.sort_by.push(SortBy(field.into(), SortOrder::Asc));
        OrderByQuery(self.0)
    }
}

pub fn select(fields: &[&str]) -> SelectQuery {
    let mut query = RawQuery::new();
    query.select = fields.join(", ");
    SelectQuery(query)
}

impl fmt::Display for RawQuery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        let mut sql = "SELECT".to_string();

        if self.distinct {
            sql.push_str(" DISTINCT");
        }
        //select
        sql.push(' ');
        sql.push_str(&self.select);
        //from
        sql.push_str(" FROM ");
        sql.push_str(&self.from);
        //where
        if !self.where_.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.where_);
        }
        //group by
        if !self.group_by.is_empty() {
            sql.push_str(" GROUP BY ");
            sql.push_str(&self.group_by);
        }
        //having
        if !self.having.is_empty() {
            sql.push_str(" HAVING ");
            sql.push_str(&self.having);
        }
        //order by
        if !self.sort_by.is_empty() {
            sql.push_str(" ORDER BY ");
            sql.push_str(&self.sort_by.iter().map(|it| it.to_string()).collect::<Vec<String>>().join(", "));
        }
        //limit
        if let Some(ref limit) = self.limit {
            sql.push_str(" LIMIT ");
            sql.push_str(&limit);
        }
        //offset
        if let Some(ref offset) = self.offset {
            sql.push_str(" OFFSET ");
            sql.push_str(&offset);
        }       
        write!(f, "{}", sql)
    }
}

macro_rules! implement_display_for {
    ($t: ty) => {
        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    }
}

implement_display_for!(SelectQuery);
implement_display_for!(WhereQuery);
implement_display_for!(FinalQuery);
implement_display_for!(HavingQuery);
implement_display_for!(GroupQuery);
implement_display_for!(OrderByQuery);

#[cfg(test)]
mod test {

    use super::select;

    #[test]
    fn simple_query() {
        let sql = select(&["firstname", "lastname", "age"])
            .from("people")
            .to_string();
        assert_eq!(sql, "SELECT firstname, lastname, age FROM people");
    }

    #[test]
    fn distinct_query() {
        let sql = select(&["firstname", "lastname", "age"])
        .distinct()
        .from("people")
        .to_string();
        assert_eq!(sql, "SELECT DISTINCT firstname, lastname, age FROM people");
    }

    #[test]
    fn limit_query() {
        let sql = select(&["firstname", "lastname", "age"])
        .limit(Some(10))
        .from("people")
        .to_string();
        assert_eq!(sql, "SELECT firstname, lastname, age FROM people LIMIT 10");
    }

    #[test]
    fn offset_query() {
        let sql = select(&["firstname", "lastname", "age"])
        .offset(Some(10))
        .from("people")
        .to_string();
        assert_eq!(sql, "SELECT firstname, lastname, age FROM people OFFSET 10");
    }

    #[test]
    fn limit_offset_query() {
        let sql = select(&["firstname", "lastname", "age"])
        .offset(Some(10))
        .limit(Some(10))
        .from("people")
        .to_string();
        assert_eq!(sql, "SELECT firstname, lastname, age FROM people LIMIT 10 OFFSET 10");
    }

    #[test]
    fn distinct_limit_offset_query() {
        let sql = select(&["firstname", "lastname", "age"])
        .offset(Some(10))
        .limit(Some(10))
        .distinct()
        .from("people")
        .to_string();
        assert_eq!(sql, "SELECT DISTINCT firstname, lastname, age FROM people LIMIT 10 OFFSET 10");
    }

    #[test]
    fn srder_by_query() {
        let sql = select(&["firstname", "lastname", "age", "gender"])
        .from("people")
        .order_by("lastname")
        .order_by("firstname").asc()
        .order_by("age").desc()
        .order_by("gender").asc().desc()
        .to_string();
        assert_eq!(sql, "SELECT firstname, lastname, age, gender FROM people ORDER BY lastname ASC, firstname ASC, age DESC, gender DESC");
    }

    #[test]
    fn where_query() {
        let sql = select(&["firstname", "lastname", "age", "gender"])
        .from("people")
        .where_("age >= 16")
        .and("age <= 65")
        .or("lastname = 'Kowalski'")
        .order_by("lastname")        
        .to_string();
        assert_eq!(sql, "SELECT firstname, lastname, age, gender FROM people WHERE age >= 16 AND age <= 65 OR lastname = 'Kowalski' ORDER BY lastname ASC");
    }

    #[test]
    fn group_by_query() {
        let sql = select(&["firstname", "lastname", "Count(age) AS count"])
        .from("people")
        .group_by(&["firstname", "lastname"])
        .order_by("lastname")        
        .to_string();
        assert_eq!(sql, "SELECT firstname, lastname, Count(age) AS count FROM people GROUP BY firstname, lastname ORDER BY lastname ASC");
    }
    
    #[test]
    fn group_by_having_query() {
        let sql = select(&["firstname", "lastname", "Count(age) AS count, Sum(age) AS sum"])
        .from("people")
        .group_by(&["firstname", "lastname"])
        .having("count > 1").and("sum < 100").or("count = sum")
        .order_by("lastname")
        .to_string();
        assert_eq!(sql, "SELECT firstname, lastname, Count(age) AS count, Sum(age) AS sum FROM people GROUP BY firstname, lastname HAVING count > 1 AND sum < 100 OR count = sum ORDER BY lastname ASC");
    }

    #[test]
    fn where_group_by_having_query() {
        let sql = select(&["firstname", "lastname", "Count(age) AS count, Sum(age) AS sum"])
        .from("people")
        .where_("lastname <> 'Kowalski'").and("firstname <> 'Jan'")
        .group_by(&["firstname", "lastname"])
        .having("count > 1").and("sum < 100")
        .order_by("lastname")
        .to_string();
        assert_eq!(sql, "SELECT firstname, lastname, Count(age) AS count, Sum(age) AS sum FROM people WHERE lastname <> 'Kowalski' AND firstname <> 'Jan' GROUP BY firstname, lastname HAVING count > 1 AND sum < 100 ORDER BY lastname ASC");
    }

    #[test]
    fn full_query() {
        let sql = select(&["firstname", "lastname", "Count(age) AS count, Sum(age) AS sum"])
        .limit(None)
        .offset(None)
        .distinct()
        .from("people")
        .where_("lastname <> 'Kowalski'").and("firstname <> 'Jan'")
        .group_by(&["firstname", "lastname"])
        .having("count > 1").and("sum < 100")
        .order_by("lastname")
        .to_string();
        assert_eq!(sql, "SELECT DISTINCT firstname, lastname, Count(age) AS count, Sum(age) AS sum FROM people WHERE lastname <> 'Kowalski' AND firstname <> 'Jan' GROUP BY firstname, lastname HAVING count > 1 AND sum < 100 ORDER BY lastname ASC LIMIT ? OFFSET ?");
    }

    #[test]
    fn sub_query() {
        let sql = select(&["title"])
        .from("books")
        .where_(&format!("authorId = ({})", select(&["authorId"]).from("authors").where_("name = 'Walter Jon Williams'")).to_string())
        .order_by("title")
        .to_string();

        assert_eq!(sql, "SELECT title FROM books WHERE authorId = (SELECT authorId FROM authors WHERE name = 'Walter Jon Williams') ORDER BY title ASC")
    }
}
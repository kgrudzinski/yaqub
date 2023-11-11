
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ViewType {
    Temporary,
    Normal
}

impl fmt::Display for ViewType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "")?,
            Self::Temporary => write!(f, "TEMP ")?
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ViewCreate {
    name: String,
    select: String,
    typ: ViewType,
    columns: Vec<String>
}

impl ViewCreate {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            select: String::new(),
            typ: ViewType::Normal,
            columns: Vec::new()
        }
    }

    pub fn column(&mut self, col: &str) {
        self.columns.push(col.to_owned());
    }

    pub fn columns(&mut self, cols: &[&str]) {
        let arr: Vec<String> = cols.iter().map(|s| s.to_string()).collect();
        self.columns.extend_from_slice(&arr);        
    }

    pub fn as_(&mut self, query: &str) {
        self.select = query.to_string();        
    }

    pub fn temporary(&mut self) {
        self.typ = ViewType::Temporary;
    }
}

impl fmt::Display for ViewCreate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let columns = if !self.columns.is_empty() { format!("({})", self.columns.join(", ")) } else { "".to_string() };
        write!(f, "CREATE {}VIEW IF NOT EXISTS {}{} AS {};", self.typ, self.name, columns, self.select)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ViewCreateDef(ViewCreate);

impl ViewCreateDef {
    fn new(name: &str) -> Self {
        Self(ViewCreate::new(name))
    }

    pub fn temporary(mut self) -> Self {
        self.0.temporary();
        self
    }

    pub fn column(mut self, col: &str) -> Self {
        self.0.column(col);
        self
    }

    pub fn columns(mut self, cols: &[&str]) -> Self {
        self.0.columns(cols);
        self
    }

    pub fn as_(mut self, query: &str) -> ViewCreateStmt {
        self.0.as_(query);
        ViewCreateStmt(self.0)
    }
}

#[derive(Debug)]
pub struct ViewCreateStmt(ViewCreate);

impl ViewCreateStmt {
    pub fn typ_(&self) -> ViewType {
        self.0.typ
    }

    pub fn name(&self) -> &str {
        &(self.0.name)
    }

    pub fn columns(&self) -> &[String] {
        &(self.0.columns)
    }
}

impl fmt::Display for ViewCreateStmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct ViewDrop(String);

impl fmt::Display for ViewDrop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DROP VIEW IF EXISTS {};", self.0)?;
        Ok(())
    }
}

pub fn create_view(name: &str) -> ViewCreateDef {
    ViewCreateDef::new(name)
}

pub fn drop_view(name: &str) -> ViewDrop {
    ViewDrop(name.to_string())
}

#[cfg(test)]
mod tests {
    use super::{create_view, drop_view};

    #[test]
    fn create() {
        const SELECT: &str = "SELECT Username FROM users";
        
        let view = create_view("my_view").as_(SELECT);
        assert_eq!(view.to_string(), format!("CREATE VIEW IF NOT EXISTS my_view AS {};", SELECT));
    }

    #[test]
    fn create_temp() {
        const SELECT: &str = "SELECT Username FROM users";

        let view = create_view("my_view").temporary().as_(SELECT);
        assert_eq!(view.to_string(), format!("CREATE TEMP VIEW IF NOT EXISTS my_view AS {};", SELECT));
    }

    #[test]
    fn columns() {
        const SELECT: &str = "SELECT Username FROM users";

        let view = create_view("my_view")
            .temporary()
            .column("a")
            .column("b")
            .columns(&["c", "d"])
            .as_(SELECT);
        assert_eq!(view.to_string(), format!("CREATE TEMP VIEW IF NOT EXISTS my_view(a, b, c, d) AS {};", SELECT));
    }

    #[test]
    fn drop_() {
        let dview = drop_view("my_view");
        assert_eq!(dview.to_string(), "DROP VIEW IF EXISTS my_view;");
    }
}
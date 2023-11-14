use std::fmt::{self, Formatter};

#[derive(Debug, Clone, Copy)]
enum Action {
    Before,
    After,
    InsteadOf
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::After => write!(f, "AFTER")?,
            Self::Before => write!(f, "BEFORE")?,
            Self::InsteadOf => write!(f, "INSTEAD OF")?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Event {
    Insert,
    Update,
    Delete
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Delete => write!(f, "DELETE")?,
            Self::Insert => write!(f, "INSERT")?,
            Self::Update => write!(f, "UPDATE")?
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum TriggerType {
    Normal,
    Temporary
}

impl fmt::Display for TriggerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "")?,
            Self::Temporary => write!(f, "TEMP ")?
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Trigger {
    name: String,
    table: String,
    action: Action,
    event: Event,
    stmts: Vec<String>,
    typ: TriggerType,
    when: Option<String>
}

impl Trigger {
    fn new(name: &str) -> Self {
        Self { 
            name: name.to_owned(), 
            table: String::new(), 
            action: Action::Before, 
            event: Event::Insert, 
            stmts: Vec::new(), 
            typ: TriggerType::Normal,
            when: None
        }
    }

    fn on(&mut self, table: &str) {
        self.table = table.to_owned();
    }

    fn before(&mut self) {
        self.action = Action::Before;
    }

    fn after(&mut self) {
        self.action = Action::After;
    }

    fn instead_of(&mut self) {
        self.action = Action::InsteadOf;
    }

    fn insert(&mut self) {
        self.event = Event::Insert;
    }

    fn update(&mut self) {
        self.event = Event::Update;
    }

    fn delete(&mut self) {
        self.event = Event::Delete;
    }

    fn temporary(&mut self) {
        self.typ = TriggerType::Temporary;
    }

    fn statement(&mut self, stmt: &str) {
        self.stmts.push(stmt.to_owned());
    }

    fn statements(&mut self, stmts: &[&str]) {
        let arr: Vec<String> = stmts.iter().map(|s| s.to_string()).collect();
        self.stmts.extend_from_slice(&arr);
    }

    fn when(&mut self, expr: &str) {
        self.when = Some(expr.to_owned());
    }
}

impl fmt::Display for Trigger {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "CREATE {}TRIGGER IF NOT EXISTS {} {} {} ON {}", self.typ, self.name, self.action, self.event, self.table)?;
        if let Some(ref w) = self.when {
            writeln!(f, "WHEN {}", w)?;
        }

        writeln!(f,"BEGIN")?;
        for s in &self.stmts {
            writeln!(f, "{};", s)?;
        }
        write!(f, "END;")?;
        Ok(())
    }
}

pub struct TriggerNew(Trigger);

impl TriggerNew {
    pub fn new(name: &str) -> Self {
        Self(Trigger::new(name))
    }

    pub fn temporary(mut self) -> Self {
        self.0.temporary();
        self
    }

    pub fn after(mut self) -> TriggerWithAction {
        self.0.after();
        TriggerWithAction(self.0)
    }

    pub fn before(mut self) -> TriggerWithAction {
        self.0.before();
        TriggerWithAction(self.0)
    }

    pub fn instead_of(mut self) -> TriggerWithAction {
        self.0.instead_of();
        TriggerWithAction(self.0)
    }
}

pub struct TriggerWithAction(Trigger);

impl TriggerWithAction {
    pub fn delete(mut self) -> TriggerWithEvent {
        self.0.delete();
        TriggerWithEvent(self.0)
    }

    pub fn insert(mut self) -> TriggerWithEvent {
        self.0.insert();
        TriggerWithEvent(self.0)
    }

    pub fn update(mut self) -> TriggerWithEvent {
        self.0.update();
        TriggerWithEvent(self.0)
    }
}

pub struct TriggerWithEvent(Trigger);

impl TriggerWithEvent {
    pub fn on(mut self, table: &str) -> TriggerWithTable {
        self.0.on(table);
        TriggerWithTable(self.0)
    }
}

pub struct TriggerWithTable(Trigger);

impl TriggerWithTable {
    pub fn when(mut self, expr: &str) -> Self {
        self.0.when(expr);
        self
    }

    pub fn statement(mut self, stmt: &str) -> TriggerFull {
        self.0.statement(stmt);
        TriggerFull(self.0)
    }

    pub fn statements(mut self, stmts: &[&str]) -> TriggerFull {
        self.0.statements(stmts);
        TriggerFull(self.0)
    }
}

pub struct TriggerFull(Trigger);

impl TriggerFull {
    pub fn statement(mut self, stmt: &str) -> Self {
        self.0.statement(stmt);
        self
    }

    pub fn statements(mut self, stmts: &[&str]) -> Self {
        self.0.statements(stmts);
        self
    }
}

impl fmt::Display for TriggerFull {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub struct TriggerDrop(String);

impl TriggerDrop {
    fn new(name: &str) -> Self {
        Self(name.to_owned())
    }
}

impl fmt::Display for TriggerDrop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DROP TRIGGER IF EXISTS {};", self.0)
    }
}

pub fn create_trigger(name: &str) -> TriggerNew {
    TriggerNew::new(name)
}

pub fn drop_trigger(name: &str) -> TriggerDrop {
    TriggerDrop::new(name)
}

#[cfg(test)]
mod tests {
    use super::{create_trigger, drop_trigger};

    #[test]
    fn trigger_drop() {
        let td = drop_trigger("MyTrigger");

        assert_eq!(td.to_string(), "DROP TRIGGER IF EXISTS MyTrigger;");
    }

    #[test]
    fn trigger_create() {
        let trg1 = create_trigger("MyTrigger").after().delete().on("table").statement("stmt");        
        let trg2 = create_trigger("MyTrigger").after().insert().on("table").statement("stmt");
        let trg3 = create_trigger("MyTrigger").after().update().on("table").statement("stmt");
        let trg4 = create_trigger("MyTrigger").instead_of().delete().on("table").statement("stmt");
        let trg5 = create_trigger("MyTrigger").instead_of().insert().on("table").statement("stmt");
        let trg6 = create_trigger("MyTrigger").instead_of().update().on("table").statement("stmt");
        let trg7 = create_trigger("MyTrigger").before().delete().on("table").statement("stmt");
        let trg8 = create_trigger("MyTrigger").before().insert().on("table").statement("stmt");
        let trg9 = create_trigger("MyTrigger").before().update().on("table").statement("stmt");

        assert_eq!(trg1.to_string(), "CREATE TRIGGER IF NOT EXISTS MyTrigger AFTER DELETE ON table\nBEGIN\nstmt;\nEND;");
        assert_eq!(trg2.to_string(), "CREATE TRIGGER IF NOT EXISTS MyTrigger AFTER INSERT ON table\nBEGIN\nstmt;\nEND;");
        assert_eq!(trg3.to_string(), "CREATE TRIGGER IF NOT EXISTS MyTrigger AFTER UPDATE ON table\nBEGIN\nstmt;\nEND;");
        assert_eq!(trg4.to_string(), "CREATE TRIGGER IF NOT EXISTS MyTrigger INSTEAD OF DELETE ON table\nBEGIN\nstmt;\nEND;");
        assert_eq!(trg5.to_string(), "CREATE TRIGGER IF NOT EXISTS MyTrigger INSTEAD OF INSERT ON table\nBEGIN\nstmt;\nEND;");
        assert_eq!(trg6.to_string(), "CREATE TRIGGER IF NOT EXISTS MyTrigger INSTEAD OF UPDATE ON table\nBEGIN\nstmt;\nEND;");
        assert_eq!(trg7.to_string(), "CREATE TRIGGER IF NOT EXISTS MyTrigger BEFORE DELETE ON table\nBEGIN\nstmt;\nEND;");
        assert_eq!(trg8.to_string(), "CREATE TRIGGER IF NOT EXISTS MyTrigger BEFORE INSERT ON table\nBEGIN\nstmt;\nEND;");
        assert_eq!(trg9.to_string(), "CREATE TRIGGER IF NOT EXISTS MyTrigger BEFORE UPDATE ON table\nBEGIN\nstmt;\nEND;");
    }

    #[test]
    fn trigger_create_temp() {
        let trg1 = create_trigger("MyTrigger").temporary().after().delete().on("table").statement("stmt"); 
        assert_eq!(trg1.to_string(), "CREATE TEMP TRIGGER IF NOT EXISTS MyTrigger AFTER DELETE ON table\nBEGIN\nstmt;\nEND;");
    }

    #[test]
    fn trigger_create_when() {
        let trg1 = create_trigger("MyTrigger").temporary().after().delete().on("table").when("x < y").statement("stmt"); 
        assert_eq!(trg1.to_string(), "CREATE TEMP TRIGGER IF NOT EXISTS MyTrigger AFTER DELETE ON table\nWHEN x < y\nBEGIN\nstmt;\nEND;");
    }

    #[test]
    fn trigger_create_multi_stmt() {
        let trg1 = create_trigger("MyTrigger").after().delete().on("table").when("x < y").statement("stmt0").statement("stmt1");
        assert_eq!(trg1.to_string(), "CREATE TRIGGER IF NOT EXISTS MyTrigger AFTER DELETE ON table\nWHEN x < y\nBEGIN\nstmt0;\nstmt1;\nEND;");

        let trg2 = create_trigger("MyTrigger").after().delete().on("table").when("x < y").statements(&["stmt0", "stmt1"]);
        assert_eq!(trg2.to_string(), "CREATE TRIGGER IF NOT EXISTS MyTrigger AFTER DELETE ON table\nWHEN x < y\nBEGIN\nstmt0;\nstmt1;\nEND;");

        let trg3 = create_trigger("MyTrigger").after().delete().on("table").when("x < y").statement("stmt0").statements(&["stmt1", "stmt2"]).statement("stmt3");
        assert_eq!(trg3.to_string(), "CREATE TRIGGER IF NOT EXISTS MyTrigger AFTER DELETE ON table\nWHEN x < y\nBEGIN\nstmt0;\nstmt1;\nstmt2;\nstmt3;\nEND;");
    }

    #[test]
    fn trigger_create_full() {
        let trg1 = create_trigger("MyTrigger").temporary().before().update().on("table").when("x < y").statement("stmt0").statement("stmt1");
        assert_eq!(trg1.to_string(), "CREATE TEMP TRIGGER IF NOT EXISTS MyTrigger BEFORE UPDATE ON table\nWHEN x < y\nBEGIN\nstmt0;\nstmt1;\nEND;");
    }
}
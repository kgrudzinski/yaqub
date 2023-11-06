use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ForeignKeyAction {
    SetNull,
    SetDefault,
    Restrict,
    NoAction,
    Cascade
}

impl ForeignKeyAction {
    fn to_str(&self) -> &str {
        match self {            
            Self::SetNull => "SET NULL",
            Self::SetDefault => "SET DEFAULT",
            Self::Restrict => "RESTRICT",
            Self::NoAction => "NO ACTION",
            Self::Cascade => "CASCADE"
        }
    }
}

impl fmt::Display for ForeignKeyAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Debug)]
pub struct ForeignKey {
    col: String,
    ref_col: String,
    ref_table: String,
    delete_action: Option<ForeignKeyAction>,
    update_action: Option<ForeignKeyAction>
}

impl ForeignKey {
    pub fn new(col: &str) -> Self {
        Self {
            col: col.to_owned(),
            ref_col: String::new(),
            ref_table: String::new(),
            delete_action: Some(ForeignKeyAction::Restrict),
            update_action: None
        }
    }

    pub fn references(mut self, table: &str, col: &str) -> Self {
        self.ref_col = String::from(col);
        self.ref_table = String::from(table);
        self
    }

    pub fn on_update(mut self, action: ForeignKeyAction) -> Self {
        self.update_action = Some(action);
        self
    }

    pub fn on_delete(mut self, action: ForeignKeyAction) -> Self {
        self.delete_action = Some(action);
        self
    }
}

impl fmt::Display for ForeignKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " FOREIGN KEY ({}) REFERENCES {} ({}) ", self.col, self.ref_table, self.ref_col)?;
        if let Some(del) = self.delete_action {
            write!(f, "ON DELETE {} ", del)?;
        }
        if let Some(upd) = self.update_action {
            write!(f, "ON UPDATE {} ", upd)?;
        }
        Ok(())
    }
}
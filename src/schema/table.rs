use std::fmt;

use super::{Column, Check, ForeignKey};

pub struct Table {    
    name: String,
    cols: Vec<Column>,
    checks: Vec<Check>,
    foreign_keys: Vec<ForeignKey>
}

impl Table {
    fn new(name: &str) -> Self {
        Self {           
            name: name.to_owned(),
            cols: Vec::new(),
            checks: Vec::new(),
            foreign_keys: Vec::new()
        }
    }

    pub fn add_column(&mut self, col: Column) -> &mut Self {
        self.cols.push(col);
        self
    }

    pub fn add_check(&mut self, check: Check) -> &mut Self {
        self.checks.push(check);
        self
    }

    pub fn add_foreign_key(&mut self, foreign_key: ForeignKey) -> &mut Self {
        self.foreign_keys.push(foreign_key);
        self
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const SEP: &str = ",\n";
        writeln!(f, "CREATE TABLE {} IF NOT EXISTS (", self.name)?;       
        write!(f, "{}", fmt_array(&self.cols, SEP))?;
        if !self.checks.is_empty() {
            write!(f, "{}", SEP)?;
        }
        write!(f, "{}", fmt_array(&self.checks, SEP))?;
        if !self.foreign_keys.is_empty() {
            write!(f, "{}", SEP)?;
        }
        write!(f, "{}", fmt_array(&self.foreign_keys, SEP))?;
        write!(f, "\n);")?;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct AlterTable {
    name: String,
    new_name: Option<String>,
    cols: Vec<Column>,
    renames: Vec<(String, String)>,
    drops: Vec<String>
}

impl AlterTable {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..Self::default()
        }
    }

    pub fn rename_to(&mut self, new_name: &str) -> &mut Self {
        self.new_name = Some(new_name.to_owned());
        self
    }

    pub fn add_column(&mut self, col: Column) -> &mut Self {
        self.cols.push(col);
        self
    }

    pub fn rename_column(&mut self, col: &str, new_col: &str) -> &mut Self {
        self.renames.push((col.to_owned(), new_col.to_owned()));
        self
    }

    pub fn drop_column(&mut self, col: &str) -> &mut Self {
        self.drops.push(col.to_owned());
        self
    }
}

impl fmt::Display for AlterTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const SEP: &str = "\n";
        let len = self.cols.len() + self.renames.len() + self.cols.len() + if self.new_name.is_some() { 1 } else { 0 };
        let mut strings = Vec::with_capacity(len);
        if let Some(ref n) = self.new_name {
            strings.push(format!("ALTER TABLE {} RENAME TO {};", self.name, n));
        }
        
        for r in &self.renames {
            strings.push(format!("ALTER TABLE {} RENAME COLUMN {} TO {};", self.name, r.0, r.1));
        }

        for c in &self.cols {
            strings.push(format!("ALTER TABLE {} ADD COLUMN {};", self.name, c));
        }

        for d in &self.drops {
            strings.push(format!("ALTER TABLE {} DROP COLUMN {};", self.name, d));
        }

        write!(f, "{}", strings.join(SEP))?;
        Ok(())
    }
}

pub struct DropTable(String);

impl fmt::Display for DropTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DROP TABLE {};", self.0)
    }
}

fn fmt_array<T>(arr: &[T], sep: &str) -> String where T: fmt::Display {
    let mut strings: Vec<String> = Vec::new();
        for it in arr {
            strings.push(it.to_string());
        }
        strings.join(sep)
}

pub fn create_table(name: &str) -> Table {
    Table::new(name)
}

pub fn alter_table(name: &str) -> AlterTable {
    AlterTable::new(name)
}

pub fn drop_table(name: &str) -> DropTable {
    DropTable(name.to_string())
}

#[cfg(test)]
mod tests {

    //TODO: Add validation of generated sql queries, using sql parser eg. sql_parser crate

    use super::*;

    use crate::schema::{Column, ForeignKey, Check};    

    #[test]
    fn drop() {
        let td = drop_table("table").to_string();
        assert_eq!(td, "DROP TABLE table;");
    }

    #[test]
    fn alter_rename_to() {
        let tart = alter_table("table_1").rename_to("table_2").to_string();
        assert_eq!(tart, "ALTER TABLE table_1 RENAME TO table_2;");
    }

    #[test]
    fn alter_rename_col() {
        let tarc = alter_table("table_1").rename_column("col_1", "col_2").to_string();
        assert_eq!(tarc, "ALTER TABLE table_1 RENAME COLUMN col_1 TO col_2;");
    }

    #[test]
    fn alter_drop_col() {
        let atdc = alter_table("table_1").drop_column("col_1").to_string();
        assert_eq!(atdc, "ALTER TABLE table_1 DROP COLUMN col_1;")
    }

    #[test]
    fn alter_add_col() {
        let atac = alter_table("table_1").add_column(Column::new("col_1")).to_string();
        assert_eq!(atac, "ALTER TABLE table_1 ADD COLUMN col_1 INTEGER NOT NULL;");
    }

    #[test]
    fn create_basic() {
        let col_1 = Column::new("id").primary_key();
        let col_2 = Column::new("name").text().nullable();

        let col_1_str = col_1.to_string();
        let col_2_str = col_2.to_string();

        let table_str = format!("CREATE TABLE table_1 IF NOT EXISTS (\n{},\n{}\n);", col_1_str, col_2_str);

        let ct = create_table("table_1")
            .add_column(col_1)
            .add_column(col_2)
            .to_string();
        assert_eq!(ct, table_str);
    }

    #[test]
    fn create_with_foreign_key() {
        let col_1 = Column::new("id").primary_key();
        let col_2 = Column::new("name").text().nullable();
        let col_3 = Column::new("deptId").int();

        let col_1_str = col_1.to_string();
        let col_2_str = col_2.to_string();
        let col_3_str = col_3.to_string();

        let fk = ForeignKey::new("deptId").references("table_2", "id");
        let fk_str = fk.to_string();

        let table_str = format!("CREATE TABLE table_1 IF NOT EXISTS (\n{},\n{},\n{},\n{}\n);", col_1_str, col_2_str, col_3_str, fk_str);
        let ct = create_table("table_1")
            .add_column(col_1)
            .add_column(col_2)
            .add_column(col_3)
            .add_foreign_key(fk)
            .to_string();

        assert_eq!(ct, table_str);
    }

    #[test]
    fn create_with_constraint() {
        let col_1 = Column::new("id").primary_key();
        let col_2 = Column::new("name").text().nullable();
        let col_3 = Column::new("age").int();

        let col_1_str = col_1.to_string();
        let col_2_str = col_2.to_string();
        let col_3_str = col_3.to_string();

        let constraint = Check::new("age > 0 AND age < 150");
        let con_str = constraint.to_string();

        let table_str = format!("CREATE TABLE table_1 IF NOT EXISTS (\n{},\n{},\n{},\n{}\n);", col_1_str, col_2_str, col_3_str, con_str);
        let ct = create_table("table_1")
            .add_column(col_1)
            .add_column(col_2)
            .add_column(col_3)
            .add_check(constraint)
            .to_string();

        assert_eq!(ct, table_str);
    }

    #[test]
    fn create_full() {
        let col_1 = Column::new("id").primary_key();
        let col_2 = Column::new("name").text().nullable();
        let col_3 = Column::new("deptId").int();        
        let col_4 = Column::new("email").text();
        let col_5 = Column::new("roleId").int();
        let col_6 = Column::new("holidays").int();

        let col_1_str = col_1.to_string();
        let col_2_str = col_2.to_string();
        let col_3_str = col_3.to_string();
        let col_4_str = col_4.to_string();
        let col_5_str = col_5.to_string();
        let col_6_str = col_6.to_string();

        let check_1 = Check::new("email LIKE '%@%'");
        let check_2 = Check::new("hoildays >= 0 AND holidays <= 26");
        let check_1_str = check_1.to_string();
        let chesk_2_str = check_2.to_string(); 

        let fk_1 = ForeignKey::new("deptId").references("table_2", "id");
        let fk_1_str = fk_1.to_string();
        let fk_2 = ForeignKey::new("roleId").references("table_3", "id");
        let fk_2_str = fk_2.to_string();

        let table_str = format!("CREATE TABLE table_1 IF NOT EXISTS (\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n{}\n);", col_1_str, col_2_str, col_3_str, col_4_str, col_5_str, col_6_str,
                check_1_str, chesk_2_str, fk_1_str,fk_2_str);
        let ct = create_table("table_1")
            .add_column(col_1)
            .add_column(col_2)
            .add_column(col_3)
            .add_column(col_4)
            .add_column(col_5)
            .add_column(col_6)
            .add_check(check_1)
            .add_check(check_2)
            .add_foreign_key(fk_1)
            .add_foreign_key(fk_2)
            .to_string();

        assert_eq!(ct, table_str);
    }
}
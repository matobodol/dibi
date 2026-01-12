use prettytable::{Attr, Cell, Row, Table, color, format};

use crate::{Database, display::pt_error::PtError};

pub fn print_db(db: &Database, table_name: &str) -> Result<(), PtError> {
    let mut pt = Table::new();
    pt.set_format(*format::consts::FORMAT_BOX_CHARS);

    let tbl = db
        .tabel
        .get(table_name)
        .ok_or_else(|| PtError::TableNotFound)?;

    let header: Vec<Cell> = tbl
        .header
        .column
        .iter()
        .map(|k| {
            if k.flags.is_pk() {
                let s = format!("*{}", &k.name);
                Cell::new(&s)
                    .style_spec("c")
                    .with_style(Attr::Bold)
                    .with_style(Attr::ForegroundColor(color::YELLOW))
            } else {
                Cell::new(&k.name).style_spec("c").with_style(Attr::Bold)
            }
        })
        .collect();

    pt.add_row(Row::new(header));

    for rows in &tbl.vrow.hrow {
        let (l, c, r) = ("l", "c", "r");

        let row: Vec<Cell> = rows
            .value
            .iter()
            .map(|v| {
                let s = match v {
                    crate::Value::Null => ("-".to_string(), c),
                    crate::Value::Int(v) => (v.to_string(), r),
                    crate::Value::Float(v) => (v.to_string(), r),
                    crate::Value::Str(v) => (v.clone(), l),
                    crate::Value::Date(v) => (v.to_string(), r),
                    crate::Value::Enum { variant } => (variant.clone(), c),
                };

                Cell::new(&s.0).style_spec(s.1)
            })
            .collect();
        pt.add_row(Row::new(row));
    }

    println!("Tabel: \"{}\"", table_name);
    pt.printstd();
    println!();

    Ok(())
}

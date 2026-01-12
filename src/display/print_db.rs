use prettytable::{Attr, Cell, Row, Table, color, format};

use crate::{Database, display::pt_error::PtError};

pub fn print_db(db: &Database, table_name: &str) -> Result<(), PtError> {
    if db.tabel.is_empty() {
        println!("Belum ada Tabel..");
        return Ok(());
    }
    let tbl = db
        .tabel
        .get(table_name)
        .ok_or_else(|| PtError::TableNotFound)?;

    let mut pt = Table::new();
    pt.set_format(*format::consts::FORMAT_BOX_CHARS);

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

    let (l, c, r) = ("l", "c", "r");
    for values in 0..tbl.vrow.vrow_len() {
        let hrows = tbl
            .vrow
            .hrow_get_read(values)
            .ok_or_else(|| PtError::RowsNotFound)?;

        let hrows = hrows
            .iter()
            .map(|value| {
                let s = match value {
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

        pt.add_row(Row::new(hrows));
    }

    println!("Tabel: \"{}\"", table_name);
    pt.printstd();
    println!();

    Ok(())
}

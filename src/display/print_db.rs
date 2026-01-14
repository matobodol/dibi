use prettytable::{Attr, Cell, Row, Table, color, format};

use crate::{Database, db::Value, display::pt_error::PtError};

pub fn print_db(db: &Database, name: &str) -> Result<(), PtError> {
    if db.tabel.is_empty() {
        println!("Database masih kosong..");
        return Ok(());
    }

    let mut pt = Table::new();
    pt.set_format(*format::consts::FORMAT_BOX_CHARS);
    let (l, c, r) = ("l", "c", "r");

    let tbl = db.tabel.get(name).ok_or_else(|| PtError::TableNotFound)?;
    let cells: Vec<_> = tbl
        .header
        .get_header_name()
        .iter()
        .map(|name| {
            if tbl.header.is_primary_key(name) {
                let s = format!("*{}", name);
                Cell::new(&s)
                    .style_spec(c)
                    .with_style(Attr::Bold)
                    .with_style(Attr::ForegroundColor(color::YELLOW))
            } else {
                Cell::new(name).style_spec(c).with_style(Attr::Bold)
            }
        })
        .collect();

    pt.add_row(Row::new(cells.clone()));

    for values in 0..tbl.rows.vrow_len() {
        let hrows = tbl
            .rows
            .hrow_get_read(values)
            .ok_or_else(|| PtError::RowsNotFound)?;

        let hrows = hrows
            .iter()
            .map(|value| {
                let s = match value {
                    Value::Null => ("-".to_string(), c),
                    Value::Int(v) => (v.to_string(), r),
                    Value::Float(v) => (v.to_string(), r),
                    Value::Str(v) => (v.clone(), l),
                    Value::Date(v) => (v.to_string(), r),
                    Value::Enum { variant } => (variant.clone(), c),
                };

                Cell::new(&s.0).style_spec(s.1)
            })
            .collect();

        pt.add_row(Row::new(hrows));
    }

    pt.printstd();
    Ok(())
}

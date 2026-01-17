use prettytable::{Attr, Cell, Row, Table, color, format};

use crate::{Tabel, db::Value, display::pt_error::PtError};

pub fn print_db(tbl: &Tabel) -> Result<(), PtError> {
    let mut pt = Table::new();
    pt.set_format(*format::consts::FORMAT_BOX_CHARS);
    let (l, c, r) = ("l", "c", "r");

    let cells: Vec<_> = tbl
        .get_list_header_name()
        .iter()
        .map(|name| {
            if tbl.is_primary_key(name) {
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

    for values in 0..tbl.vrow_len() {
        let hrows = tbl
            .get_read_hrows(values)
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

                if s.0 == "-" {
                    Cell::new(&s.0)
                        .style_spec(s.1)
                        .with_style(Attr::ForegroundColor(color::BRIGHT_WHITE))
                } else {
                    Cell::new(&s.0).style_spec(s.1)
                }
            })
            .collect();

        pt.add_row(Row::new(hrows));
    }

    pt.printstd();
    println!();
    Ok(())
}

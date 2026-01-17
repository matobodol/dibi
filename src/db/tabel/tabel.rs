use serde::{Deserialize, Serialize};

use crate::db::{
    DbError, HeaderType, Value,
    baris::{
        baris::VRows,
        filter::{Filter, Op, Update},
    },
    flags::Eflags,
    schema::Header,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tabel {
    header: Header,
    rows: VRows,
}

impl Tabel {
    pub fn new() -> Self {
        Self {
            header: Header::new(),
            rows: VRows::new(),
        }
    }

    pub fn create(
        &mut self,
        name: &str,
        tipe: HeaderType,
        eflag: &[Eflags],
    ) -> Result<(), DbError> {
        self.validate_duplicate_header(name)?;

        self.header.add(name, tipe, eflag)?;
        Ok(())
    }

    pub fn insert(&mut self, hrow: Vec<Value>) -> Result<(), DbError> {
        let select_header = self.get_list_header_name();
        if hrow.len() > select_header.len() {
            return Err(DbError::ValuesCountIsGreet);
        }

        let mut row = self.rows.default(self.header_len());
        for (name, value) in select_header.into_iter().zip(hrow) {
            let idx = self.get_index_header(name)?;
            row[idx] = value;
        }

        self.validate_nullable(&row)?;
        self.rows.insert_values(row);
        Ok(())
    }

    pub fn insert_into(
        &mut self,
        select_header: Vec<&str>,
        values: Vec<Value>,
    ) -> Result<(), DbError> {
        self.validate_values_count(&select_header, &values)?;

        let mut row = self.rows.default(self.header_len());
        for (name, value) in select_header.into_iter().zip(values) {
            let idx = self.get_index_header(name)?;
            row[idx] = value;
        }

        self.validate_nullable(&row)?;
        self.rows.insert_values(row);
        Ok(())
    }

    pub fn drop_header(&mut self, names: &[&str]) -> Result<(), DbError> {
        self.validate_is_empty(&names)?;

        // validasi & kumpulkan index
        let mut indices = Vec::new();
        for &n in names {
            self.validate_is_axisting(n)?;
            self.validate_is_primary(n)?;

            indices.push(self.get_index_header(n)?);
        }

        // urutkan dan dedup
        indices.sort_unstable();
        indices.dedup();

        // drop ROWS dulu (pakai index lama)
        for &idx in indices.iter().rev() {
            self.rows.drop_value(idx);
        }

        // drop HEADER
        for &idx in indices.iter().rev() {
            self.header.drop_header_by_index(idx);
        }

        // rebuild index SEKALI
        self.header.update_index_header();

        Ok(())
    }
    pub fn drop_by_select(&mut self, name: &str, values: Vec<Value>) -> Result<(), DbError> {
        let vlen = self.rows.vrow_len();
        let mut baris = Vec::new();

        for i in 0..vlen {
            baris.push(self.rows.get_read_hrows(i).unwrap());
        }

        let idx = self.get_index_header(name)?;
        let mut bs: Vec<usize> = Vec::new();

        for v in &values {
            let b: Vec<usize> = baris
                .iter()
                .enumerate()
                .filter(|(_, k)| k.get(idx) == Some(v))
                .map(|(i, _)| i)
                .collect();

            bs.extend(b);
        }

        bs.sort_unstable();
        bs.dedup();

        for &v in bs.iter().rev() {
            self.rows.drop_vrow(v);
        }

        Ok(())
    }

    pub fn apply_delete(&mut self, name: &str, op: Op, value: Value) -> Result<(), DbError> {
        let filter = Filter::f(self.get_index_header(name)?, op, value);
        self.rows.apply_delete(&filter);

        Ok(())
    }
    pub fn apply_update(
        &mut self,
        _where: &str,
        set: Value,
        select_column: &str,
        op: Op,
        select_value: Value,
    ) -> Result<(), DbError> {
        let filter = Filter::f(self.get_index_header(select_column)?, op, select_value);
        let update = Update::u(self.get_index_header(_where)?, set);

        self.rows.apply_update(update, &filter);
        Ok(())
    }
}

// flags api service public
impl Tabel {
    pub fn set_primary_key(&mut self, name: &str) -> Result<(), DbError> {
        self.header.set_primary_key(name)?;
        Ok(())
    }
    pub fn unset_primary_key(&mut self) -> Result<(), DbError> {
        self.header.unset_primary_key()?;
        Ok(())
    }
    pub fn set_nullable(&mut self, name: &str) -> Result<(), DbError> {
        self.header.set_nullable(name)?;
        Ok(())
    }
    pub fn unset_nullable(&mut self, name: &str) -> Result<(), DbError> {
        self.header.unset_nullable(name)?;
        Ok(())
    }
    pub fn set_increment(&mut self, name: &str) -> Result<(), DbError> {
        self.header.set_increment(name)?;
        Ok(())
    }
    pub fn unset_increment(&mut self, name: &str) -> Result<(), DbError> {
        self.header.unset_increment(name)?;
        Ok(())
    }
}

// header api service public
impl Tabel {
    pub fn header_len(&self) -> usize {
        self.header.header_len()
    }

    pub fn get_header_type(&self, name: &str) -> Result<&HeaderType, DbError> {
        self.header.get_header_type(name)
    }

    pub fn get_list_header_name(&self) -> Vec<&str> {
        self.header.get_list_names()
    }

    pub fn get_index_header(&self, name: &str) -> Result<usize, DbError> {
        self.header.get_header_index(name)
    }

    pub fn is_primary_key(&self, name: &str) -> bool {
        self.header.is_primary_key(name)
    }
    pub fn is_nullable(&self, name: &str) -> bool {
        self.header.is_nullable(name)
    }
    pub fn is_increment(&self, name: &str) -> bool {
        self.header.is_increment(name)
    }
}

// rows api service public
impl Tabel {
    pub fn vrow_len(&self) -> usize {
        self.rows.vrow_len()
    }

    pub fn hrow_len(&self, idx_vrow: usize) -> Option<usize> {
        self.rows.hrow_len(idx_vrow)
    }

    pub fn get_read_hrows(&self, idx_vrow: usize) -> Option<&[Value]> {
        self.rows.get_read_hrows(idx_vrow)
    }

    pub fn get_read_values(&self, idx_vrow: usize, idx_header: usize) -> Option<&Value> {
        self.rows.get_read_values(idx_vrow, idx_header)
    }
    pub(crate) fn is_axisting(&self, name: &str) -> bool {
        self.header.is_axisting(name)
    }
}

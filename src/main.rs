use mini_db::{Database, Flags, HRows, HeaderType, Value, display::print_db::print_db};

fn main() {
    let mut db = Database::new();

    /* nama tabel */
    let user = "user";
    db.create_table(user).unwrap();

    /* nama tabel2 */
    let siswa = "siswa";
    db.create_table(siswa).unwrap();

    /* USER */
    let tabel_user = db.tabel.get_mut(user).unwrap();
    tabel_user
        .new_header("id", HeaderType::Int, Flags::default())
        .unwrap();
    tabel_user
        .new_header("User", HeaderType::Str, Flags::default())
        .unwrap();
    tabel_user
        .new_header("Alamat", HeaderType::Str, Flags::default())
        .unwrap();

    tabel_user.header.set_pk("User").unwrap();

    tabel_user
        .insert_rows(HRows {
            value: vec![Value::Int(1), Value::Str("Jono".into())],
        })
        .unwrap();
    tabel_user
        .insert_rows(HRows {
            value: vec![Value::Int(2), Value::Str("Budi Wisesa".into())],
        })
        .unwrap();

    /* SISWA */
    let tabel_siswa = db.tabel.get_mut(siswa).unwrap();
    tabel_siswa
        .new_header("id", HeaderType::Int, Flags::default())
        .unwrap();
    tabel_siswa
        .new_header("Siswa", HeaderType::Str, Flags::default())
        .unwrap();
    tabel_siswa
        .new_header("Kelas", HeaderType::Str, Flags::default())
        .unwrap();

    tabel_siswa.header.set_pk("id").unwrap();

    tabel_siswa
        .insert_rows(HRows {
            value: vec![
                Value::Int(1),
                Value::Str("Jani".into()),
                Value::Str("7E".into()),
            ],
        })
        .unwrap();
    tabel_siswa
        .insert_rows(HRows {
            value: vec![
                Value::Int(2),
                Value::Str("Bunga Teman Sebangku Jani".into()),
                Value::Str("9A".into()),
            ],
        })
        .unwrap();
    tabel_siswa
        .insert_rows(HRows {
            value: vec![
                Value::Int(3),
                Value::Str("Budi Bukan Pacar Jani".into()),
                Value::Str("8B".into()),
            ],
        })
        .unwrap();

    print_db(&db, siswa).unwrap();
    print_db(&db, user).unwrap();
}

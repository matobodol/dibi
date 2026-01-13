use mini_db::{
    Database, Tabel,
    db::{HeaderType, Value, flags::EFlags},
};

fn main() {
    let mut db = Database::new();
    db.tabel.insert("user".into(), Tabel::new());

    let user = db.tabel.get_mut("user").unwrap();

    user.new_header("id", HeaderType::Int, EFlags::Pk).unwrap();
    user.new_header("nama", HeaderType::Str, EFlags::Default)
        .unwrap();
    user.new_header("alamat", HeaderType::Str, EFlags::Pk)
        .unwrap();

    user.insert_rows(vec![
        Value::Int(1),
        Value::Str("Jani".into()),
        Value::Str("jl angkasa".into()),
    ])
    .unwrap();
    user.insert_rows(vec![
        Value::Int(1),
        Value::Str("Joni".into()),
        Value::Str("jl samudra".into()),
    ])
    .unwrap();
    user.insert_rows(vec![
        Value::Int(1),
        Value::Str("Nono".into()),
        Value::Str("jl bumi datar".into()),
    ])
    .unwrap();

    user.header.set_pk("nama").unwrap();

    println!("{:#?}", user)
}

use mini_db::{
    Database, Tabel,
    db::{HeaderType, Value, flags::Eflags},
    display::print_db::print_db,
};

fn main() {
    let mut db = Database::new();
    db.tabel.insert("user".into(), Tabel::new());

    let user = db.tabel.get_mut("user").unwrap();

    user.new_header("id", HeaderType::Int, &[Eflags::Inc, Eflags::Pk])
        .unwrap();
    user.new_header("nama", HeaderType::Str, &[Eflags::Default])
        .unwrap();
    user.new_header("alamat", HeaderType::Str, &[Eflags::Default])
        .unwrap();

    user.insert_rows(vec![
        Value::Int(1),
        Value::Str("Jani".into()),
        Value::Str("jl angkasa".into()),
    ])
    .unwrap();
    user.insert_rows(vec![
        Value::Int(2),
        Value::Str("Joni bukan pacar jani".into()),
        Value::Str("jl bumi bulat apa datar".into()),
    ])
    .unwrap();
    user.insert_rows(vec![
        Value::Int(003),
        Value::Str("Jono nyebrang samudra".into()),
        Value::Str("jl laut kidul".into()),
    ])
    .unwrap();

    user.header.set_primary_key("id").unwrap();
    println!("{:#?}", user);
    print_db(&db, "user").unwrap();
}

use mini_db::{
    Database, DbError, HeaderType, Tabel, Value,
    db::{baris::filter::Op, flags::Eflags},
    display::print_db::print_db,
};
fn build_tabel(user: &mut Tabel) -> Result<(), DbError> {
    user.create("id", HeaderType::Int, &[Eflags::Inc, Eflags::Pk])?;
    user.create("nama", HeaderType::Str, &[Eflags::Default])?;
    user.create("alamat", HeaderType::Str, &[Eflags::Default])?;

    user.insert(vec![
        Value::Int(1),
        Value::Str("Jani".into()),
        Value::Str("jl angkasa".into()),
    ])?;
    user.insert(vec![
        Value::Int(2),
        Value::Str("Joni bukan pacar jani".into()),
        Value::Str("jl bumi bulat apa datar".into()),
    ])?;
    user.insert(vec![
        Value::Int(003),
        Value::Str("Jono nyebrang samudra".into()),
        Value::Str("jl laut kidul".into()),
    ])?;

    Ok(())
}

fn main() -> Result<(), DbError> {
    let mut db = Database::new();
    db.tabel.insert("user".into(), Tabel::new());

    let user = db.tabel.get_mut("user").unwrap();
    build_tabel(user).unwrap();
    println!("-----build tabel------");
    print_db(&user).unwrap();

    user.set_primary_key("nama")?;
    user.set_nullable("nama")?;
    user.set_increment("nama")?;
    println!("-----ubah flags------");
    print_db(&user).unwrap();

    // user.unset_primary_key()?;
    user.insert_into(vec!["nama"], vec![Value::Str("jana".into())])?;
    user.insert_into(vec!["nama"], vec![Value::Str("jana".into())])?;
    user.insert_into(vec!["nama"], vec![Value::Str("jana".into())])?;
    user.insert_into(vec!["nama"], vec![Value::Str("jana".into())])?;
    println!("-----insert into------");
    print_db(&user).unwrap();

    // user.drop_header(&["alamat"])?;
    // println!("-----drop header------");
    // print_db(&user).unwrap();

    // delete by filter
    user.apply_delete("nama", Op::Eq, Value::Str("jana".into()))?;
    println!("-----apply delete------");
    print_db(&user).unwrap();

    // update by filter
    user.apply_update(
        "nama",
        Value::Str("Jini oh jini".into()),
        "nama",
        Op::Eq,
        Value::Str("Jani".into()),
    )?;
    println!("-----apply update------");
    print_db(&user).unwrap();

    // println!("{:#?}", user);

    Ok(())
}

use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Person {
    account: String,
    forename: String,
    surname: String,
    role: String,
    criminal: bool,
    data: Option<String>,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute("\
            CREATE TABLE person (
            account text not null primary key, \
            forename text not null, \
            surname text not null, \
            role text not null, \
            criminal bool not null default false, \
            data text defalt none);
        )",
        (), // empty list of parameters.
    )?;
    let me = Person {
        account: "nils.wrenger".into(),
        forename: "Nils".into(),
        surname: "Wrenger".into(),
        role: "Klasse 10a".into(),
        criminal: false,
        data: None,
    };
    conn.execute(
        "INSERT INTO person (account, forename, surname, role, criminal, data) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (&me.account, &me.forename, &me.surname, &me.role, &me.criminal, &me.data),
    )?;

    let mut stmt = conn.prepare("SELECT account, forename, surname, role, criminal, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            account: row.get("account")?,
            forename: row.get("forename")?,
            surname: row.get("surname")?,
            role: row.get("role")?,
            criminal: row.get("criminal")?,
            data: row.get("data")?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}
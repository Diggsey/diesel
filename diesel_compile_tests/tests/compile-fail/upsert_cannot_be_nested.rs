#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

use diesel::*;
use diesel::pg::upsert::*;

table! {
    users {
        id -> Integer,
        name -> VarChar,
    }
}

#[derive(Insertable)]
#[table_name="users"]
struct NewUser(#[column_name(name)] &'static str);

#[allow(deprecated)]
fn main() {
    use self::users::dsl::*;
    let connection = PgConnection::establish("postgres://localhost").unwrap();

    insert_into(users)
        .values(&NewUser("Sean")
                .on_conflict_do_nothing()
                .on_conflict_do_nothing());
                //~^ ERROR no method named `on_conflict_do_nothing` found
    insert_into(users)
        .values(&NewUser("Sean").on_conflict_do_nothing())
        .on_conflict_do_nothing();
        //~^ ERROR no method named `on_conflict_do_nothing` found
    insert_into(users)
        .values(&NewUser("Sean"))
        .on_conflict_do_nothing()
        .on_conflict_do_nothing();
        //~^ ERROR no method named `on_conflict_do_nothing` found
    insert_into(users)
        .values(&NewUser("Sean")
                .on_conflict(id, do_nothing())
                .on_conflict_do_nothing());
                //~^ ERROR no method named `on_conflict_do_nothing` found
    insert_into(users)
        .values(&NewUser("Sean").on_conflict(id, do_nothing()))
        .on_conflict_do_nothing();
        //~^ ERROR no method named `on_conflict_do_nothing` found
    insert_into(users)
        .values(&NewUser("Sean")
                .on_conflict_do_nothing()
                .on_conflict(id, do_nothing()));
                //~^ ERROR no method named `on_conflict` found
    insert_into(users)
        .values(&NewUser("Sean"))
        .on_conflict_do_nothing()
        .on_conflict(id);
        //~^ ERROR no method named `on_conflict` found
    insert_into(users)
        .values(&NewUser("Sean")
                .on_conflict_do_nothing())
        .on_conflict(id);
        //~^ ERROR no method named `on_conflict` found
    insert_into(users)
        .values(&NewUser("Sean")
                .on_conflict(id, do_nothing())
                .on_conflict(id, do_nothing()));
                //~^ ERROR no method named `on_conflict` found
    insert_into(users)
        .values(&NewUser("Sean"))
        .on_conflict(id)
        .do_nothing()
        .on_conflict(id);
        //~^ ERROR no method named `on_conflict` found
    insert_into(users)
        .values(&vec![NewUser("Sean").on_conflict_do_nothing()]);
        //~^ ERROR UndecoratedInsertRecord
    insert_into(users)
        .values(&vec![&NewUser("Sean").on_conflict_do_nothing()]);
        //~^ ERROR UndecoratedInsertRecord
    insert_into(users)
        .values(&vec![&NewUser("Sean").on_conflict(id, do_nothing())]);
        //~^ ERROR UndecoratedInsertRecord
    insert_into(users)
        .values(&(name.eq("Sean").on_conflict_do_nothing(),));
        //~^ ERROR UndecoratedInsertRecord
}

use crate::model::User;
use postgres::Error as PostgresError;
use postgres::{Client, NoTls};
use std::env;
use std::error::Error;

fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL is required")
}

pub fn set_database() -> Result<(), PostgresError> {
    let database_url = get_database_url();
    let mut client = Client::connect(&database_url, NoTls)?;

    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        name VARCHAR NOT NULL,
        email VARCHAR NOT NULL
    )",
        &[],
    )?;

    Ok(())
}

fn get_client() -> Result<Client, PostgresError> {
    Client::connect(&get_database_url(), NoTls)
}

pub fn create_user(user: &User) -> Result<User, PostgresError> {
    let mut client = get_client()?;

    let row = client.query_one(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
        &[&user.name, &user.email],
    )?;

    Ok(User {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
    })
}

pub fn get_user(id: i32) -> Result<User, PostgresError> {
    let mut client = get_client()?;

    let row = client.query_one("SELECT id, name, email FROM users WHERE id=$1", &[&id])?;

    Ok(User {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
    })
}

pub fn get_all_users() -> Result<Vec<User>, PostgresError> {
    let mut client = get_client()?;

    let rows = client
        .query("SELECT id, name, email FROM users ORDER BY id ASC", &[])
        .unwrap();

    let mut users = Vec::new();
    for row in rows {
        users.push(User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        });
    }

    Ok(users)
}

pub fn update_user(user: User) -> Result<User, Box<dyn Error>> {
    let mut client = get_client()?;

    let rows_affected = client.execute(
        "UPDATE users SET name = $1, email = $2 WHERE id = $3",
        &[&user.name, &user.email, &user.id],
    )?;

    if rows_affected == 0 {
        return Err(format!("User with id {} not found", user.id.unwrap()).into());
    }

    Ok(user)
}

pub fn delete_user(id: i32) -> Result<(), Box<dyn Error>> {
    let mut client = get_client()?;

    let rows_affected = client.execute("DELETE FROM users WHERE id = $1", &[&id])?;

    if rows_affected == 0 {
        return Err(format!("User with id {} not found", id).into());
    }

    Ok(())
}

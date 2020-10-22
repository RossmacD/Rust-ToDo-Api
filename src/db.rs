use crate::models::{TodoItem, TodoList};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    let statement = client
        .prepare("SELECT * FROM todo_list ORDER BY id DESC")
        .await
        .unwrap();
    let todos = client
        .query(&statement, &[])
        .await
        .expect("Error: Could not retrieve lists")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(todos)
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, io::Error> {
    let statement = client
        .prepare("SELECT * FROM todo_item WHERE list_id = $1 ORDER BY id")
        .await
        .unwrap();

    let todos = client
        .query(&statement, &[&list_id])
        .await
        .expect("Error: Could not retrieve lists")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();

    Ok(todos)
}

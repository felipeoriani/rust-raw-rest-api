use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::{db::*, model::User};

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";
const UNPROCESSABLE_ENTITY: &str = "HTTP/1.1 422 UNPROCESSABLE ENTITY\r\n\r\n";
const NO_CONTENT: &str = "HTTP/1.1 204 NO CONTENT\r\n\r\n";

fn get_id(request: &str) -> &str {
    request
        .split("/")
        .nth(2)
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .unwrap_or_default()
}

fn get_user_request_body(request: &str) -> Result<User, String> {
    let body = request.split("\r\n\r\n").last().unwrap_or_default();

    let user: User = match serde_json::from_str(body) {
        Ok(user) => user,
        Err(_) => return Err("Invalid JSON format".to_string()),
    };

    if user.name.trim().is_empty() {
        return Err("Name is required".to_string());
    }

    if user.email.trim().is_empty() {
        return Err("Email is required".to_string());
    }

    Ok(user)
}

fn handle_post_request(request: &str) -> (String, String) {
    match get_user_request_body(request) {
        Ok(user) => match create_user(&user) {
            Ok(user) => (
                OK_RESPONSE.to_string(),
                serde_json::to_string(&user).unwrap(),
            ),
            _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
        },
        Err(error) => (UNPROCESSABLE_ENTITY.to_string(), error.to_string()),
    }
}

fn handle_get_request(request: &str) -> (String, String) {
    match get_id(&request).parse::<i32>() {
        Ok(id) => match get_user(id) {
            Ok(user) => (
                OK_RESPONSE.to_string(),
                serde_json::to_string(&user).unwrap(),
            ),
            _ => (NOT_FOUND.to_string(), "User not found".to_string()),
        },
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

fn handle_get_all_request(_request: &str) -> (String, String) {
    match get_all_users() {
        Ok(users) => (
            OK_RESPONSE.to_string(),
            serde_json::to_string(&users).unwrap(),
        ),
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

fn handle_put_request(request: &str) -> (String, String) {
    let user_id = match get_id(request).parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            return (
                INTERNAL_SERVER_ERROR.to_string(),
                "Invalid user ID".to_string(),
            );
        }
    };

    let mut user = match get_user_request_body(request) {
        Ok(user) => user,
        Err(error) => {
            return (UNPROCESSABLE_ENTITY.to_string(), error.to_string());
        }
    };

    user.id = Some(user_id);

    match update_user(user) {
        Ok(user) => (
            OK_RESPONSE.to_string(),
            serde_json::to_string(&user).unwrap(),
        ),
        Err(_) => (
            INTERNAL_SERVER_ERROR.to_string(),
            "Failed to update user".to_string(),
        ),
    }
}

fn handle_delete_request(request: &str) -> (String, String) {
    match get_id(&request).parse::<i32>() {
        Ok(id) => match delete_user(id) {
            Ok(_) => (NO_CONTENT.to_string(), "User deleted".to_string()),
            _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
        },
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            let s = String::from_utf8_lossy(&buffer[..size].as_ref());
            request.push_str(&s);

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /users") => handle_post_request(r),
                r if r.starts_with("GET /users/") => handle_get_request(r),
                r if r.starts_with("GET /users") => handle_get_all_request(r),
                r if r.starts_with("PUT /users") => handle_put_request(r),
                r if r.starts_with("DELETE /users") => handle_delete_request(r),
                _ => (NOT_FOUND.to_string(), "Not Found".to_string()),
            };

            stream
                .write_all(format!("{}{}", status_line, content).as_bytes())
                .unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn init_server(port: i32) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    println!("Server started at port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

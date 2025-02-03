use actix_web::{web, App, HttpServer, HttpResponse, Responder, Error};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use async_stream::stream;
use actix_web::web::Bytes;

#[derive(Serialize, Deserialize, Clone)]
struct Usuario {
    id: Option<i32>,
    nombre: String,
    contrasena: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Tarea {
    id: i32,
    descripcion: String,
    progreso: f32,
}

fn connect_db() -> rusqlite::Result<Connection> {
    Connection::open("api_tarea1.db")
}

async fn get_usuarios() -> impl Responder {
    let conn = connect_db().unwrap();
    
    let mut stmt = conn.prepare("SELECT id, nombre, contrasena FROM usuarios").unwrap();
    let usuarios_iter = stmt.query_map([], |row| {
        Ok(Usuario {
            id: row.get(0)?,
            nombre: row.get(1)?,
            contrasena: row.get(2)?,
        })
    }).unwrap();

    let usuarios: Vec<Usuario> = usuarios_iter.map(|user| user.unwrap()).collect();

    HttpResponse::Ok().json(usuarios)
}

async fn login(usuario: web::Json<Usuario>) -> impl Responder {
    let conn = connect_db().unwrap();
    
    let mut stmt = conn.prepare("SELECT id, nombre, contrasena FROM usuarios WHERE nombre = ? AND contrasena = ?").unwrap();
    let mut rows = stmt.query([&usuario.nombre, &usuario.contrasena]).unwrap();

    if let Some(row) = rows.next().unwrap() {
        let usuario_db = Usuario {
            id: row.get(0).unwrap(),
            nombre: row.get(1).unwrap(),
            contrasena: row.get(2).unwrap(),
        };

        HttpResponse::Ok().json(usuario_db)
    } else {
        HttpResponse::Unauthorized().json("Usuario o contraseña incorrectos")
    }
}

async fn crear_usuario(usuario: web::Json<Usuario>) -> impl Responder {
    let conn = connect_db().unwrap();

    let query = format!(
        "INSERT INTO usuarios (nombre, contrasena) VALUES ('{}', '{}')", 
        usuario.nombre, usuario.contrasena
    );

    conn.execute(&query, []).unwrap();

    HttpResponse::Ok().json(usuario.into_inner())
}


async fn tarea_proceso(sender: broadcast::Sender<Tarea>) {
    for i in 0..=100 {
        let tarea = Tarea {
            id: 1,
            descripcion: "Tarea en proceso".to_string(),
            progreso: i as f32,
        };
        // El método send es síncrono. Se ignora el resultado.
        let _ = sender.send(tarea);
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}


async fn iniciar_tarea(sender: web::Data<broadcast::Sender<Tarea>>) -> impl Responder {
    let sender_inner = sender.get_ref().clone();
    tokio::spawn(async move {
        tarea_proceso(sender_inner).await;
    });
    HttpResponse::Ok().json("Tarea iniciada")
}



async fn progreso_tarea(sender: web::Data<broadcast::Sender<Tarea>>) -> impl Responder {
    let mut rx = sender.subscribe();

    
    let task_stream = stream! {
        loop {
            match rx.recv().await {
                Ok(tarea) => {
                    
                    let json = serde_json::to_string(&tarea).unwrap() + "\n";
                    yield Ok::<Bytes, Error>(Bytes::from(json));
                }
                Err(broadcast::error::RecvError::Lagged(_)) => continue,
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    };

    HttpResponse::Ok().streaming(task_stream)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    
    let (tx, _rx) = broadcast::channel::<Tarea>(32);

    HttpServer::new(move || {
        App::new()
        
            .app_data(web::Data::new(tx.clone()))
            .route("/usuarios", web::get().to(get_usuarios))
            .route("/login", web::post().to(login))
            .route("/crearUsuario", web::post().to(crear_usuario))
            .route("/iniciarTarea", web::post().to(iniciar_tarea))
            .route("/progresoTarea", web::get().to(progreso_tarea))
    })
    .bind("0.0.0.0:8080")?
    // .bind("127.0.0.1:8080")?
    .run()
    .await
}

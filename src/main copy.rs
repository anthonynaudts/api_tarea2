use actix_web::{web, App, HttpServer, HttpResponse, Responder, Error};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, Mutex};
use std::sync::Arc;
use reqwest::Client;
use yup_oauth2::{ServiceAccountAuthenticator, ServiceAccountKey};
use std::collections::HashMap;
use std::fs;
use env_logger;

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

#[derive(Serialize, Deserialize)]
struct IniciarTareaRequest {
    token_fcm: String,
}

#[derive(Serialize, Deserialize)]
struct NotificacionRequest {
    token_fcm: String,
    titulo: String,
    mensaje: String,
}


type UsuariosTokens = Arc<Mutex<HashMap<i32, String>>>;

#[allow(dead_code)]
fn connect_db() -> rusqlite::Result<Connection> {
    Connection::open("api_tarea1.db")
}



async fn obtener_access_token() -> Result<String, Error> {
    let creds = fs::read_to_string("service-account.json")
        .map_err(|_| actix_web::error::ErrorInternalServerError("❌ No se pudo leer el archivo de credenciales"))?;
    
    let service_account_key: ServiceAccountKey = serde_json::from_str(&creds)
        .map_err(|_| actix_web::error::ErrorInternalServerError("❌ Error en el formato de credenciales"))?;

    let auth = ServiceAccountAuthenticator::builder(service_account_key)
        .build()
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("❌ No se pudo autenticar con Firebase"))?;

    let token = auth.token(&["https://www.googleapis.com/auth/firebase.messaging"]).await
        .map_err(|_| actix_web::error::ErrorInternalServerError("❌ Error al obtener el token de acceso"))?;

        eprintln!("✅ Access Token obtenido correctamente.");
    Ok(token.token().unwrap_or("").to_string())
}



async fn enviar_notificacion(
    token_fcm: &str,
    titulo: &str,
    mensaje: &str
) -> Result<(), Error> {
    let access_token = obtener_access_token().await?;

    let project_id = "notificacionestarea2"; 
    let url = format!("https://fcm.googleapis.com/v1/projects/{}/messages:send", project_id);

    let payload = serde_json::json!({
        "message": {
            "token": token_fcm,
            "notification": {
                "title": titulo,
                "body": mensaje
            }
        }
    });

    let client = Client::new();
    let response = client
        .post(&url)
        .bearer_auth(&access_token)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("❌ Error al enviar la solicitud a Firebase"))?;

    let status = response.status();
    let body = response.text().await.unwrap_or_else(|_| "❌ Error obteniendo respuesta".to_string());

    eprintln!("🔹 Respuesta de Firebase: Status = {}, Body = {}", status, body);

    if status.is_success() {
        Ok(())
    } else {
        Err(actix_web::error::ErrorInternalServerError(format!("❌ Error en la respuesta de Firebase: {}", body)))
    }
}

async fn api_enviar_notificacion(request: web::Json<NotificacionRequest>) -> impl Responder {


    let result = enviar_notificacion(
        &request.token_fcm,
        &request.titulo,
        &request.mensaje
    ).await;

    match result {
        Ok(_) => HttpResponse::Ok().json("✅ Notificación enviada correctamente"),
        Err(err) => HttpResponse::InternalServerError().json(format!("❌ Error al enviar notificación: {:?}", err)),
    }
}



async fn tarea_proceso(sender: broadcast::Sender<Tarea>, token_fcm: String) {
    eprintln!("🔄 tarea_proceso() ha iniciado para el token: {}", token_fcm);

    for i in 0..=100 {
        let tarea = Tarea {
            id: 1,
            descripcion: "Tarea en proceso".to_string(),
            progreso: i as f32,
        };
        let _ = sender.send(tarea);
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        eprintln!("📊 Progreso: {}%", i);
    }

    eprintln!("⚡ Enviando notificación a: {}", token_fcm);

    let result = enviar_notificacion(&token_fcm, "Tarea Completada", "Tu tarea ha finalizado exitosamente").await;

    match result {
        Ok(_) => eprintln!("✅ Notificación enviada con éxito"),
        Err(err) => eprintln!("❌ Error al enviar notificación: {:?}", err),
    }
}


async fn iniciar_tarea(
    sender: web::Data<broadcast::Sender<Tarea>>,
    usuarios_tokens: web::Data<UsuariosTokens>,
    request: web::Json<IniciarTareaRequest>,
) -> impl Responder {
    let token_fcm = request.token_fcm.clone();
    let usuario_id = 1;

    
    eprintln!("✅ iniciar_tarea() fue llamado con token: {}", token_fcm);

    let mut tokens = usuarios_tokens.lock().await;
    tokens.insert(usuario_id, token_fcm.clone());

    let sender_inner = sender.get_ref().clone();
    tokio::spawn(async move {
        eprintln!("🔄 Ejecutando tarea_proceso()...");
        tarea_proceso(sender_inner, token_fcm).await;
    });

    HttpResponse::Ok().json("Tarea iniciada")
}


async fn progreso_tarea(sender: web::Data<broadcast::Sender<Tarea>>) -> impl Responder {
    let mut rx = sender.subscribe();

    let task_stream = async_stream::stream! {
        loop {
            match rx.recv().await {
                Ok(tarea) => {
                    let json = serde_json::to_string(&tarea).unwrap() + "\n";
                    yield Ok::<actix_web::web::Bytes, Error>(actix_web::web::Bytes::from(json));
                }
                Err(broadcast::error::RecvError::Lagged(_)) => continue,
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    };

    HttpResponse::Ok().streaming(task_stream)
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


#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    eprintln!("🚀 Servidor iniciando en http://0.0.0.0:8080");
    let (tx, _rx) = broadcast::channel::<Tarea>(32);
    let usuarios_tokens: UsuariosTokens = Arc::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
        
            .app_data(web::Data::new(tx.clone()))
            .app_data(web::Data::new(usuarios_tokens.clone()))
            .route("/usuarios", web::get().to(get_usuarios))
            .route("/login", web::post().to(login))
            .route("/crearUsuario", web::post().to(crear_usuario))
            .route("/iniciarTarea", web::post().to(iniciar_tarea))
            .route("/progresoTarea", web::get().to(progreso_tarea))
            .route("/enviarNotificacion", web::post().to(api_enviar_notificacion))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

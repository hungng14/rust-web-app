#[allow(dead_code)]
#[allow(unused)]
use actix_web::{
    Result, dev, http, middleware, error, Error, web, delete, post, put, get, guard, App, HttpResponse, HttpRequest, HttpServer, Responder
};
mod config;
use mylib;
use actix_files as fs;
// use futures::StreamExt;

#[get("/posts")]
async fn get_posts() -> HttpResponse {
    let conn = mylib::db::establish_connection();
    let posts = mylib::repo::post::get_posts(&conn);
    HttpResponse::Ok().json(posts)
}

#[get("/posts/{post_id}")]
async fn get_post(req: HttpRequest) -> HttpResponse {
    let id = req.match_info().query("post_id").parse::<i32>().unwrap();
    let conn = mylib::db::establish_connection();
    let post = mylib::repo::post::get_post(&conn, id);
    match post {
        Ok(p) => {
            if p.len() > 0 {return HttpResponse::Ok().json(&p[0]);}
            return HttpResponse::NotFound().finish();
        },
        Err(e) => {
            HttpResponse::Ok().body(format!("Error here: {}", e))
        }
    }
}

#[post("/posts")]
async fn create_post(new_post: web::Json<mylib::models::NewPost>) -> HttpResponse {
    let conn = mylib::db::establish_connection();
    let post = mylib::repo::post::create_post(&conn, new_post.0);
    HttpResponse::Ok().json(mylib::models::Post {..post})
}

#[put("/posts/{post_id}")]
async fn update_post(req: HttpRequest, post: web::Json<mylib::models::UpdatePost>) -> HttpResponse {
    let id = req.match_info().query("post_id").parse::<i32>().unwrap();
    let conn = mylib::db::establish_connection();
    let post = mylib::repo::post::update_post(&conn, id, post.0);
    HttpResponse::Ok().json(mylib::models::Post {..post})
}

#[delete("/posts/{post_id}")]
async fn delete_post(web::Path(id): web::Path<i32>) -> HttpResponse {
    let conn = mylib::db::establish_connection();
    let post = mylib::repo::post::delete_post(&conn, id);
    HttpResponse::Ok().json(mylib::models::Post {..post})
}

async fn index() -> impl Responder {
    let template = match mylib::utils::get_file_content(String::from("client/dist/index.html")) {
        Ok(t) => t,
        Err(_) => String::from("")
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .header("X-Hdr", "sample")
        .body(template)
}

#[get("/{filename:.*}")]
async fn static_f(req: HttpRequest) -> impl Responder {
    let filename = req.match_info().query("filename");
    println!("filename here{}", filename);
    let mut path = String::from("client/dist/");
    path.push_str(filename);
    let check_img = mylib::utils::is_image_file(&filename.to_string());
    if check_img.ok {
        match mylib::utils::get_img_content(path) {
            Ok(f) => {
                return dev::HttpResponseBuilder::new(http::StatusCode::OK)
                .set_header(http::header::CONTENT_TYPE, check_img.kind)
                .body(f);
            },
            Err(_) => {
                return HttpResponse::NotFound().finish();
            }
        };
        
    }

    match mylib::utils::get_file_content(path) {
        Ok(f) => {
            let content_type = mylib::utils::get_content_type(filename.to_string());
            return dev::HttpResponseBuilder::new(http::StatusCode::OK)
            .set_header(http::header::CONTENT_TYPE, content_type)
            .body(f)
        },
        Err(_) => {
            return HttpResponse::NotFound().finish();
        }
    }
}

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let app_config = config::Config::from_env();
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    
    let server = HttpServer::new( || {
        let logger = middleware::Logger::default();
        App::new()
            .service(fs::Files::new("/client", ".").show_files_listing().use_last_modified(true))
            .wrap(logger)
            .app_data(
                web::JsonConfig::default()
                    .limit(1024 * 1024)
                    .error_handler(|err, _req| { 
                        println!("Error here: {}", err);
                        error::InternalError::from_response(
                        err, HttpResponse::BadRequest().finish()).into()
                    })
            )
            .route("/", web::get().to(index))
            .service(
                web::scope("/")
                    .service(static_f)
            )
            .service(
                web::scope("/api/v1")
                    .service(get_posts)
                    .service(get_post)
                    .service(create_post)
                    .service(update_post)
                    .service(delete_post)
            )
            
            
    });
    println!("Server is running at {}:{}", app_config.host, app_config.port);
    server.bind(format!("{}:{}", app_config.host, app_config.port))?.run().await
}

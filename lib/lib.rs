use actix_files::NamedFile;
use actix_multipart::Multipart;
use futures::TryStreamExt;
use actix_web::dev::Server;
use actix_web::{ routes, get, post, web, http, error, App, Error as ActixError, HttpResponse, HttpServer };
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpListener;
use std::fs::{File, self};
use handlebars::Handlebars;

#[get("/")]
async fn index() -> Result<HttpResponse, ActixError> {
    Ok(HttpResponse::MovedPermanently().append_header((http::header::LOCATION, "/index.html")).finish())
}

#[routes]
#[get("/info")]
#[get("/info/")]
#[get("/assets")]
#[get("/assets/")]
#[get("/upload")]
#[get("/upload/{_:.*}")]
#[post("/upload")]
#[post("/upload/")]
async fn bad_request() -> Result<HttpResponse, ActixError> {
    let msg = "Bad request, are you pray yet?";
    println!("{}", msg);
    Err(error::ErrorBadRequest(msg))
}

#[get("/{name:.*}")]
async fn site(name: web::Path<String>) -> Result<NamedFile, ActixError> {
    println!("Serving page {} from file site/{}", name, name);
    let path = format!("site/{}", name);
    Ok(NamedFile::open(path)?)
}

#[routes]
#[get("/health")]
#[get("/health/.*")]
async fn health() -> Result<HttpResponse, ActixError> {
    Ok(HttpResponse::Ok().body("OK!!!"))
}

#[get("/info/{name}")]
async fn info(name: web::Path<String>) -> Result<NamedFile, ActixError> {
    println!("Serving page /info/{} from file pages/{}", name, name);
    let path = format!("pages/{}", name);
    Ok(NamedFile::open(path)?)
}

#[get("/assets/{name:.*}")]
async fn assets(name: web::Path<String>) -> Result<NamedFile, ActixError> {
    println!("Serving assets /assets/{} from file assets/{}", name, name);
    let path = format!("assets/{}", name);
    Ok(NamedFile::open(path)?)
}

#[post("/upload/{template:.*}")]
async fn upload(hb: web::Data<Handlebars<'_>>, template: web::Path<String>, mut payload: Multipart) -> Result<HttpResponse, ActixError> {
    println!("Handle form upload /upload/{} and will render file template/{}.html as response", template, template);
    let mut datamap: HashMap<String, String> = HashMap::new();
    let mut filemap: HashMap<String, Vec<u8>> = HashMap::new();

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        let key = content_disposition.get_name().unwrap().to_string();
        let filename = content_disposition.get_filename().map(|s| s.to_string());

        let mut file_buffer = Vec::<u8>::new();
        match filename {
            None => {
                let bytes = field.try_next().await?;
                let value = String::from_utf8(bytes.unwrap().to_vec()).expect("Unable read form data");
                println!("Got key [{}] with value [{:?}]", key, value);
                datamap.insert(key, value);
            }

            Some(fname) => {
                // buffer uploaded file into memory first
                while let Some(chunk) = field.try_next().await? {
                    file_buffer.extend_from_slice(&chunk);
                }

                println!("Uploaded key [{}] file [{}] buffered", key, fname);
                datamap.insert(key, fname.clone());
                filemap.insert(fname, file_buffer);
            }
        }
    }

    if let Some(id) = datamap.get("id") {
        if let Some(name) = datamap.get(id) {
            // render result
            let html = hb.render(template.as_str(), &datamap).expect("Unable render page");

            //save as file
            let redirect = format!("/info/{}.html", name.as_str());
            println!("Will redirected to {}", redirect);
            let hpath = format!("./pages/{}.html", name);
            println!("Caching result to {} directory", hpath);
            _ = web::block(move || {
                let dirpath = std::path::Path::new(&hpath);
                if let Some(dir) = dirpath.parent() {
                    fs::create_dir_all(dir).expect("unable to create directory")
                }
                let mut f = File::create(dirpath).unwrap();
                f.write_all(html.as_bytes()).expect("Unable to save rendered page");
            }).await;

            // save all uploaded files
            for (filename, buffer) in filemap {
                let filepath = format!("./assets/upload/{}", filename);
                println!("Saving uploaded file into {:?} ", filepath);
                
                // save uploaded file
                _ = web::block(move || {
                    let dirpath = std::path::Path::new(&filepath);
                    if let Some(dir) = dirpath.parent() {
                        fs::create_dir_all(dir).expect("unable to create directory")
                    }
                    let mut f = File::create(dirpath).expect("unable to create file");
                    f.write_all(buffer.as_slice()).expect("unable to write file");
                }).await;
            }
            
            // return redirect
            Ok(HttpResponse::SeeOther().append_header((http::header::LOCATION, redirect)).finish())
        }
        else {
            let msg = format!("Unable to find key [{:#?}] to render, check your hidden param named 'id' value", id);
            println!("{}", msg);
            Err(error::ErrorBadRequest(msg))
        }
    }
    else {
        let msg = "Unable to find id to render, have you set hidden param named 'id' already?";
        println!("{}", msg);
        Err(error::ErrorBadRequest(msg))
    }
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(false);
    handlebars.register_templates_directory(".html", "./templates/").expect("Unable load template directory");

    let handlebars_ref = web::Data::new(handlebars);

    let server = HttpServer::new(move|| {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(bad_request)
            .service(index)
            .service(health)
            .service(upload)
            .service(info)
            .service(assets)
            .service(site)
    })
    .listen(listener)?
    .run();
    Ok(server)
}

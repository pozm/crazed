use std::process::Stdio;
use rocket::{catchers, Request, routes, catch, route, fs::{relative}, Responder, Data, request};
use rocket::data::{ToByteUnit};
use rocket::fs::{FileServer, TempFile};
use rocket::http::Status;
use rocket::post;
use rocket::request::{FromRequest, Outcome};
use rocket::tokio::process::Command;
use uuid::Uuid;


#[derive(Responder)]
#[response(status = 404, content_type = "text")]
struct Err404(String);
#[catch(404)]
fn invalid_request(req: &Request) -> Err404 {
    // println!("{:?}",req.client_ip().unwrap());
    Err404(format!("hi, {:?} is not a valid endpoint",req.uri()))
}

struct ProgramLang(String);

#[rocket::async_trait]
impl<'a> FromRequest<'a> for ProgramLang {
    type Error = &'a str;

    async fn from_request(request: &'a Request<'_>) -> request::Outcome<Self, Self::Error> {
        let lang = request.headers().get_one("lang");
        match lang {
            Some(lang) => {
                // check validity
                Outcome::Success(ProgramLang(lang.to_string()))
            }
            None => Outcome::Failure((Status::BadRequest, "Unable to get lang")),
        }
    }
}

#[post("/run",data = "<data>")]
async fn run(data: TempFile<'_>,lang:ProgramLang) -> String {
    let mut output = String::new();

    // output = lang.0.clone();
    // output.push_str(data.path().unwrap().to_str().unwrap());

    let cmd_output = match lang.0.as_str() {
        "js" => {
            Some(Command::new("node")
                .arg(data.path().unwrap().to_str().unwrap())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn().unwrap())
        }
        "py" => { // please compile this time
            Some(Command::new("python")
                .arg(data.path().unwrap().to_str().unwrap())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn().unwrap())
        }
        "rust" => {

            Some(Command::new("cargo")
                .arg("play")
                .arg(data.path().unwrap().to_str().unwrap())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn().unwrap())
        }
        "c++" => {
            let uuid = Uuid::new_v4().to_string();
            Some(Command::new("bash")
                .arg("-c")
                .arg(format!("eval \"g++ -x c++ {} -o {} && ./{} && rm -rf {}\"",data.path().unwrap().to_str().unwrap(),uuid,uuid,uuid))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn().unwrap())
        }
        "c" => {
            let uuid = Uuid::new_v4().to_string();
            Some(Command::new("bash")
                .arg("-c")
                .arg(format!("eval \"g++ -x c {} -o {} && ./{} && rm -rf {}\"",data.path().unwrap().to_str().unwrap(),uuid,uuid,uuid))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn().unwrap())
        }
        _ => {
            output.push_str(format!("{:?} Is not a valid language",lang.0).as_str());
            None
        }
    };
    if let Some(cmd_output) = cmd_output {
        let c = tokio::time::timeout(std::time::Duration::from_secs(40), cmd_output.wait_with_output()).await;
        if let Ok(Ok(pog)) = c {
            output.push_str(&format!("Errors:\n{}",String::from_utf8(pog.stderr.to_vec()).unwrap_or("".to_string())));
            output.push_str(&format!("\n\nOutput:\n{}",String::from_utf8(pog.stdout.to_vec()).unwrap_or("".to_string())));
        } else {
            output.push_str("Timeout");
            // bx_cmd.kill().await.unwrap();
        }
    }

    output
}

#[rocket::main]
async fn main() {
    println!("working file: {}",std::env::current_exe().unwrap().to_str().unwrap());
    let rocket = rocket::build()
        .mount("/api/", routes![run])
        .register("/", catchers![invalid_request])
        .launch()
        .await
        .unwrap();
}

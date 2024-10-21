# nodejs-expressjs-to-rust

refer: 
[https://www.polynique.com/web-development/converting-a-rest-api-from-node-js-and-express-to-rust](https://www.polynique.com/web-development/converting-a-rest-api-from-node-js-and-express-to-rust)

โค้ดจาก expressjs

ไพล์ `server.js'

    const express = require('express');
    const app = express();

    app.use(express.json());

    app.post('/', (req, res) => {
    const user = req.body;
    res.json(user);
    });

    app.listen(8080, () => {
        console.log('API listening on port 8080');
    });

สร้าง cargo project

    $ cargo new demoapi

ีupdate ไพล์ `Cargo.toml`

    [dependencies]
    actix-web = "3.0"
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"

update ไพล์ `src/main.rs`

    use actix_web::{web, App, HttpResponse, HttpServer, Result};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct User {
        name: String,
        age: i32,
    }

    async fn index(user: web::Json<User>) -> Result<HttpResponse> {
        Ok(HttpResponse::Ok().json(user.0))
    }

    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
        HttpServer::new(|| App::new().route("/", web::post().to(index)))
            .bind(("127.0.0.1", 7010))?
            .run()
            .await
    }

Testing 

รันด้วยคำสั่ง 

    $ cargo run

เรียกใช้ api ด้วย endpoint / จะใช้ cURL หรือ Postman ก็ได้

    $ curl -X POST -H "Content-Type: application/json" -d ' {"name":"Polynique","age":30}' http://127.0.0.1:7010/
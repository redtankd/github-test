use actix::prelude::*;
use actix_web::{get, web, App, HttpServer, Responder};

use rand::{thread_rng, Rng};

use bench::*;

pub struct LimitServer {
    limit_manager: LimitManager,
}

impl LimitServer {
    pub fn new(entity_count: usize, entity_sqno_shift: usize) -> LimitServer {
        let mut rng = thread_rng();
        let mut limit_manager = LimitManager::new(entity_count, entity_sqno_shift);
        for i in 0..entity_count {
            for j in (i + 1)..entity_count {
                let left_amount = rng.gen_range(1, 101) * 1000;
                let right_amount = rng.gen_range(1, 101) * 1000;
                let _ = limit_manager.insert(i, left_amount, j, right_amount);
            }
        }
        return LimitServer {
            limit_manager: limit_manager,
        };
    }
}

impl Actor for LimitServer {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<(), LimitError>")]
pub struct Deduct(pub usize, pub usize, pub LimitAmount);

impl Handler<Deduct> for LimitServer {
    type Result = Result<(), LimitError>;

    fn handle(&mut self, msg: Deduct, _: &mut Context<Self>) -> Self::Result {
        self.limit_manager.deduct(msg.0, msg.1, msg.2)
    }
}

#[get("/{left}/{right}/{amount}")]
async fn index(
    info: web::Path<(usize, usize, LimitAmount)>,
    limit_server: web::Data<Addr<LimitServer>>,
) -> impl Responder {
    if let Ok(Ok(())) = limit_server.get_ref().send(Deduct(info.0, info.1, info.2)).await {
        // println!("{} - {} - {} - ok", info.0, info.1, info.2);
        format!("ok")
    } else {
        // println!("{} - {} - {} - error", info.0, info.1, info.2);
        format!("error")
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let limit_server = LimitServer::new(10000, 100000);
    let limit_server = limit_server.start();
    let limit_server = web::Data::new(limit_server);

    HttpServer::new(move || App::new().app_data(limit_server.clone()).service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

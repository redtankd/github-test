use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;

use actix::prelude::*;
use actix_web::{get, web, App, HttpServer, Responder};
use log::info;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use limit::*;

#[derive(Serialize, Deserialize, Debug)]
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
    if let Ok(Ok(())) = limit_server
        .get_ref()
        .send(Deduct(info.0, info.1, info.2))
        .await
    {
        format!("ok")
    } else {
        format!("error")
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let opts = clap::App::new("Limit Server")
        .arg(clap::Arg::from_usage("-s 'Save Limit Manager'"))
        .arg(clap::Arg::from_usage("-l 'Load Limit Manager'"))
        .get_matches();

    if opts.is_present("s") {
        info!("Initing limit server");
        let limit_server = LimitServer::new(10000, 100000);

        info!("Serializing limit server");
        let encoded: Vec<u8> = bincode::serialize(&limit_server).unwrap();

        info!("Saving limit server");
        let mut file = File::create("limit_server.bin")?;
        file.write_all(&encoded)?;
        info!("Saving finished");

        return Ok(());
    }

    let limit_server = if opts.is_present("l") {
        info!("Loading limit server");
        let mut file = File::open("limit_server.bin")?;
        let size = file.metadata()?.len();
        let mut decode: Vec<u8> = Vec::with_capacity(size.try_into().unwrap());
        file.read_to_end(&mut decode)?;

        info!("Deserializing limit server");
        let ls = bincode::deserialize(&decode).unwrap();
        info!("Deserializing finished");
        ls
    } else {
        LimitServer::new(10000, 100000)
    };

    let limit_server = limit_server.start();
    let limit_server = web::Data::new(limit_server);

    println!("Limit Server started");

    HttpServer::new(move || App::new().app_data(limit_server.clone()).service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

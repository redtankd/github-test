use std::convert::TryInto;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

use actix::prelude::*;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
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

        let mut limit_manager =
            LimitManager::new(entity_count * (entity_count - 1) / 2, entity_sqno_shift);
        for i in 0..entity_count {
            for j in (i + 1)..entity_count {
                let left_amount = rng.gen_range(1..101) * 1000;
                let right_amount = rng.gen_range(1..101) * 1000;
                let _ = limit_manager.insert(i, left_amount, j, right_amount);
            }
        }

        info!(
            "Creating a new Limit Manager with {} items and the shift is {}.",
            limit_manager.get_limits().len(),
            limit_manager.get_shift()
        );

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
    path: web::Path<(usize, usize, LimitAmount)>,
    limit_server: web::Data<Addr<LimitServer>>,
) -> impl Responder {
    let (left, right, amount) = path.into_inner();
    
    if let Ok(Ok(())) = limit_server
        .get_ref()
        .send(Deduct(left, right, amount))
        .await
    {
        HttpResponse::Ok()
    } else {
        HttpResponse::NotFound()
    }
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let opts = clap::App::new("Limit Server")
        .args_from_usage(
            "-s            'Save Limit Manager to file'
             -l            'Load Limit Manager from file'
             --redis       'Save Limit Manager to redis'",
        )
        .arg(
            clap::Arg::from_usage("-f [FILE] 'Location of archive'")
                .default_value("limit_server.bin"),
        )
        .arg(
            clap::Arg::from_usage("--count [COUNT] 'The number of Entities'")
                .default_value("10000"),
        )
        .arg(
            clap::Arg::from_usage(
                "--shift [SHIFT] 'The shift of key, key = left sqno * shift + right sqno'",
            )
            .default_value("100000"),
        )
        .get_matches();

    let entity_count = usize::from_str(opts.value_of("count").unwrap()).unwrap();
    let shift = usize::from_str(opts.value_of("shift").unwrap()).unwrap();
    let file_name = opts.value_of("f").unwrap();

    if opts.is_present("s") {
        return save_file(entity_count, shift, file_name).await;
    }

    if opts.is_present("redis") {
        match save_redis(entity_count, shift) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }

    let limit_server = if opts.is_present("l") {
        load_file(file_name).await?
    } else {
        LimitServer::new(entity_count, shift)
    };

    let limit_server = limit_server.start();
    let limit_server = web::Data::new(limit_server);

    println!("Limit Server started");

    HttpServer::new(move || App::new().app_data(limit_server.clone()).service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

async fn save_file(entity_count: usize, shift: usize, file_name: &str) -> io::Result<()> {
    info!("Creating a new limit server");
    let limit_server = LimitServer::new(entity_count, shift);

    info!("Serializing limit server");
    let encoded: Vec<u8> = bincode::serialize(&limit_server).unwrap();

    info!("Saving limit server");
    let mut file = File::create(file_name)?;
    file.write_all(&encoded)?;

    info!("Saving finished");
    Ok(())
}

async fn load_file(file_name: &str) -> io::Result<LimitServer> {
    info!("Loading limit server");
    let mut file = File::open(file_name)?;
    let size = file.metadata()?.len();
    let mut decode: Vec<u8> = Vec::with_capacity(size.try_into().unwrap());
    file.read_to_end(&mut decode)?;

    info!("Deserializing limit server");
    let limit_server = bincode::deserialize(&decode).unwrap();

    info!("Deserializing finished");
    Ok(limit_server)
}

fn save_redis(entity_count: usize, shift: usize) -> redis::RedisResult<()> {
    info!("Creating a new limit server");
    let limit_server = LimitServer::new(entity_count, shift);

    info!("Starting mass inject");

    let mut pipe = redis::pipe();
    for (k, v) in limit_server.limit_manager.get_limits() {
        let f = vec![
            ("left", v.get_left()),
            ("right", v.get_right()),
            ("double", v.get_double()),
        ];
        pipe.hset_multiple(*k, &f);
    }

    info!("Saving limit server to a local default redis");

    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut conn = client.get_connection()?;
    pipe.execute(&mut conn);

    info!("Saving finished");

    return Ok(());
}

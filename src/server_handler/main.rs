use anyhow::{bail, Result};
use backend::http::HttpCodec;
use backend::{image, recipe};
use backend::{Config, ConfigFile};
use clap::Parser;
use http::{Request, Response};
use std::fs;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use futures::executor::block_on;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use backend::authorization::Authorization;

const DEFAULT_CONFIG: &str = "./config.toml";

/// Simple http server
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Address to bind to
    #[arg(short, long)]
    address: Option<String>,

    /// Config file
    #[arg(short, long, default_value = DEFAULT_CONFIG, long_help = "Path to a server config file. \
    Config file options are overridden by arguments provided via the command line.")]
    config: Option<PathBuf>,

    /// Root image folder
    #[arg(short, long)]
    image_folder: Option<PathBuf>,

    /// Log verbosity level
    #[arg(short, long)]
    verbosity: Option<log::Level>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = parse_args_into_config(args)?;
    simple_logger::init_with_level(config.log_level)?;

    let auth = Authorization::new(config.auth_file.clone());

    let listener = TcpListener::bind(&config.address)?;
    log::info!("Bound TcpListener to {}", config.address);
    let db_pool = block_on(create_db_connection(&config))?;
    log::info!("Established database connection with {}", config.database.address);

    for stream in listener.incoming() {
        log::info!("Incoming connection");
        match stream {
            Ok(stream) => {
                match handle_connection(stream, &config, &db_pool, &auth) {
                    Ok(_) => { log::info!("Successfully handled connection"); },
                    Err(err) => { log::error!("Error handling connection - {}", err); }
                }
            }
            Err(e) => { log::error!("Error with incoming connection - {}", e); }
        }
    }
    
    Ok(())
}

fn handle_connection(stream: TcpStream, config: &Config, db_pool: &PgPool, auth: &Authorization) -> Result<()> {
    let mut http = HttpCodec::new(stream)?;

    let request = http.receive_request();
    log::info!("Received request");
    let response = match request {
        Ok(request) => {
            log::info!("Routing request '{}'", request.uri());
            route_request(request, config, db_pool, auth)
        },
        Err(err) => {
            log::error!("Error receiving request - {}", err);
            http::Response::builder().status(http::StatusCode::BAD_REQUEST).body(vec![])?
        },
    };

    log::info!("Sending {} response", response.status());
    http.send_response(response)?;
    
    Ok(())
}

fn route_request(request: Request<Vec<u8>>, config: &Config, db_pool: &PgPool, auth: &Authorization) -> Response<Vec<u8>> {
    if image::can_handle_request(&request) {
        log::debug!("Routing request to image");
        return image::handle_request(request, config, auth)
    }

    if recipe::can_handle_request(&request) {
        log::debug!("Routing request to recipe");
        return recipe::handle_request(request, db_pool, auth)
    }

    log::info!("No valid route for request '{}'", request.uri());
    http::Response::builder().status(http::StatusCode::BAD_REQUEST).body(vec![]).unwrap()
}

fn parse_args_into_config(args: Args) -> Result<Config> {
    let config_file = args.config.clone();
    let config_file = config_file.unwrap_or(PathBuf::from(DEFAULT_CONFIG));

    let config = match config_file.exists() {
        true => toml::from_str(&fs::read_to_string(config_file)?)?,
        false => bail!("Config file does not exist: {}", config_file.display())
    };

    config_from_file_and_args(config, args)
}

fn config_from_file_and_args(config: ConfigFile, args: Args) -> Result<Config> {
    let mut file_address = config.address;
    if args.address.is_some() {
        file_address = args.address;
    }
    let address = match file_address {
        Some(address) => address,
        None => bail!("No address provided"),
    };

    let mut image_folder = config.image_folder;
    if args.image_folder.is_some() {
        image_folder = args.image_folder;
    }
    let image_folder = match image_folder {
        Some(image_folder) => image_folder,
        None => bail!("No image folder provided"),
    };

    let database = config.database.try_into()?;

    let log_level = match args.verbosity {
        Some(verbose) => verbose,
        None => config.verbose.unwrap_or(log::Level::Info),
    };

    let auth_file = config.auth_file.unwrap_or_else(|| PathBuf::from(".auth"));

    Ok(Config { address, image_folder, database, log_level, auth_file })
}

async fn create_db_connection(config: &Config) -> anyhow::Result<PgPool> {
    let username = &config.database.username;
    let password = &config.database.password;
    let address = &config.database.address;
    let name = &config.database.name;
    let conn = format!("postgresql://{username}:{password}@{address}/{name}");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn).await?;
    Ok(pool)
}
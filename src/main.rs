use std::io::Cursor;
use std::net::{IpAddr, SocketAddr};

use chrono::{Utc, SecondsFormat};
use warp::Filter;
use warp::http::{Response, StatusCode};

#[tokio::main]
async fn main() {
    println!("[{}] Initializing {} {}", iso_string(), env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let server_address: SocketAddr = ([0, 0, 0, 0], 3033).into();

    let info = warp::path::end()
        .and(warp::get())
        .map(|| format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));

    let tracking_pixel = warp::path!("pixel" / String / "a.png")
        .and(warp::get())
        .and(warp::filters::addr::remote())
        .and_then(tracking_pixel_handler);

    let routes = info
        .or(tracking_pixel);

    println!("[{}] Starting web server on {}...", iso_string(), server_address);
    warp::serve(routes)
        .run(server_address)
        .await;
}

fn iso_string() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}

async fn tracking_pixel_handler(_cache_nonce: String, socket_addr: Option<SocketAddr>) -> Result<impl warp::Reply, warp::Rejection> {
    let image = socket_addr
        .ok_or("no remote address".to_string())
        .and_then(|socket_addr| ip_to_image(socket_addr.ip()).map_err(|e| format!("Error encoding PNG: {:?}", e)));

    match image {
        Ok(image) => {
            Ok(
                Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "image/png")
                    .body(image)
            )
        }
        Err(e) => {
            eprintln!("[{}] {}", iso_string(), e);
            Ok(
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header("Content-Type", "text/plain")
                    .body(e.into())
            )
        }
    }
}

fn ip_to_image(addr: IpAddr) -> Result<Vec<u8>, png::EncodingError> {
    println!("[{}] got request from {}", iso_string(), addr);

    let mut buffer = Cursor::new(Vec::new());

    {
        let width: u32 = match addr {
            IpAddr::V4(_) => 2, // 2 * 3 = 6, which is enough for 4 bytes
            IpAddr::V6(_) => 6, // 6 * 3 = 18, which is enough for 16 bytes
        };

        let mut encoder = png::Encoder::new(&mut buffer, width, 1);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(png::Compression::Fast);
        let mut writer = encoder.write_header()?;

        match addr {
            IpAddr::V4(addr) => {
                let octets = addr.octets();
                let data = [
                    octets[0], octets[1], octets[2],
                    octets[3], 0, 0,
                ];
                writer.write_image_data(&data)?;
            }
            IpAddr::V6(addr) => {
                let octets = addr.octets();
                let data = [
                    octets[0], octets[1], octets[2],
                    octets[3], octets[4], octets[5],
                    octets[6], octets[7], octets[8],
                    octets[9], octets[10], octets[11],
                    octets[12], octets[13], octets[14],
                    octets[15], 0, 0,
                ];
                writer.write_image_data(&data)?;
            }
        };
    };

    Ok(buffer.into_inner())
}

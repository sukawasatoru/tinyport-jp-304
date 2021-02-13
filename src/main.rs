/*
 * Copyright 2021 sukawasatoru
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use anyhow::Result as Fallible;
use structopt::StructOpt;
use tracing::info;

#[derive(StructOpt)]
struct Opt {
    /// Port number
    #[structopt(short, long, default_value = "38081")]
    port: u16,

    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
}

#[tokio::main]
async fn main() -> Fallible<()> {
    dotenv::dotenv().ok();
    let opt: Opt = Opt::from_args();
    setup_log(opt.verbose);

    info!("Hello");

    hyper::Server::bind(&([0, 0, 0, 0], opt.port).into())
        .serve(hyper::service::make_service_fn(|_| async {
            Ok::<_, hyper::Error>(hyper::service::service_fn(|req| async move {
                info!(?req, uri = ?req.uri());
                if req.method() != hyper::Method::GET {
                    info!("404");
                    return hyper::Response::builder()
                        .status(hyper::StatusCode::NOT_FOUND)
                        .body(hyper::Body::empty());
                }

                match req.uri().path() {
                    "/" => hyper::Response::builder()
                        .status(hyper::StatusCode::MOVED_PERMANENTLY)
                        .header(hyper::header::LOCATION, "https://sukawasatoru.com/")
                        .body(hyper::Body::empty()),
                    "/android-emulator" => hyper::Response::builder()
                        .status(hyper::StatusCode::MOVED_PERMANENTLY)
                        .header(
                            hyper::header::LOCATION,
                            "http://sukawasatoru.com/docs/android-emulator",
                        )
                        .body(hyper::Body::empty()),
                    "/android-things-raspberry-pi-3" => hyper::Response::builder()
                        .status(hyper::StatusCode::MOVED_PERMANENTLY)
                        .header(
                            hyper::header::LOCATION,
                            "http://sukawasatoru.com/docs/android-things-raspberry-pi-3",
                        )
                        .body(hyper::Body::empty()),
                    "/jenkins" => hyper::Response::builder()
                        .status(hyper::StatusCode::MOVED_PERMANENTLY)
                        .header(
                            hyper::header::LOCATION,
                            "http://sukawasatoru.com/docs/jenkins",
                        )
                        .body(hyper::Body::empty()),
                    "/surfacepro3-recovery" => hyper::Response::builder()
                        .status(hyper::StatusCode::MOVED_PERMANENTLY)
                        .header(
                            hyper::header::LOCATION,
                            "https://sukawasatoru.com/docs/surfacepro3-recovery",
                        )
                        .body(hyper::Body::empty()),
                    _ => hyper::Response::builder()
                        .status(hyper::StatusCode::NOT_FOUND)
                        .body(hyper::Body::empty()),
                }
            }))
        }))
        .await?;

    info!("Bye");

    Ok(())
}

fn setup_log(level: u8) {
    let builder = tracing_subscriber::fmt();
    match std::env::var(tracing_subscriber::EnvFilter::DEFAULT_ENV) {
        Ok(data) => {
            let builder = builder.with_env_filter(tracing_subscriber::EnvFilter::new(data));
            match level {
                0 => builder.init(),
                1 => builder.with_max_level(tracing::Level::DEBUG).init(),
                _ => builder.with_max_level(tracing::Level::TRACE).init(),
            }
        }
        Err(_) => match level {
            0 => builder.with_max_level(tracing::Level::INFO).init(),
            1 => builder.with_max_level(tracing::Level::DEBUG).init(),
            _ => builder.with_max_level(tracing::Level::TRACE).init(),
        },
    }
}

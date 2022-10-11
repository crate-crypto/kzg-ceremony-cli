mod args;
use args::CliArgs;
mod files;
use files::project_dir_check;
mod rest_api;
use rest_api::RestAPI;
mod constants;

mod commands;
use commands::{capture_entropy, contribute, login, logout, poll, receipt, user, view_transcript};

use crate::commands::status;
use clap::Parser;
use log::{error, LevelFilter};
use owo_colors::OwoColorize;
use std::io::Write;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format(|buf, record| match record.level() {
            log::Level::Error => {
                writeln!(buf, "[{}] : {}", record.level().red(), record.args())
            }
            log::Level::Warn => writeln!(
                buf,
                "[{}] : {}",
                record.level().yellow(),
                record.args().underline()
            ),
            log::Level::Info => {
                writeln!(
                    buf,
                    "[{}] : {}",
                    record.level().green(),
                    record.args().blue()
                )
            }
            log::Level::Debug => {
                writeln!(buf, "[{}] : {}", record.level().black(), record.args())
            }
            log::Level::Trace => writeln!(
                buf,
                "[{}] : {}",
                record.level().bright_magenta(),
                record.args()
            ),
        })
        .init();

    if !project_dir_check() {
        error!("could not create the project directory to store credentials");
        return;
    };

    let api = RestAPI::default();

    let args = CliArgs::parse();
    match args {
        CliArgs::Login => {
            login::cmd(&api).await;
        }
        CliArgs::Logout => logout::cmd(),
        CliArgs::Receipt => receipt::cmd(),
        CliArgs::User => user::cmd(),
        CliArgs::Status => {
            status::cmd(&api).await;
        }
        CliArgs::CaptureEntropy => capture_entropy::cmd(),
        CliArgs::Poll => {
            poll::cmd(&api).await;
        }
        CliArgs::ViewTranscript => view_transcript::cmd(&api),
        CliArgs::Contribute => {
            contribute::cmd(&api).await;
        }
    };
}

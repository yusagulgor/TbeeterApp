use wet_tweet_gpu;

use crate::gpu_menus::main_terminal_ui;

pub mod gpu_menus;
pub mod utils;

#[tokio::main]
async fn main() {
    main_terminal_ui().await;
}

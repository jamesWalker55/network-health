use blocking::unblock;
use futures_timer::Delay;
use iced::futures::sink::SinkExt;
use iced::futures::{future, Stream};
use iced::widget::{column, text, Column};
use iced::Center;
use iced::Subscription;
use iced::{stream, Color, Theme};
use std::net::Ipv4Addr;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PingStatus {
    /// Pinged successfully
    Success,
    /// Still pinging, taking a long time
    Warning,
    /// Ping failed / timeout
    Failed,
}

struct PingCell {
    status: PingStatus,
}

impl Default for PingCell {
    fn default() -> Self {
        Self::new()
    }
}

impl PingCell {
    fn new() -> Self {
        Self {
            status: PingStatus::Success,
        }
    }

    fn update(&mut self, new_status: PingStatus) {
        match new_status {
            PingStatus::Warning => {
                if self.status == PingStatus::Success {
                    self.status = new_status
                }
            }
            new_status => self.status = new_status,
        }
    }

    fn view(&self) -> Column<PingStatus> {
        let x = match self.status {
            PingStatus::Success => text("O").style(|_: &Theme| {
                let mut style = text::Style::default();
                style.color = Some(Color::from_rgb8(240, 240, 240));
                style
            }),
            PingStatus::Warning => text("?").style(|_: &Theme| {
                let mut style = text::Style::default();
                style.color = Some(Color::from_rgb8(252, 207, 3));
                style
            }),
            PingStatus::Failed => text("X").style(|_: &Theme| {
                let mut style = text::Style::default();
                style.color = Some(Color::from_rgb8(252, 40, 3));
                style
            }),
        };
        column![x.size(50)].padding(20).align_x(Center)
    }

    fn subscription(&self) -> Subscription<PingStatus> {
        Subscription::run(Self::subscription_worker)
    }

    fn subscription_worker() -> impl Stream<Item = PingStatus> {
        // stream::channel(100, |mut output| async move {
        //     loop {
        //         Delay::new(Duration::from_secs_f64(0.5)).await;
        //         output.send(PingStatus::Success).await.unwrap();
        //         Delay::new(Duration::from_secs_f64(0.5)).await;
        //         output.send(PingStatus::Warning).await.unwrap();
        //         Delay::new(Duration::from_secs_f64(0.5)).await;
        //         output.send(PingStatus::Failed).await.unwrap();
        //     }
        // })
        stream::channel(100, |mut output| async move {
            output.send(PingStatus::Success).await.unwrap();

            loop {
                println!("Start loop!");
                let loop_duration = Delay::new(Duration::from_secs(2));

                let ping_task = unblock(|| {
                    println!("  pinging...");
                    ping::ping(
                        std::net::IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
                        Some(Duration::from_secs(2)),
                        None,
                        None,
                        None,
                        None,
                    )
                });

                match future::select(ping_task, Delay::new(Duration::from_secs(1))).await {
                    // ping done before 1 second
                    future::Either::Left((ping_result, _)) => match ping_result {
                        Ok(_) => {
                            println!("  ping Success!");
                            output.send(PingStatus::Success).await
                        }
                        Err(_) => {
                            println!("  ping Failed!");
                            output.send(PingStatus::Failed).await
                        }
                    },
                    // 1 second passed, ping not done
                    future::Either::Right((_, ping_task)) => {
                        println!("  1 second passed...");
                        let _ = output.send(PingStatus::Warning).await;

                        match ping_task.await {
                            Ok(_) => {
                                println!("  ping Success!");
                                output.send(PingStatus::Success).await
                            }
                            Err(_) => {
                                println!("  ping Failed!");
                                output.send(PingStatus::Failed).await
                            }
                        }
                    }
                }
                .unwrap();

                // don't overload the pinging
                loop_duration.await;
            }
        })
    }
}

fn main() -> iced::Result {
    // fn main() {
    iced::application("Network Health", PingCell::update, PingCell::view)
        .subscription(PingCell::subscription)
        .run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_8888() {
        ping::ping(
            std::net::IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            Some(Duration::from_secs(2)),
            None,
            None,
            None,
            None,
        )
        .expect("failed to ping");
    }
}

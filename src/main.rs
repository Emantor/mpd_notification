extern crate mpd;
extern crate notify_rust;

use mpd::Client;
use mpd::idle::Subsystem;
use mpd::Idle;
use std::time::Duration;
use std::thread;
use notify_rust::Notification;

fn read_loop(mut conn: Client) {
    loop{
        let unavailable = "NA".to_string();
        let nullsong = mpd::Song::default();

        let notify_system = conn.wait(&[Subsystem::Player]);
        let currentsong = match conn.currentsong() {
            Ok(song) => song.unwrap_or(nullsong),
            Err(_) => break,
        };

        let status = match conn.status() {
            Ok(status) => status,
            Err(_) => break,
        };

        let playpause = match status.state {
            mpd::status::State::Play => "Play",
            mpd::status::State::Pause => "Pause",
            mpd::status::State::Stop => "Stop",
        };

        let time = match status.time {
            Some(v) => v,
            None => break,
        };

        let title = currentsong.title.unwrap_or(unavailable.clone());
        let artist = currentsong.tags.get("Artist").unwrap_or(&unavailable);
        let notification_title = playpause.to_string() + " " + &time.0.num_seconds().to_string() + "/" + &time.1.num_seconds().to_string();
        let body = artist.clone() + ": " + &title;

        for x in notify_system.unwrap() {
            if x == Subsystem::Player {
                Notification::new().body(&body).appname("MPD").summary(&notification_title).show().unwrap();
            }
        }
    }
}

fn main() {
    loop {
        let conn = Client::connect("127.0.0.1:6600");

        match conn {
            Ok(v) => read_loop(v),
            Err(t) => println!("Error: {}", t),
        }
        thread::sleep(Duration::from_secs(5));
    }
}

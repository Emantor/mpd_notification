extern crate mpd;
extern crate notify_rust;
extern crate time;

use mpd::Client;
use mpd::idle::Subsystem;
use mpd::Idle;
use std::thread;
use notify_rust::Notification;
use notify_rust::NotificationUrgency;
use time::Duration;

fn calculate_disp_time(times: (Duration,Duration)) -> std::string::String {
    let min_left = times.0.num_minutes();
    let min_comp = times.1.num_minutes();
    let sec_left = times.0.num_seconds() - 60 * min_left;
    let sec_comp = times.1.num_seconds() - 60 * min_comp;

    format!("{}:{:02}/{}:{:02}", min_left, sec_left, min_comp, sec_comp)

}

fn read_loop(mut conn: Client) {
    loop{
        let unavailable = "NA".to_string();
        let nullsong = mpd::Song::default();

        let _ = conn.wait(&[Subsystem::Player]);
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
            Some(v) => calculate_disp_time(v),
            None => break,
        };

        let title = currentsong.title.unwrap_or(unavailable.clone());
        let artist = currentsong.tags.get("Artist").unwrap_or(&unavailable);
        let notification_title = playpause.to_string() + " " + &time;
        let body = artist.clone() + ": " + &title;

        Notification::new().urgency(NotificationUrgency::Low).timeout(2).body(&body).appname("MPD").summary(&notification_title).show().unwrap();
    }
}

fn main() {
    loop {
        let conn = Client::connect("127.0.0.1:6600");

        match conn {
            Ok(v) => read_loop(v),
            Err(t) => println!("Error: {}", t),
        }
        thread::sleep(std::time::Duration::from_secs(5));
    }
}

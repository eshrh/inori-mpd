extern crate inori_mpd as mpd;

mod helpers;

#[test]
fn playback() {
    let mut mpd = helpers::connect();
    mpd.play().unwrap();
}

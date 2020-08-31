use std::fs::File;

pub async fn play_bell() {
    let (_stream, stream_handle) =
        rodio::OutputStream::try_default().expect("Must have output device");
    let source = File::open("./sound/bell.mp3").expect("Can't read ./sound/bell.mp3");
    let sink = stream_handle.play_once(source).expect("Must be played");
    sink.sleep_until_end();
}

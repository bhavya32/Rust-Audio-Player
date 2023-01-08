use soloud::*;
use std::env;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Ok(());
    }
    let play_type = &args[1];
    let play_content = &args[2];

    let sl = Soloud::default()?;

    if play_type == "text" {
        let mut speech = audio::Speech::default();
        speech.set_text(play_content)?;
        sl.play(&speech);
        while sl.active_voice_count() > 0 {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    else if play_type == "audio" {
        let mut wav = audio::Wav::default();
        wav.load(&std::path::Path::new(play_content))?;
        sl.play(&wav);
        while sl.voice_count() > 0 {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    Ok(())
}

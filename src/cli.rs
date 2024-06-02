use clap::{Command, Arg};

const VERSION_NUMBER: &str = "4.8.7";

pub fn build_cli() -> Command<'static> {
    Command::new("ani-cli")
        .version(VERSION_NUMBER)
        .about("Anime CLI")
        .arg(Arg::new("query").about("Anime query").required(false))
        .arg(Arg::new("continue").short('c').long("continue").about("Continue watching from history"))
        .arg(Arg::new("download").short('d').long("download").about("Download the video instead of playing it"))
        .arg(Arg::new("delete").short('D').long("delete").about("Delete history"))
        .arg(Arg::new("syncplay").short('s').long("syncplay").about("Use Syncplay to watch with friends"))
        .arg(Arg::new("select_nth").short('S').long("select-nth").about("Select nth entry").takes_value(true))
        .arg(Arg::new("quality").short('q').long("quality").about("Specify the video quality").takes_value(true))
        .arg(Arg::new("vlc").short('v').long("vlc").about("Use VLC to play the video"))
        .arg(Arg::new("version").short('V').long("version").about("Show the version of the script"))
        .arg(Arg::new("help").short('h').long("help").about("Show this help message and exit"))
        .arg(Arg::new("episode").short('e').long("episode").about("Specify the number of episodes to watch").takes_value(true))
        .arg(Arg::new("dub").long("dub").about("Play dubbed version"))
        .arg(Arg::new("rofi").long("rofi").about("Use rofi instead of fzf for the interactive menu"))
        .arg(Arg::new("skip").long("skip").about("Use ani-skip to skip the intro of the episode (mpv only)"))
        .arg(Arg::new("no_detach").long("no-detach").about("Don't detach the player (useful for in-terminal playback, mpv only)"))
        .arg(Arg::new("exit_after_play").long("exit-after-play").about("Exit the player, and return the player exit code (useful for non interactive scenarios, works only if --no-detach is used, mpv only)"))
        .arg(Arg::new("skip_title").long("skip-title").about("Use given title as ani-skip query").takes_value(true))
        .arg(Arg::new("nextep_countdown").short('N').long("nextep-countdown").about("Display a countdown to the next episode"))
        .arg(Arg::new("update").short('U').long("update").about("Update the script"))
}

pub fn version_info() {
    println!("{}", VERSION_NUMBER);
}

pub fn help_info() {
    println!("{}", build_cli().render_usage());
}

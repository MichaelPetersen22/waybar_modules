use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args{
    /// Icon to use with media player
    #[clap(short, long, value_parser, default_value = "")]
    pub icon: String,

    /// Media Player to get currently playing from; Provide in format --player=yourplayer
    #[clap(short, long, value_parser, default_value = "spotify")]
    pub media: String,
}
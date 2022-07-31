use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args{
    /// Longitude
    #[clap(long, value_parser, allow_hyphen_values(true), required=true)]
    pub latitude: f32,

    /// Latitude
    #[clap(long, value_parser, allow_hyphen_values(true), required=true)]
    pub longitude: f32,
}
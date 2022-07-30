use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args{
    /// Longitude
    #[clap(long, value_parser)]
    pub latitude: String,

    /// Latitude
    #[clap(long, value_parser,)]
    pub longitude: String,
}
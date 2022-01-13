use clap::Parser;
use pixelliarmus::{pixelliarmus, Operation};
use std::path::Path;

//Parse CLI thanks to the clap
#[derive(Parser)]
#[clap(author, version, about)]
struct Config {
    #[clap(short, long)]
    input: String,

    #[clap(short, long)]
    factor: u32,

    #[clap(short, long, default_value = "")]
    effect: String,

    #[clap(short, long, default_value = "true")]
    resize: String,

    #[clap(short, long, default_value = "")]
    output: String,

    #[clap(long, default_value = "nearest")]
    filter_type: String,
}

fn main() {
    //Parse arguments
    let cli = Config::parse();
    let name = cli.input.clone();
    let path = Path::new(&name);
    let extension = path.extension().unwrap().to_str().unwrap();

    //Create operation struct
    let img = Operation {
        name: cli.input,
        factor: cli.factor,
        resize: cli.resize,
        effect: cli.effect,
        output_file_name: cli.output,
        extension: extension.to_string(),
        filter_type: cli.filter_type,
    };

    //TODO
    //pixelliarmus function might return (ImageBuffer<Rgba<u8>>, Vec<u8>>, String(output_file_name))
    //With this way, parse_output doesn't have to be public

    //Do the magic
    let (pixelized_img, output_file_name) = pixelliarmus(img.clone());

    //Save the result
    let _result = pixelized_img.save(output_file_name);
}

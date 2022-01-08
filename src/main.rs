use clap::Parser;
use pixelliarmus::{Operation, pixelliarmus};
use std::path::Path;


#[derive(Parser)]
#[clap(author, version, about)]
struct Config{
    #[clap(short, long)]
    input: String,

    #[clap(short, long)]
    factor: u32,
    
    #[clap(short, long, default_value = "")]
    effect: String,

    #[clap(short, long, default_value = "true")]
    resize: String
}



fn main() {
    
    let cli = Config::parse();
    let fac = cli.factor;
    let name = cli.input.clone();
    let eff = cli.effect.clone();
    
    let path = Path::new(&name);
    let filename = path.file_stem().unwrap().to_str().unwrap();
    let extension = path.extension().unwrap().to_str().unwrap();

    let img = Operation{
        name: cli.input,
        factor: cli.factor,
        resize: cli.resize,
        effect: cli.effect
    };

    let pixelized_img = pixelliarmus(img);

    let output_file_name = format!("{}-{}-{}.{}",filename , fac, eff, extension );
    let _result = pixelized_img.save(output_file_name);
    
}

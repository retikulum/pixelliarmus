use clap::Parser;
use pixelliarmus::{Operation, pixelliarmus, parse_output};
use std::path::Path;

//Parse CLI thanks to the clap
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
    resize: String,

    #[clap(short, long, default_value="")]
    output: String
}



fn main() {
    
    //Parse arguments
    let cli = Config::parse();
    let name = cli.input.clone();
    let path = Path::new(&name);
    let extension = path.extension().unwrap().to_str().unwrap();


    //Create operation struct
    let img = Operation{
        name: cli.input,
        factor: cli.factor,
        resize: cli.resize,
        effect: cli.effect,
        output_file_name: cli.output,
        extension: extension.to_string()
    };

    //Do the magic
    let pixelized_img = pixelliarmus(img.clone());

    //parse output parameter
    let output_file_name = parse_output(img);
    
    //Save the result
    let _result = pixelized_img.save(output_file_name);
    
}

use image::{
    imageops::{self, grayscale, invert},
    GenericImageView, ImageBuffer, Luma, Pixel, Rgba,
};

//Operation struct
// It is used for storing data processing image
#[derive(Clone)]
pub struct Operation {
    pub name: String,
    pub factor: u32,
    pub resize: String,
    pub effect: String,
    pub output_file_name: String,
    pub extension: String,
    pub filter_type: String,
}

//I counldn't find luma_to_rgba functio so implement it
//Not sure if it is the best way
fn luma_to_rgba(
    greyscaled_image: ImageBuffer<Luma<u8>, Vec<u8>>,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut rgba_image = ImageBuffer::new(greyscaled_image.width(), greyscaled_image.height());

    for x in 0..greyscaled_image.width() {
        for y in 0..greyscaled_image.height() {
            let pixel = greyscaled_image.get_pixel(x, y);
            rgba_image.put_pixel(x, y, pixel.to_rgba());
        }
    }
    rgba_image
}

pub fn pixelliarmus(img: Operation) -> (ImageBuffer<Rgba<u8>, Vec<u8>>, String) {
    //Parse from struct
    let filename = img.name.clone();
    let factor = img.factor;
    let resize = img.resize.clone();
    let effect = img.effect.clone();
    let input_image = image::open(filename).unwrap();
    let (width, height) = input_image.dimensions();

    //Middle layers widt and height
    let new_width = width / factor;
    let new_height = height / factor;

    //Create empty buffer with new dimensions
    let mut pixelized_img = ImageBuffer::new(new_width, new_height);

    //Resize with the given factor
    for x in 0..pixelized_img.width() {
        for y in 0..pixelized_img.height() {
            let pixel = input_image.get_pixel(x * factor, y * factor);
            pixelized_img.put_pixel(x, y, pixel);
        }
    }

    let output_file_name = parse_output(img.clone());

    //Check the effects
    if effect.eq("greyscale") {
        let greyscaled_image = grayscale(&pixelized_img);
        pixelized_img = luma_to_rgba(greyscaled_image);
    } else if effect.eq("invert") {
        invert(&mut pixelized_img);
    } else if effect.eq("") {
    } else {
        panic!("Effect can't find or doesn't implemented yet");
    }

    let filter = parse_filter_type(img.filter_type.to_string());

    if resize.eq("false") {
        return (pixelized_img, output_file_name);
    } else if resize.eq("true") {
        pixelized_img = imageops::resize(&pixelized_img, width, height, filter);
    } else {
        let (resized_width, resized_height) = parse_resize(resize);
        pixelized_img = imageops::resize(&pixelized_img, resized_width, resized_height, filter);
    }

    (pixelized_img, output_file_name)
}

//Parse resize argument
//Argument should be in format of "widthxheight"
fn parse_resize(dimensions: String) -> (u32, u32) {
    let width;
    let height;
    let splitted: Vec<&str> = dimensions.split("x").collect();
    let width_res = splitted[0].parse::<u32>();

    //Handling errors
    match width_res {
        Ok(p) => width = p,
        Err(error) => panic!(
            "Problem while parsing width: Format should be \"widthxheight\" {:?}",
            error
        ),
    }

    let height_res = splitted[1].parse::<u32>();

    //Handling errors
    match height_res {
        Ok(p) => height = p,
        Err(error) => panic!(
            "Problem while parsing height: Format should be \"widthxheight\" {:?}",
            error
        ),
    }

    (width, height)
}

//parse output parameter
fn parse_output(cli: Operation) -> String {
    let output_file_name;
    if cli.output_file_name.eq("") {
        if cli.effect.eq("") {
            output_file_name = format!("{}-{}.{}", cli.name, cli.factor, cli.extension);
        } else {
            output_file_name = format!(
                "{}-{}-{}.{}",
                cli.name, cli.factor, cli.effect, cli.extension
            );
        }
    } else {
        output_file_name = cli.output_file_name.clone();
    }

    output_file_name
}

//Parse filter type parameter
fn parse_filter_type(filter_type: String) -> imageops::FilterType {
    let lower_filter_type = filter_type.to_lowercase();

    match lower_filter_type.as_str(){
        "nearest" => imageops::FilterType::Nearest,
        "triangle" => imageops::FilterType::Triangle,
        "catmullrom" => imageops::FilterType::CatmullRom,
        "gaussian" => imageops::FilterType::Gaussian,
        "lanczos3" => imageops::FilterType::Lanczos3,
        _ => panic!("This effect couldn't found. These are the available filter types: Nearest, Triangle, CatmullRom, Gaussian, Lanczos3")
    }
}

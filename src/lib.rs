use image::{GenericImageView, ImageBuffer, imageops::{self,  grayscale, invert}, Rgba, Pixel, Luma};

pub struct Operation{
    pub name: String,
    pub factor: u32,
    pub resize: String,
    pub effect: String,
}


fn luma_to_rgba(greyscaled_image: ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>>{
    let mut rgba_image =  ImageBuffer::new(greyscaled_image.width(), greyscaled_image.height());

    for x in 0..greyscaled_image.width(){
        for y in 0..greyscaled_image.height(){
            let pixel = greyscaled_image.get_pixel(x , y );
            rgba_image.put_pixel(x, y, pixel.to_rgba() );

        }
    }
    rgba_image
}

pub fn pixelliarmus(img: Operation) ->  ImageBuffer<Rgba<u8>, Vec<u8>>{

    
    let filename = img.name;
    let factor = img.factor;
    let resize = img.resize;
    let effect = img.effect;
    let img = image::open(filename).unwrap();
    let (width, height) = img.dimensions();

    let new_width = width / factor;
    let new_height = height / factor;
    let mut pixelized_img =  ImageBuffer::new(new_width, new_height);

    for x in 0..pixelized_img.width(){
        for y in 0..pixelized_img.height(){
            let pixel = img.get_pixel(x * factor, y * factor);
            pixelized_img.put_pixel(x, y, pixel );

        }
    }

    if effect.eq("greyscale"){
        let greyscaled_image = grayscale(&pixelized_img);
        pixelized_img = luma_to_rgba(greyscaled_image);
 

    }
    else if effect.eq("invert"){
        invert(&mut pixelized_img);

    }
    else if effect.eq(""){
        
    }
    else{
        panic!("Effect can't find or doesn't implemented yet");

    }

    if resize.eq("false"){
        return pixelized_img;
    }

    pixelized_img = imageops::resize(&pixelized_img, width, height, imageops::FilterType::Nearest);



    pixelized_img

}


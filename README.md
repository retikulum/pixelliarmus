# Pixelliarmus

This is a toy rust project that does Pixelliarmus spell on your pictures. In simpler terms, images are resized with the given factor and resized again to its original dimensions.

## Install and Usage

You can clone the repository

`git clone https://github.com/retikulum/pixelliarmus.git`

Run it with cargo:

`cargo run -- --input $path-of-the-image -f $factor -e $effect-name -r $resize`

Help:

`cargo run -- --help`

## Demo

There is a little cat image in images folder. You can do pixelliarmus spell on it.

`cargo run -- --input .\images\test.jpg -f 2`

Original photo:

![](/images/test.jpg)

Only pixelliarmus:

![](/images/test-2-.jpg)

Pixelliarmus + greyscale:

![](/images/test-2-greyscale.jpg)

Pixelliarmus + invert:

![](/images/test-2-invert.jpg)


## Future Work

- Research and implement other algorithms.
- User controlled output format.
- Resize image according to user input.
- Refactor code while learning it.
- Publish it as crate.
- Implement other effects.
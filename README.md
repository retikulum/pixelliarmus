# Pixelliarmus

This is a toy rust project that does Pixelliarmus spell on your pictures with effects. In simpler terms, images are resized with the given factor and resized again to its original dimensions.

## Install and Usage

You can clone the repository

`git clone https://github.com/retikulum/pixelliarmus.git`

Run it with cargo:

`cargo run -- --input $path-of-the-image -f $factor -e $effect-name -r $widthxheight`

Help:

`cargo run -- --help`

## Demo

There is a little cat image in images folder. You can do pixelliarmus spell on it.

`cargo run -- --input .\images\test.jpg -f 2`

Original photo:

![](/images/test.jpg)

Only Pixelliarmus:

`cargo run -- --input .\images\test.jpg -f 2`


![](/images/test-2-.jpg)

Pixelliarmus + Greyscale:

`cargo run -- --input .\images\test.jpg -f 2 --effect greyscale`


![](/images/test-2-greyscale.jpg)

Pixelliarmus + Invert:

`cargo run -- --input .\images\test.jpg -f 2 --effect invert`

![](/images/test-2-invert.jpg)

**Now it supports different filter types:**

Pixelliarmus + Gaussian:

`cargo run -- --input .\images\test.jpg -f 2 -o .\test.jpg  --filter-type gaussian `

![](/images/test-2-gaussian.jpg)

Pixelliarmus + Triangle:

`cargo run -- --input .\images\test.jpg -f 2 -o .\test.jpg  --filter-type triangle `

![](/images/test-2-triangle.jpg)

Pixelliarmus + Greyscale + Gaussian:

`cargo run -- --input .\images\test.jpg -f 2 -o .\test.jpg  -effect greyscale --filter-type gaussian `

![](/images/test-2-greyscale-gaussian.jpg)


## Future Work

- Research and implement other algorithms
- <del>User controlled output file<del>
- <del>Resize image according to user input<del>
- Refactor code while learning it
- Publish it as crate
- Create documentation
- <del>Implement other effects<del>
# img_combine

A utility to combine images into one image. 

```
$ cargo run path_to_config_file
```

```
// config file
OUTPUT = output.jpg
ROWS = 2
COLS = 2
HEIGHT = 400
WIDTH = 1000
BACKGROUND_COLOR = #FFFFFF
IMAGE = 0,0,cat.jpg
IMAGE = 0,1,cat2.jpg
IMAGE = 1,0,cat3.jpg
IMAGE = 1,1,cat4.png
```

See `examples` folder for more details.

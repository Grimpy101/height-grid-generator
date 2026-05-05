# Height Grid Generator

Creates a uniform grid with height information derived from an input point cloud. The point cloud can be in PLY or LAS/LAZ format.

The output can be either a PLY point cloud where each point is a grid vertex, or a custom format of the form:

```
x y z
x y z
x y z
...
```

The dimensions of the output grid are printed to the standard output.

Building the program:

```bash
cargo build --release
```

Running the program:

```bash
height-grid-generator [OPTIONS] --input-filepath <INPUT_FILEPATH> --output-filepath <OUTPUT_FILEPATH> --resolution <RESOLUTION> --format <FORMAT>
```

```
Options:
  -i, --input-filepath <INPUT_FILEPATH>    Filepath to the point cloud
  -o, --output-filepath <OUTPUT_FILEPATH>  Output filepath
  -r, --resolution <RESOLUTION>            Grid resolution (in meters)
  -b, --binary                             Determines whether the file should be binary or textual
  -f, --format <FORMAT>                    Format to write to (ply or custom)
  -h, --help                               Print help
  -V, --version                            Print version
```
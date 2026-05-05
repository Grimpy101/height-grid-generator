# Height Grid Generator

Creates a uniform grid with height information derived from an input point cloud.

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
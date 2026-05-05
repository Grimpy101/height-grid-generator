use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    mem,
    time::Instant,
};

use clap::Parser;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::point::Point;

mod arguments;
mod ply_files;
mod point;

pub struct Grid {
    pub x_count: usize,
    pub min_x: f32,
    pub min_y: f32,
    pub element_size: f32,
}

fn point_to_grid(p: &Point, grid: &Grid) -> usize {
    let xi = ((p.x - grid.min_x) / grid.element_size).round();
    let yi = ((p.y - grid.min_y) / grid.element_size).round();
    xi as usize + yi as usize * grid.x_count
}

fn write_output(f: File, z: &[f32], grid: &Grid, binary: bool) -> bool {
    let mut bad_writes = false;
    let mut f = BufWriter::new(f);

    if binary {
        for (i, z) in z.iter().enumerate() {
            let xi = i % grid.x_count;
            let yi = i / grid.x_count;

            let x = grid.min_x + xi as f32 * grid.element_size;
            let y = grid.min_y + yi as f32 * grid.element_size;

            let bytes: [u8; 12] = unsafe { mem::transmute([x, y, *z]) };
            if f.write_all(&bytes).is_err() {
                bad_writes = true;
                break;
            }
        }
    } else {
        for (i, z) in z.iter().enumerate() {
            let xi = i % grid.x_count;
            let yi = i / grid.x_count;

            let x = grid.min_x + xi as f32 * grid.element_size;
            let y = grid.min_y + yi as f32 * grid.element_size;
            if writeln!(f, "{} {} {}", x, y, *z).is_err() {
                bad_writes = true;
                break;
            }
        }
    }

    bad_writes
}

fn main() {
    let args = arguments::Args::parse();

    if !args.input_filepath.exists() || !args.input_filepath.is_file() {
        panic!(
            "{} is not a valid file",
            args.input_filepath.to_string_lossy()
        );
    }

    let parent_dir = args.output_filepath.parent().expect("No parent directory");
    fs::create_dir_all(parent_dir).expect("Could not create a parent directory");

    let extension = args
        .input_filepath
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or("".to_string());

    let start_time = Instant::now();

    let points = if extension == "ply" {
        ply_files::read_file(&args.input_filepath)
    } else if extension == "las" || extension == "laz" {
        let mut reader = las::Reader::from_path(&args.input_filepath).unwrap_or_else(|_| {
            panic!(
                "Could not read {} as LAS file",
                args.input_filepath.to_string_lossy()
            )
        });
        reader
            .points()
            .flatten()
            .map(|p| Point {
                x: p.x as f32,
                y: p.y as f32,
                z: p.z as f32,
            })
            .collect::<Vec<Point>>()
    } else {
        panic!(
            "{} is not a valid file extension for the file {}",
            extension,
            args.input_filepath.to_string_lossy()
        );
    };

    if points.is_empty() {
        println!("No points in the file, exiting...");
        return;
    }
    println!("Read cloud with {} points...", points.len());

    let first = points.first().unwrap();
    let [min_x, max_x, min_y, max_y] = points
        .par_iter()
        .fold(
            || [first.x, first.x, first.y, first.y],
            |mut acc, p| {
                acc[0] = acc[0].min(p.x);
                acc[1] = acc[1].max(p.x);
                acc[2] = acc[2].min(p.y);
                acc[3] = acc[3].max(p.y);
                acc
            },
        )
        .reduce(
            || [first.x, first.x, first.y, first.y],
            |a, b| {
                [
                    a[0].min(b[0]),
                    a[1].max(b[1]),
                    a[2].min(b[2]),
                    a[3].max(b[3]),
                ]
            },
        );

    let x_span = max_x - min_x;
    let y_span = max_y - min_y;
    let grid_element_size = args.resolution;

    let x_count = (x_span / grid_element_size).ceil().max(1.0) as usize;
    let y_count = (y_span / grid_element_size).ceil().max(1.0) as usize;

    let modified_span_x = x_count as f32 * grid_element_size;
    let modified_span_y = y_count as f32 * grid_element_size;

    let gap_x = modified_span_x - x_span;
    let gap_y = modified_span_y - y_span;

    let half_gap_x = gap_x / 2.0;
    let half_gap_y = gap_y / 2.0;

    let grid_min_x = min_x - half_gap_x + (grid_element_size / 2.0);
    let grid_min_y = min_y - half_gap_y + (grid_element_size / 2.0);

    println!(
        "Creating grid of size {}x{} ({} elements)...",
        x_count,
        y_count,
        x_count * y_count
    );

    let grid = Grid {
        x_count,
        min_x: grid_min_x,
        min_y: grid_min_y,
        element_size: grid_element_size,
    };

    let mut accumulator = vec![[0.0, 0.0]; x_count * y_count];
    for point in points.iter() {
        let grid_index = point_to_grid(point, &grid);
        accumulator[grid_index][0] += point.z;
        accumulator[grid_index][1] += 1.0;
    }
    let z = accumulator
        .iter()
        .map(|[a, b]| a / b.max(1.0))
        .collect::<Vec<f32>>();

    let f = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(args.output_filepath.clone())
        .unwrap_or_else(|_| {
            panic!(
                "Could not write the output {}",
                args.output_filepath.to_string_lossy()
            )
        });

    let bad_writes = if args.format == "custom" {
        write_output(f, &z, &grid, args.binary)
    } else if args.format == "ply" {
        ply_files::write_file(f, &z, &grid)
    } else {
        println!(
            "No valid format provided (should be ply or custom): {}",
            args.format
        );
        true
    };

    let end_time = Instant::now();

    println!("Done in {:.3} secs.", (end_time - start_time).as_secs_f32());

    if bad_writes {
        eprintln!("Failed to write some values, stopped prematurely!");
    }
}

use std::{
    fs,
    io::{self, BufWriter},
    path::Path,
};

use ply_rs::{
    parser,
    ply::{Addable, DefaultElement, ElementDef, Ply, PropertyDef},
    writer::Writer,
};

use crate::{Grid, point::Point};

pub fn read_file(filepath: &Path) -> Vec<Point> {
    let f = fs::File::open(filepath).expect("Could not read the file");
    let mut f = io::BufReader::new(f);

    let vertex_parser = parser::Parser::<Point>::new();
    let header = vertex_parser
        .read_header(&mut f)
        .expect("Could not read PLY header");
    let mut point_list = Vec::new();
    for (_key, element) in &header.elements {
        if element.name == "vertex" {
            point_list = vertex_parser
                .read_payload_for_element(&mut f, element, &header)
                .expect("Could not read PLY payload");
        }
    }
    point_list
}

pub fn write_file(f: fs::File, accumulator: &[f32], grid: &Grid) -> bool {
    let mut ply = Ply::<DefaultElement>::new();
    ply.header.encoding = ply_rs::ply::Encoding::BinaryLittleEndian;

    let mut vertex_def = ElementDef::new("vertex".to_string());
    for name in ["x", "y", "z"] {
        vertex_def.properties.add(PropertyDef::new(
            name.to_string(),
            ply_rs::ply::PropertyType::Scalar(ply_rs::ply::ScalarType::Float),
        ));
    }
    ply.header.elements.add(vertex_def);

    let mut vertices = Vec::new();
    for (i, z) in accumulator.iter().enumerate() {
        let xi = i % grid.x_count;
        let yi = i / grid.x_count;

        let x = grid.min_x + xi as f32 * grid.element_size;
        let y = grid.min_y + yi as f32 * grid.element_size;

        let mut vertex = DefaultElement::new();
        vertex.insert("x".to_string(), ply_rs::ply::Property::Float(x));
        vertex.insert("y".to_string(), ply_rs::ply::Property::Float(y));
        vertex.insert("z".to_string(), ply_rs::ply::Property::Float(*z));
        vertices.push(vertex);
    }

    ply.header.elements.get_mut("vertex").unwrap().count = vertices.len();
    ply.payload.insert("vertex".to_string(), vertices);

    let mut f = BufWriter::new(f);
    let writer = Writer::new();
    writer
        .write_ply(&mut f, &mut ply)
        .expect("Failed to write PLY");

    false
}

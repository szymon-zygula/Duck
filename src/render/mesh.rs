use crate::primitives::vertex::{DuckVertex, SimpleVertex, Vertex};
use nalgebra::{Point3, Vector2, Vector3};

#[derive(Debug)]
pub struct ParseError;

pub struct Triangle([u32; 3]);

pub struct Mesh<V: Vertex> {
    pub vertices: Vec<V>,
    pub triangles: Vec<Triangle>,
}

impl Mesh<DuckVertex> {
    pub fn from_file(path: &std::path::Path) -> Self {
        let path_string = path.to_str().expect("Cannot convert path to string");
        let error_msg = format!("Could not load mesh at {}", path_string);
        let string = std::fs::read_to_string(path).expect(&error_msg);
        Self::parse_model(&string).expect("Error parsing model file")
    }

    pub fn parse_model(string: &str) -> Result<Self, ParseError> {
        let mut lines = string.lines();

        let vertex_count = Self::parse_u32(lines.next().ok_or(ParseError {})?)?;
        let mut vertices = Vec::new();
        for _ in 0..vertex_count {
            vertices.push(Self::parse_vertex(lines.next().ok_or(ParseError {})?)?);
        }

        let triangle_count = Self::parse_u32(lines.next().ok_or(ParseError {})?)?;
        let mut triangles = Vec::new();
        for _ in 0..triangle_count {
            triangles.push(Self::parse_triangle(lines.next().ok_or(ParseError {})?)?);
        }

        Ok(Self {
            triangles,
            vertices,
        })
    }

    pub fn parse_u32(string: &str) -> Result<u32, ParseError> {
        string.parse().map_err(|_| ParseError {})
    }

    pub fn parse_vertex(string: &str) -> Result<DuckVertex, ParseError> {
        let nums: Vec<_> = string.split(' ').flat_map(|s| s.parse()).collect();
        if nums.len() != 8 {
            return Err(ParseError {});
        }

        Ok(DuckVertex {
            position: Point3::new(nums[0], nums[1], nums[2]),
            normal: Vector3::new(nums[3], nums[4], nums[5]),
            tex: Vector2::new(nums[6], nums[7]),
        })
    }

    pub fn parse_triangle(string: &str) -> Result<Triangle, ParseError> {
        let nums: Vec<_> = string.split(' ').flat_map(Self::parse_u32).collect();
        if nums.len() != 3 {
            return Err(ParseError {});
        }

        Ok(Triangle([nums[0], nums[1], nums[2]]))
    }
}

impl Mesh<SimpleVertex> {
    pub fn rect() -> Self {
        let vertices = vec![
            SimpleVertex(Point3::new(0.0, 0.0, 0.0)),
            SimpleVertex(Point3::new(1.0, 0.0, 0.0)),
            SimpleVertex(Point3::new(1.0, 0.0, 1.0)),
            SimpleVertex(Point3::new(0.0, 0.0, 1.0)),
        ];

        let triangles = vec![Triangle([0, 1, 2]), Triangle([0, 2, 3])];

        Self {
            vertices,
            triangles,
        }
    }
}

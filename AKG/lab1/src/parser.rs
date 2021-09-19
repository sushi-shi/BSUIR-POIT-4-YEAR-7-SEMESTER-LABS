use nom::{
    IResult,
    multi::{many0, many1, many_m_n},
    character::complete::{space1, i64, not_line_ending, line_ending, space0},
    bytes::complete::{tag},
    number::complete::{double},
    combinator::{opt, eof, not, fail},
};

use crate::types::{
    Vertex,
    VertexTexture,
    VertexNormal,
    Face,
    ObjFile,
};

pub fn parse_vertex(input: &str) -> IResult<&str, Vertex> {
    let (input, _) = tag("v")(input)?; 

    let (input, coords) = many_m_n(3, 4, |input| {
        let (input, _) = space1(input)?;
        double(input)
    })(input)?;
    Ok((input, match coords[..] {
        [x, y, z, w] => Vertex::new(x, y, z, w),
        [x, y, z] => Vertex::new(x, y, z, 1.0),
        _ => panic!(),
    }))
}

pub fn parse_vertex_texture(input: &str) -> IResult<&str, VertexTexture> {
    let (input, _) = tag("vt")(input)?; 

    let (input, coords) = many_m_n(1, 3, |input| {
        let (input, _) = space1(input)?;
        double(input)
    })(input)?;
    Ok((input, match coords[..] {
        [u] => VertexTexture::new(u, 1.0, 1.0),
        [u, v] => VertexTexture::new(u, v, 1.0),
        [u, v, w] => VertexTexture::new(u, v, w),
        _ => panic!(),
    }))
}

pub fn parse_vertex_normal(input: &str) -> IResult<&str, VertexNormal> {
    let (input, _) = tag("vn")(input)?; 

    let (input, coords) = many_m_n(3, 3, |input| {
        let (input, _) = space1(input)?;
        double(input)
    })(input)?;
    Ok((input, match coords[..] {
        [i, j, k] => VertexNormal::new(i, j, k),
        _ => panic!(),
    }))
}

pub fn parse_face(input: &str) -> IResult<&str, Face> {
    let (input, _) = tag("f")(input)?; 

    let (input, coords) = many_m_n(3, usize::MAX, |input| {
        let (input, _) = space1(input)?;
        let (input, vertex_id) = i64(input)?;

        let (input, _) = opt(tag("/"))(input)?;
        let (input, texture_id) = opt(i64)(input)?;

        let (input, _) = opt(tag("/"))(input)?;
        let (input, normal_id) = opt(i64)(input)?;

        Ok((input, (vertex_id, texture_id, normal_id)))
    })(input)?;
    
    Ok((input, Face::new(coords)))
}

pub fn parse_comment(input: &str) -> IResult<&str, &str> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("#")(input)?; 

    not_line_ending(input)
}

pub fn parse_obj_file(input: &str) -> IResult<&str, ObjFile> {
    let mut verticies = Vec::new();
    let mut textures = Vec::new();
    let mut normals = Vec::new();
    let mut faces = Vec::new();

    let (input, _) = many1(|input| {
        not(eof)(input)?;

        let (input, line) = not_line_ending(input)?;

        if let ("", Some(vertex)) = opt(parse_vertex)(line)? {
            verticies.push(vertex);
        } else if let ("", Some(texture)) = opt(parse_vertex_texture)(line)? {
            textures.push(texture);
        } else if let ("", Some(normal)) = opt(parse_vertex_normal)(line)? {
            normals.push(normal);
        } else if let ("", Some(face)) = opt(parse_face)(line)? {
            faces.push(face.concrete(&verticies, &textures, &normals));
        } else if let ("", Some(_)) = opt(parse_comment)(line)? {
            // Ignore comments
        } else {
            eprintln!("Couldn't parse a line: {}", line);
        };

        let (input, _) = many0(line_ending)(input)?;
        Ok((input, ()))
    })(input)?;

    if verticies.len() == 0 || faces.len() == 0 { let _: (&str, ()) = fail(input)?; };

    Ok((input, ObjFile {
        verticies,
        textures,
        normals,
        faces,
    }))
}

#[cfg(test)]
mod test {
    use super::*;

    fn parser(s: &str) -> IResult<&str, Vec<&str>> {
        many1(tag("abc"))(s)
    }

    #[test]
    fn parsing() {
        let input: &str = "#One\n#Two";
        let parser = |input| -> IResult<&str, &str> { 
            not_line_ending(input)
        };
        let (leftovers, rightovers) = parser(input).unwrap(); 
        assert_eq!(leftovers, "\n#Two");
        assert_eq!(rightovers, "#One");


        let input = "v 0.32 0.33 0.43\n";
        let (leftovers, obj) = parse_obj_file(input).expect("Is parsing");
        assert_eq!(obj.verticies[0], Vertex::new(0.32, 0.33, 0.43));
        assert_eq!(leftovers, "");

        let input = "v 0.32 0.33 0.43";
        let (leftovers, obj) = parse_obj_file(input).expect("Is parsing");
        assert_eq!(obj.verticies[0], Vertex::new(0.32, 0.33, 0.43));
        assert_eq!(leftovers, "");


        let input = " #FIRST COMMENT\nv 0.32 0.33 0.43\n\n\nf 1 1 1\n# Comment\n";
        let (leftovers, obj) = parse_obj_file(input).expect("Is parsing");
        assert_eq!(obj.verticies[0], Vertex::new(0.32, 0.33, 0.43));
        assert_eq!(leftovers, "");


        let input = std::fs::read_to_string("./example/pyramid.obj").unwrap();
        assert!(parse_obj_file(&input).is_ok());

        let input = std::fs::read_to_string("./example/maya.obj").unwrap();
        assert!(parse_obj_file(&input).is_ok());
    }
}

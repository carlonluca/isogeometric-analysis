/*
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2022.02.06
 *
 * Copyright (c) 2021 Luca Carlon. All rights reserved.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 * 
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 */

use crate::bezier::BezierSurf;
use crate::bezier::BezierFactory;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

///
/// Parser a line containing a patch.
/// 
fn parse_patch(line: &String) -> [usize; 16] {
    let v = line.split(",").collect::<Vec<&str>>();
    let mut ret = [0usize; 16];
    let mut index = 0usize;
    for i in v {
        ret[index] = i.parse().unwrap();
        index += 1;
    }
    ret
}

///
/// Parses a line containing a vertex.
/// 
fn parse_vertex(line: &String) -> [f64; 3] {
    let v = line.split(",").collect::<Vec<&str>>();

    //{
    //    let mut iter = v.iter();
    //    log::info!("[ {:+.9}, {:+.9}, {:+.9} ],",
    //iter.next().unwrap().parse::<f64>().unwrap(),
    //iter.next().unwrap().parse::<f64>().unwrap(),
    //iter.next().unwrap().parse::<f64>().unwrap());
    //}

    let mut iter = v.iter();
    [
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap()
    ]
}

///
/// Reads a file containing bezier patches into usable structures.
/// 
pub fn read_utah_format(file_path: String) -> Result<Vec<BezierSurf<3>>, ()> {
    let file = File::open(file_path);
    match file {
        Err(_) => { return Err(()); }
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut lines = reader.lines();

            // Number of patches.
            let patch_count;
            match lines.next() {
                None => { return Err(()); }
                Some(line) => { patch_count = line.unwrap().parse().unwrap(); }
            }

            // Parse patches.
            let mut patches = Vec::<[usize; 16]>::new();
            for _ in 0..patch_count {
                match lines.next() {
                    None => { return Err(()); }
                    Some(line) => { patches.push(parse_patch(&line.unwrap())); }
                }
            }

            // Number of vertices.
            let vertex_count;
            match lines.next() {
                None => { return Err(()); }
                Some(line) => { vertex_count = line.unwrap().parse().unwrap(); }
            }

            // Parse vertices.
            let mut vertices = Vec::<[f64; 3]>::new();
            for _ in 0..vertex_count {
                match lines.next() {
                    None => { return Err(()); }
                    Some(line) => { vertices.push(parse_vertex(&line.unwrap())); }
                }
            }

            return Ok(BezierFactory::from_indexed_vertices(patches, vertices));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bezier::read_utah_format;

    #[test]
    fn test_eq() {
        assert_eq!(read_utah_format("utah_teapot_data/teapot".to_string()).unwrap().len(), 32);
        assert_eq!(read_utah_format("utah_teapot_data/teacup".to_string()).unwrap().len(), 26);
        assert_eq!(read_utah_format("utah_teapot_data/teaspoon".to_string()).unwrap().len(), 16);
    }
}

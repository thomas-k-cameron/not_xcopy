use std::fs;
use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::path;

struct ErrDetail {
    source: String,
    err: io::Error
}

fn main() {
    let source = "your source directory";
    let dest = "your destination directory";

    let result = "";
    let file = fs::File::create(result);
    let mut r_file = file.unwrap();

    let mut dir_stack = VecDeque::new();
    dir_stack.push_back(source.to_string());

    let mut err_stack = vec![];
    let mut len_match = vec![];
    'main: while let Some(i) = dir_stack.pop_back() {
        let source = i;
        let dest = format!("{}/{}", dest, source.replace("C:/", "C/"));
        let i = source.to_string();
        
        let path = path::Path::new(i.as_str());
        if path.is_dir() {
            let path = path::Path::new(dest.as_str());
            if !path.is_dir() {
                let _ = fs::create_dir_all(dest.as_str());
            }
            let readdir = match fs::read_dir(i.as_str()) {
                Ok(i) => i,
                Err(e) => {
                    let err = ErrDetail{
                        source: source.to_string(),
                        err: e
                    };
                    err_stack.push(err);
                    continue;
               }
            };
            for i in readdir {
                let dir = match i {
                    Ok(i) => i,
                    Err(err) => {
                        let e = ErrDetail{
                            source: source.to_string(),
                            err
                        };
                        err_stack.push(e);
                        continue;
                    }
                };
                
                match dir.file_name().to_str() {
                    Some(i) => {
                        let item = format!("{}/{}", source, i);
                        dir_stack.push_back(item);
                    },
                    _ => continue
                }
            }
            continue;
        } else {
            println!("{}", i);
        }

        let s_file = match fs::File::open(source.as_str()) {
            Ok(x) => x,
            Err(err) => {
                let err = ErrDetail {
                    source: source.to_string(),
                    err
                };
                err_stack.push(err);
                continue;
            }
        };
        
        
        let mut ops = fs::OpenOptions::new();
        let mut d_file = match ops.write(true).create(true).open(dest) {
            Ok(x) => x,
            Err(e) => {
                let err = ErrDetail {
                    source: source.to_string(),
                    err: e
                };
                err_stack.push(err);
                continue;
            }
        };
    
        'inner: for (s, d) in s_file.metadata().iter().zip(d_file.metadata().iter()) {
            if s.is_dir() {
                continue 'main;
            } else if s.len() == d.len() {
                len_match.push(source.to_string());
            } else {
                break 'inner;
            }
            continue 'main;
        }
    
        let mut reader = io::BufReader::new(s_file);
        loop {
            let buf_len = {
                let i = match reader.fill_buf() {
                    Ok(i) => i,
                    Err(err) => {
                        let ed = ErrDetail {
                            source: source.to_string(),
                            err
                        };
                        err_stack.push(ed);
                        break;
                    }
                };
    
                if i.len() == 0 {
                    break;
                }
    
                let size = match d_file.write(i) {
                    Ok(i) => i,
                    Err(err) => {
                        err_stack.push(ErrDetail {
                            source: source.to_string(),
                            err
                        });
                        break;
                    }
                };

                if size != i.len() {
                    println!("size mis-match {:?}", (size, i.len()));
                    break
                }

                i.len()
            };
            
            reader.consume(buf_len);
        }
    }

    
    for i in err_stack {
        let row = format!("{}\t{}\n", i.err.to_string(), i.source.to_string());
        r_file.write(row.as_bytes());
    }
    for i in dir_stack {
        r_file.write(i.as_bytes());
        r_file.write(b"\n");
    }
}

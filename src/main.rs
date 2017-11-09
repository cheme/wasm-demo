
#![feature(link_args)]

//extern crate webplatform;
extern crate itertools;
extern crate serde_json;
// extern crate openssl; fail as cross compile without target lib
#[macro_use] extern crate serde_derive;
use itertools::Itertools;
use std::os::raw::c_char;
use std::ffi::CString;
use std::ffi::CStr;
use std::collections::HashMap;
use Direction::*;
use std::fs::File;
use std::io::Result as IoResult;
use std::io::Write;
use std::net::{TcpListener};
//use openssl::hash::MessageDigest;

#[cfg_attr(target_arch="wasm32", link_args = "\
    --js-library site/utilities.js\
")]
#[cfg_attr(target_arch="asmjs", link_args = "\
    --js-library site/utilities.js\
")]

extern {
    fn get_data2() -> *mut c_char;
}

fn get_data_safe() -> String {
    let data = unsafe {
        CStr::from_ptr(get_data2())
    };
    data.to_string_lossy()
        .into_owned()
}
/* fail as any cross compilation
pub fn link_lib() {
 let handle_digest = MessageDigest::sha1();
 println!("{:?}",handle_digest.as_ptr());
}*/
// to call from js (use cwrap for returned string)
#[no_mangle]
pub fn get_data() -> *mut c_char {
    let mut data = HashMap::new();
    data.insert("Alice", "send");
    data.insert("Bob", "recieve");
    data.insert("Carol", "intercept");
    
    let descriptions = data.iter()
        .map(|(p,a)| format!("{} likes to {} messages", p, a))
        .collect::<Vec<_>>();

    CString::new(descriptions.join(", "))
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub fn write_tofile(dest : *mut c_char) -> *mut c_char {
/*  let p = unsafe {CStr::from_ptr(dest).to_string_lossy().into_owned()};
    CString::new(p)
        .unwrap()
        .into_raw()
        */

  match write_str_tofileinner(dest) {
    Ok(_) => CString::new("ok").unwrap().into_raw(),
    Err(e) => CString::new(format!("{}",e)).unwrap().into_raw(),
  }
}

fn write_str_tofileinner(dest : *mut c_char) -> IoResult<File> {
  // copy in rust string
  let path = unsafe {CStr::from_ptr(dest).to_string_lossy().into_owned()};
  let mut file = File::create(path)?;
  file.write_all(b"Hello, world!")?;
  file.flush()?;
  Ok(file)
/* 
let get_dat2 = Module.cwrap('write_tofile', 'string', ['string']);
emcc create false filesystem (cf site.js)
{root: FS.c…e.FS.FSNode, mounts: Array(0), devices: Array(16641), streams: Array(4), nextInode: 21, …}
DB_NAME
:
ƒ ()
DB_STORE_NAME
:
"FILE_DATA"
DB_VERSION
:...
Note that it is not a proper index db but seems in memory only (for my conf at least (chromium,firefox august 2017)), and does not persist a F5 call TODO plug it on indexdb
*/
}

#[derive(Serialize,Deserialize,Debug,PartialEq,Eq,Hash)]
enum Direction { North, South, East, West }

#[derive(Serialize, Deserialize, Debug)]
struct Example {
    favorite_animal: String,
    favorite_direction: Direction
}
pub fn test_tcp() {
// Try binding a tcp receiver -> panic into no protocol (as expected)
let listener = TcpListener::bind("127.0.0.1:80").unwrap();

// accept connections and process them serially
listener.accept().unwrap();

    println!("received");
}
fn main() {

    let mut users_facing = HashMap::new();
    users_facing.insert("Alice", North);
    users_facing.insert("Bob", South);
    users_facing.insert("Carol", East);

    let users_not_facing_north = users_facing.iter()
        .filter(|&(_, d)| *d != North)
        .collect::<HashMap<_,_>>();
    println!("{:?}", users_not_facing_north);


    let directions = vec![North, North, South, East, West, West];

    let unique_directions = directions.iter()
        .unique()
        .collect::<Vec<_>>();
    println!("{:?}", unique_directions);

    let data = r#" { "favorite_animal": "Bear", "favorite_direction": "North" } "#;
    let parsed: Example = serde_json::from_str(data).unwrap();
    println!("Parsed : {:?}", parsed);
    let data = get_data_safe();
    println!("Get data from js : {:?}", data);

    // wp test
/*    let document = webplatform::init();
    let body = document.element_query("body")
        .unwrap();
    body.html_append("\
        <h1>This header brought to you by Rust</h1>\
        <button>Click me!</button>\
    ");
    
    let button = document.element_query("button")
        .unwrap();
    button.on("mouseenter", move |_| {
        println!("Mouse entered!");
        body.html_append("<p>Mouse entered!</p>");
    });
    button.on("click", |_| {
        println!("Clicked!");
        webplatform::alert("Clicked!");
    });

    webplatform::spin();*/
}

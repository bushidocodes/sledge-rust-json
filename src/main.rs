use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug)]
struct ModuleC {
    name: [c_char; 32],
    path: [c_char; 256],
    port: u32,
}

#[derive(Deserialize, Serialize, Debug)]
struct Module {
    name: String,
    path: String,
    port: u32,
    #[serde(default)]
    #[serde(rename(deserialize = "expected-execution-us"))]
    expected_execution_us: u64,
    #[serde(rename(deserialize = "admissions-percentile"))]
    admissions_percentile: u32,
    #[serde(rename(deserialize = "relative-deadline-us"))]
    relative_deadline_us: u64,
    #[serde(rename(deserialize = "http-req-size"))]
    http_req_size: u64,
    #[serde(rename(deserialize = "http-resp-size"))]
    http_resp_size: u64,
    #[serde(rename(deserialize = "http-resp-content-type"))]
    http_resp_content_type: String,
}

fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
    [
        {
            "name": "html",
            "path": "html.wasm.so",
            "port": 1337,
            "expected-execution-us": 10000000,
            "admissions-percentile": 70,
            "relative-deadline-us": 20000000,
            "http-req-size": 1024,
            "http-resp-size": 102400,
            "http-resp-content-type": "text/html"
        }
    ]
    "#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Vec<Module> = serde_json::from_str(data)?;

    let mut output = ModuleC {
        name: [0; 32],
        path: [0; 256],
        port: 0,
    };

    unsafe {
        libc::memcpy(
            output.name.as_mut_ptr() as *mut libc::c_void,
            p[0].name.as_ptr() as *const libc::c_void,
            if p[0].name.len() < 32 {
                p[0].name.len()
            } else {
                32
            },
        );
        libc::
        libc::memcpy(
            output.path.as_mut_ptr() as *mut libc::c_void,
            p[0].path.as_ptr() as *const libc::c_void,
            if p[0].path.len() < 256 {
                p[0].path.len()
            } else {
                256
            },
        );
    }

    // Do things just like with any other Rust data structure.
    println!("{:?}", p[0]);
    output.port = p[0].port;
    println!("{:?}", output);

    Ok(())
}

fn main() {
    typed_example();
    println!("Hello, world!");
}

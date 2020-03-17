use jsonrpc_http_server::jsonrpc_core::Params;
use serde::{Deserialize, Serialize};
use sysinfo::SystemExt;

#[derive(Serialize, Deserialize)]
struct Sysinfo {
    // The 'a defines a lifetime
    total_mem: u64,
    use_mem: u64,
    total_swap: u64,
    use_swap: u64,
}

pub fn author() -> String {
    let author = "Neil".to_owned();
    return author;
}

pub fn healthz() -> String {
    let system = sysinfo::System::new_all();
    let system_info = Sysinfo {
        total_mem: system.get_total_memory(),
        use_mem: system.get_used_memory(),
        total_swap: system.get_total_swap(),
        use_swap: system.get_used_swap(),
    };
    let resp = serde_json::to_string(&system_info).unwrap();
    resp
}

// pub fn add(params: Params) -> String {
//     if let Params::Array(a) = params {
//         println!("Totle {} elements: {:?}", a.len(), a);
//         a[0].is_i64();
//         a[1].is_i64();
//         let result = a[0] + a[1];
//     };
//     let resp = serde_json::to_string(&result).unwrap();
//     resp
// }

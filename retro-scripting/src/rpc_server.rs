use core::panic;
use std::collections::HashMap;
use serde_json;
use jsonrpc_http_server::jsonrpc_core::{IoHandler, Params};

type Request = HashMap<String, HashMap<String, usize>>;

pub fn handler(core_get_memory_data: fn(usize) -> u8, core_get_rom_data: fn(usize) -> u8) {
    let mut io = IoHandler::default();
    io.add_sync_method("read_memory", move |params: Params| {
        // println!("read_memory: {:?}", params);

        params.parse::<Request>()
        .map(|request| {
            request.into_iter()
            .map(|(k, addresses)| {
                match k.as_str() {
                    "u8" => {
                        let addresses = addresses.into_iter()
                        .map(|(name, address)| {
                            let value = core_get_memory_data(address);
                            (name, value as u64)
                        })
                        .collect::<HashMap<String, u64>>();

                        (k, addresses)
                    },
                    "u16le" => {
                        let addresses = addresses.into_iter()
                        .map(|(name, address)| {
                            let l = core_get_memory_data(address) as u16;
                            let r = (core_get_memory_data(address + 1) as u16) << 8;
                            let value = l + r;
                            (name, value as u64)
                        })
                        .collect::<HashMap<String, u64>>();

                        (k, addresses)
                    },
                    _ => panic!()
                }
            })
            .collect::<HashMap<String, HashMap<String, u64>>>()
        })
        .map(|response| { serde_json::to_value(response).unwrap() })
    });

    io.add_sync_method("read_rom", move |params: Params| {
        params.parse::<Request>()
        .map(|request| {
            request.into_iter()
            .map(|(k, addresses)| {
                match k.as_str() {
                    "u8" => {
                        let addresses = addresses.into_iter()
                        .map(|(name, address)| {
                            let value = core_get_rom_data(address);
                            (name, value as u64)
                        })
                        .collect::<HashMap<String, u64>>();

                        (k, addresses)
                    },
                    _ => panic!()
                }
            })
            .collect::<HashMap<String, HashMap<String, u64>>>()
        })
        .map(|response| { serde_json::to_value(response).unwrap() })
    });

    let server = jsonrpc_http_server::ServerBuilder::new(io)
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .expect("Unable to start RPC server");

    // println!("RPC server started on 127.0.0.1:3030");

    server.wait();
}

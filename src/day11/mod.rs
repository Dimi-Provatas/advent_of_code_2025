use std::collections::HashMap;

use util::Server;

use crate::util::read_file_to_lines;

mod util;

pub fn part1(filename: &str) -> usize {
    let lines = read_file_to_lines(filename);
    let servers = lines.iter().map(Server::from).collect::<Vec<Server>>();
    let servers_map = servers
        .iter()
        .map(|s| (s.name.clone(), s.clone()))
        .collect::<HashMap<String, Server>>();

    let init_server = servers_map.get("you").unwrap();
    init_server.server_path_count(&servers_map)
}

pub fn part2(filename: &str) -> usize {
    let lines = read_file_to_lines(filename);
    let servers = lines.iter().map(Server::from).collect::<Vec<Server>>();
    let servers_map = servers
        .iter()
        .map(|s| (s.name.clone(), s.clone()))
        .collect::<HashMap<String, Server>>();

    let init_server = servers_map.get("svr").unwrap();
    init_server.server_path_dac_fft_count(&servers_map)
}

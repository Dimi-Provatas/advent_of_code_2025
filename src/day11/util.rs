use std::collections::HashMap;

#[derive(Clone)]
pub struct Server {
    pub name: String,
    pub paths: Vec<String>,
    pub has_out: bool,
}
impl From<&String> for Server {
    fn from(value: &String) -> Self {
        let split = value.split(" ").collect::<Vec<&str>>();
        let mut split = split.iter();

        let name = split.next().cloned().unwrap();
        if !name.ends_with(':') {
            panic!("name was not the first element");
        }
        let name = name.replace(':', "");

        let mut has_out = false;
        let mut paths = Vec::new();
        while let Some(path) = split.next().cloned() {
            if path == "out" {
                has_out = true;
            }
            paths.push(path.to_string());
        }

        Self {
            name,
            paths,
            has_out,
        }
    }
}
impl Server {
    pub fn server_path_count(&self, servers_map: &HashMap<String, Server>) -> usize {
        assert_eq!(self.name, "you".to_string());
        let mut res = 0;
        self.path_has_out(servers_map, &mut res);
        res
    }

    fn path_has_out(&self, servers_map: &HashMap<String, Server>, current_count: &mut usize) {
        if self.has_out {
            *current_count += 1;
            return;
        }

        for node in &self.paths {
            let server = servers_map.get(node).unwrap();
            server.path_has_out(servers_map, current_count);
        }
    }

    pub fn server_path_dac_fft_count(&self, servers_map: &HashMap<String, Server>) -> usize {
        assert_eq!(self.name, "svr");

        let mut cache: HashMap<(String, u8), usize> = HashMap::new();
        let start_mask = 0u8;

        let mut res = 0;
        for node in &self.paths {
            let server = servers_map.get(node).unwrap();
            res += server.walk_count_cached(servers_map, start_mask, &mut cache);
        }

        res
    }

    fn walk_count_cached(
        &self,
        servers_map: &HashMap<String, Server>,
        mask: u8,
        cache: &mut HashMap<(String, u8), usize>,
    ) -> usize {
        if let Some(&v) = cache.get(&(self.name.to_string(), mask)) {
            return v;
        }

        let mut cur_mask = mask;
        if &self.name == "fft" {
            cur_mask |= 1;
        }
        if &self.name == "dac" {
            cur_mask |= 2;
        }

        let res = if self.has_out {
            if cur_mask == 3 { 1 } else { 0 }
        } else {
            let mut acc = 0usize;
            for next in &self.paths {
                let server = servers_map.get(next).unwrap();
                acc += server.walk_count_cached(servers_map, cur_mask, cache);
            }
            acc
        };

        cache.insert((self.name.clone(), cur_mask), res); // store keyed by (node,mask)
        res
    }
}

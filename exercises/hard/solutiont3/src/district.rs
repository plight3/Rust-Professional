use std::{collections::{HashMap, HashSet}, fs};


struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    fn new() -> Self {
        DSU {
            parent: Vec::new(),
        }
    }

    fn add(&mut self, n: usize){
        self.parent.push(n);
    }

    // 查找元素所属集合的代表元素（路径压缩优化）
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // 路径压缩
        }
        self.parent[x]
    }

    // 合并两个集合（按秩合并优化）
    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            // 按秩合并
            if root_x < root_y {
                self.parent[root_y] = root_x;
            } else {
                self.parent[root_x] = root_y;
            }
        }
    }

    // 检查两个元素是否属于同一集合
    fn is_connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn get_isolated_part(&mut self) -> usize {
        for i in 0..self.parent.len() {
            self.find(i);
        }
        let unique_elements: HashSet<_> = self.parent.iter().collect();
        return unique_elements.len();
    }
}


pub fn count_provinces() -> String {
    let json_data = fs::read_to_string("district.json").unwrap();
    let mut vec_input_str: Vec<&str> = json_data.lines().collect();
    let mut hashmap_input_data: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    let mut is_fist_marker = true;
    let mut cur_id = "0";
    let mut cur_data: Vec<String> = Vec::new();
    let mut cur_key = String::new();
    let mut cur_hashmap: HashMap<String, Vec<String>> = HashMap::new();
    for (i, line) in vec_input_str.iter().enumerate() {
        let line = line.trim();
        if line.contains("[") {
            if is_fist_marker {
                let temp: Vec<&str> = vec_input_str[i-1].trim().split("\"").collect();
                cur_id = temp[1];
                cur_hashmap = HashMap::new();
                is_fist_marker = false;
            }
            cur_key = line.split("\"").nth(1).unwrap().to_string();
            cur_data = line.split(",").map(|x|x.replace("\"", "")).collect();
            for item in cur_data.iter_mut() {
                if item.contains("[") {
                    *item = item.split("[").nth(1).unwrap().to_string();
                } 
                if item.contains("]") {
                    *item = item.split("]").nth(0).unwrap().to_string();
                }
                *item = item.trim().to_string();
            }
            if cur_data[cur_data.len()-1].is_empty() {
                cur_data.pop();
            }
            if cur_hashmap.contains_key(&cur_key) {
                cur_hashmap.get_mut(&cur_key).unwrap().append(&mut cur_data);
            } else {
                cur_hashmap.insert(cur_key, cur_data);
            }
        } else {
            if !is_fist_marker {
                is_fist_marker = true;
                hashmap_input_data.insert(cur_id.to_string(), cur_hashmap.clone());
            }
        }
    }
    let mut vec_ret = Vec::new();
    for (id, hashmap_case) in hashmap_input_data {
        let mut hashmap_k = HashMap::<String, usize>::new();
        let mut dsu = DSU::new();
        let mut cnt = 0;
        let mut k_node = 0;
        for (k, v) in hashmap_case {
            if !hashmap_k.contains_key(&k) {
                hashmap_k.insert(k, cnt);
                dsu.add(cnt);
                k_node = cnt;
                cnt += 1;
            } else {
                k_node = *hashmap_k.get(&k).unwrap();
            }
            for city in v {
                if !hashmap_k.contains_key(&city) {
                    hashmap_k.insert(city.to_string(), cnt);
                    dsu.add(cnt);
                    dsu.union(k_node, cnt);
                    cnt += 1;
                } else {
                    dsu.union(k_node, *hashmap_k.get(&city).unwrap());
                }
            }
        }
        vec_ret.push((id, dsu.get_isolated_part()));
    }
    vec_ret.sort_by(|x, y|x.0.cmp(&y.0));
    let mut string_ret = String::new();
    for item in vec_ret {
        string_ret.push_str(item.1.to_string().as_str());
        string_ret.push_str(",");
    }
    return string_ret.trim_end_matches(",").to_string();
}

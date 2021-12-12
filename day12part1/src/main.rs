use std::{
    collections::{HashMap, HashSet, LinkedList},
    fs,
};

#[derive(Debug)]
struct CaveSystem {
    graph: HashMap<String, LinkedList<String>>,
}

impl CaveSystem {
    fn create_connection(&mut self, cave_1: &str, cave_2: &str) {
        if !self.graph.contains_key(cave_1) {
            self.graph.insert(cave_1.to_string(), LinkedList::new());
        }

        if !self.graph.contains_key(cave_2) {
            self.graph.insert(cave_2.to_string(), LinkedList::new());
        }

        self.graph
            .get_mut(cave_1)
            .unwrap()
            .push_back(cave_2.to_string());

        self.graph
            .get_mut(cave_2)
            .unwrap()
            .push_back(cave_1.to_string());
    }

    pub fn new(input: Vec<&str>) -> Self {
        let mut cave_system = Self {
            graph: HashMap::new(),
        };

        for line in input {
            let mut connection = line.splitn(2, "-");
            let cave_1 = connection.next().unwrap();
            let cave_2 = connection.next().unwrap();

            cave_system.create_connection(cave_1, cave_2);
        }

        return cave_system;
    }

    fn get_paths(
        &self,
        node: String,
        paths: &mut HashSet<LinkedList<String>>,
        visited: &mut LinkedList<String>,
    ) {
        visited.push_back(node.clone());

        if node == "end" {
            paths.insert(visited.clone());
            return;
        }

        for adjcent_cave in self.graph.get(&node).unwrap() {
            if adjcent_cave == &adjcent_cave.to_uppercase() || !visited.contains(adjcent_cave) {
                self.get_paths(adjcent_cave.to_string(), paths, &mut visited.clone());
            }
        }
    }

    pub fn num_paths(&self) -> usize {
        let mut paths: HashSet<LinkedList<String>> = HashSet::new();

        self.get_paths("start".to_string(), &mut paths, &mut LinkedList::new());

        return paths.len();
    }
}

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let cave_system: CaveSystem = CaveSystem::new(data);

    println!("{}", cave_system.num_paths());
}

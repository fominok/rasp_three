extern crate rand;

use rand::Rng;

#[derive(Debug, PartialEq)]
struct Task {
    name: String,
    weight: u8,
}

#[derive(Debug, PartialEq)]
struct SlaveNode {
    name: &'static str,
    queue: Vec<Task>,
}

impl Task {
    pub fn new(name: String, weight: u8) -> Self {
        Task {
            name: name,
            weight: weight,
        }
    }
}

impl SlaveNode {
    pub fn new(name: &'static str) -> Self {
        SlaveNode {
            name: name,
            queue: vec![],
        }
    }

    pub fn place(&mut self, t: Task) {
        self.queue.push(t);
        self.queue.sort_unstable_by(|a, b| a.weight.cmp(&b.weight));
    }

    pub fn pop_fattest(&mut self) -> Option<Task> {
        self.queue.pop()
    }

    pub fn get_load(&self) -> u8 {
        self.queue.iter().fold(0, |sum, x| sum + x.weight)
    }
}

fn median_load(nodes: &[SlaveNode]) -> (usize, usize) {
    let mut min = nodes.first().expect("Nodes required");
    let mut max = nodes.last().expect("Nodes required");

    let mut loads = vec![];

    for n in nodes {
        let load = n.get_load();

        if load < min.get_load() {
            min = n;
        }

        if load > max.get_load() {
            max = n;
        }

        loads.push(load);
    }

    loads.sort();
    let lc = loads.len();
    let median = if lc % 2 == 0 {
        *loads.get(lc / 2).unwrap() as f64 + *loads.get((lc / 2) + 1).unwrap() as f64 / 2.0
    } else {
        *loads.get(lc / 2).unwrap() as f64
    };

    println!("Median load: {}", median);

    let min_pos = nodes.iter().position(|x| x == min).unwrap();
    let max_pos = nodes.iter().position(|x| x == max).unwrap();

    (min_pos, max_pos)
}

fn migrate(nodes: &mut Vec<SlaveNode>, from: usize, to: usize) {
    let (f, t) = if from < to {
        let (l,r) = nodes.split_at_mut(to);
        let f = l.get_mut(from).unwrap();
        let t = r.get_mut(0).unwrap();
        (f, t)
    } else {
        let (l, r) = nodes.split_at_mut(from);
        let f = r.get_mut(0).unwrap();
        let t = l.get_mut(to).unwrap();
        (f, t)
    };

    if let Some(task) = f.pop_fattest() {
        t.place(task);
    }
}

fn random_spawn(gen: &mut rand::ThreadRng, nodes: &mut [SlaveNode], count: u32) {
    for _ in 0..count {
        let w = gen.gen_range(1, 4);
        let i = gen.gen_range(0, nodes.len());
        let name = gen.gen_range(10, 50);

        if let Some(n) = nodes.get_mut(i) {
            n.place(Task::new(format!("Random {}", name), w));
            println!("Placing new task w = {} on node {}", w, i);
        }
    }

}

fn monitor(nodes: &[SlaveNode]) {
    println!("Load: {}", nodes.iter()
             .map(|x| x.get_load().to_string())
             .collect::<Vec<String>>()
             .join(" "));
}

fn deviation(nodes: &[SlaveNode]) -> f64 {
    let mean = (nodes.iter().fold(0, |acc, x| acc + x.get_load())) as f64 / nodes.len() as f64;
    let s: f64 = nodes.iter().map(|x| (x.get_load() as f64 - mean) * (x.get_load() as f64 - mean)).sum();
    f64::sqrt(s / nodes.len() as f64)
}

fn main() {
    let mut nodes = vec![];
    nodes.push(SlaveNode::new("Node 0"));
    nodes.push(SlaveNode::new("Node 1"));
    nodes.push(SlaveNode::new("Node 2"));
    nodes.push(SlaveNode::new("Node 3"));
    nodes.push(SlaveNode::new("Node 4")) ;

    let mut generator = rand::prelude::thread_rng();
    random_spawn(&mut generator, &mut nodes, 10);

    monitor(&nodes);
    println!("Std. dev. {}", deviation(&nodes));
    let (min_load, max_load) = median_load(&nodes);

    println!("Min loaded: {}, Max loaded: {}", min_load, max_load);
    println!("Migrating...");

    migrate(&mut nodes, max_load, min_load);
    monitor(&nodes);
    println!("Std. dev. {}", deviation(&nodes));
}

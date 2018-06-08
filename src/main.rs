//const TH_T: u32 =

#[derive(Debug)]
struct Task {
    name: &'static str,
    weight: u8,
}

#[derive(Debug)]
struct SlaveNode {
    name: &'static str,
    queue: Vec<Task>,
}

impl Task {
    pub fn new(name: &'static str, weight: u8) -> Self {
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

type Median<'a> = (&'a SlaveNode, &'a SlaveNode, f64);

fn median_load<'a>(nodes: &'a [SlaveNode]) -> Median<'a> {
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
    let mut median = if lc % 2 == 0 {
        *loads.get(lc / 2).unwrap() as f64 + *loads.get((lc / 2) + 1).unwrap() as f64 / 2.0
    } else {
        *loads.get((lc / 2) + 1).unwrap() as f64
    };

    (min, max, median)
}

fn main() {
    let mut node1 = SlaveNode::new("Node 1");
    let mut node2 = SlaveNode::new("Node 2");
    let mut node3 = SlaveNode::new("Node 3");
    let mut node4 = SlaveNode::new("Node 4");
    let mut node5 = SlaveNode::new("Node 5");

    node1.place(Task::new("Task 1", 5));
    node1.place(Task::new("Task 2", 3));
    node1.place(Task::new("Task 3", 4));

    let t = node1.pop_fattest();
    println!("{:?}", t);
}

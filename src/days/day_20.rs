use crate::day_tasks;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;

pub struct Day20;

impl day_tasks::DayTasks for Day20 {
    fn day_number (&self) -> String {
        "20".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let (map, start, finish) = parse(input);
        shortest_path(start, finish, &map).expect("- no result -").to_string()
    }
    fn task_1 (&self, input: &String) -> String {
        parse_1(input).shortest_path().expect("- no result -").to_string()
    }
}

fn shortest_path (
    start: (i32, i32), 
    finish: (i32, i32),
    map: &HashMap<(i32, i32), Node>) -> Option<i32> {
    
    let mut heap: BinaryHeap<PathState> = BinaryHeap::new();
    heap.push(PathState { position: start, distance: 0 });
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    while let Some(current_state) = heap.pop() {
        visited.insert(current_state.position);
        let current_node = if let Some(node) = map.get(&current_state.position) { node } else { panic!() };
        if current_state.position == finish {
            return Some(current_state.distance)
        }
        for (node_position, distance) in [current_node.up, current_node.down, current_node.left, current_node.right]
            .iter()
            .filter_map(|opt| *opt)
            .filter(|(pos, _)| !visited.contains(pos)){
            heap.push(PathState { position: node_position, distance: current_state.distance + distance });
        };
    }

    None
}

#[derive(PartialEq, Eq)]
struct PathState {
    position: (i32, i32),
    distance: i32
}

impl Ord for PathState {
    fn cmp(&self, other: &PathState) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &PathState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq)]
struct PathState1 {
    position: (i32, i32),
    distance: i32,
    floor: i32
}

impl Ord for PathState1 {
    fn cmp(&self, other: &PathState1) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for PathState1 {
    fn partial_cmp(&self, other: &PathState1) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Layer {
    Outer,
    Inner
}

#[derive(Clone, Copy)]
struct Node {
    up: Option<((i32, i32), i32)>,
    down: Option<((i32, i32), i32)>,
    left: Option<((i32, i32), i32)>,
    right: Option<((i32, i32), i32)>
}

struct Maze {
    map: HashMap<(i32, i32), Node>,
    start: (i32, i32),
    finish: (i32, i32),
    pos_to_label_and_layer: HashMap<(i32, i32), (String, Layer)>,
    outer_layer_to_port_position: HashMap<String, (i32, i32)>,
    inner_layer_to_port_position: HashMap<String, (i32, i32)>
}

impl Maze {
    fn shortest_path (&self) -> Option<i32> {
        
        let mut heap: BinaryHeap<PathState1> = BinaryHeap::new();
        heap.push(PathState1 { position: self.start, distance: 0, floor: 0 });
        let mut visited: HashSet<((i32, i32), i32)> = HashSet::new();
    
        while let Some(current_state) = heap.pop() {
            visited.insert((current_state.position, current_state.floor));
            let current_node = if let Some(node) = self.map.get(&current_state.position) { node } else { panic!() };
            if current_state.position == self.finish && current_state.floor == 0 {
                return Some(current_state.distance)
            }
            for (node_position, distance, floor) in [self.get_port(&current_node.up, (current_state.position.0, current_state.position.1 - 1), current_state.floor), 
                self.get_port(&current_node.down, (current_state.position.0, current_state.position.1 + 1), current_state.floor), 
                self.get_port(&current_node.left, (current_state.position.0 - 1, current_state.position.1), current_state.floor), 
                self.get_port(&current_node.right, (current_state.position.0 + 1, current_state.position.1), current_state.floor)]
                .iter()
                .filter_map(|opt| *opt)
                .filter(|(pos, _, floor)| !visited.contains(&(*pos, *floor))){
                heap.push(PathState1 { position: node_position, distance: current_state.distance + distance, floor: floor });
            };
        }
    
        None
    }
    fn get_port (&self, next_node: &Option<((i32, i32), i32)>, possible_label_position: (i32, i32), current_floor: i32) -> Option<((i32, i32), i32, i32)> {
        next_node
            .map(|(pos, dist)| (pos, dist, current_floor))
            .or_else(|| {
                if let Some((label, layer)) = self.pos_to_label_and_layer.get(&possible_label_position) {
                    if *layer == Layer::Inner {
                        Some((*self.outer_layer_to_port_position.get(label).expect("- unexpected -"), 1, current_floor + 1))
                    }
                    else if *layer == Layer::Outer && current_floor > 0 {
                        Some((*self.inner_layer_to_port_position.get(label).expect("- unexpected -"), 1, current_floor - 1))
                    }
                    else { None }
                }
                else { None }
            })
    }
}

fn parse_1 (input: &String) -> Maze {
    fn removing_criteria (node: &Node, endpoint_count: usize) -> bool {
        [node.up, node.down, node.left, node.right]
            .iter()
            .filter_map(|x| *x)
            .count() == endpoint_count 
    }

    let lines = input.lines().collect::<Vec<_>>();

    let outer_up = 1;
    let outer_down = lines.len() as i32 - 2;
    let outer_left = 1;
    let outer_right = lines.iter().map(|line| line.len()).max().expect("no lines in input") as i32 - 2;

    let mut starting_position = (0, 0);
    let mut finish_position = (0, 0);

    let mut pos_to_label_and_layer: HashMap<(i32, i32), (String, Layer)> = HashMap::new();
    let mut outer_layer_to_port_position: HashMap<String, (i32, i32)> = HashMap::new();
    let mut inner_layer_to_port_position: HashMap<String, (i32, i32)> = HashMap::new();
    let mut port_positions: HashSet<(i32, i32)> = HashSet::new();

    let mut map = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| 
            line
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '.')
                .map(move |(x, _)| (x, y))
        )
        .map(|(x, y)| {
            let node = Node {
                up: None,
                down: None,
                left: None,
                right: None };
            ((x as i32, y as i32), node)
        })
        .collect::<HashMap<(i32, i32), Node>>();

    for (x, y) in lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| 
            line
                .chars()
                .enumerate()
                .filter(|(_, c)| match *c { 'A'..='Z' => true, _ => false })
                .map(move |(x, _)| (x, y))) {
        let (x, y) = (x as i32, y as i32);
        if let Some((direction, port_position)) = 
            if let Some(_) = map.get(&(x, y + 1)) { Some((Direction::Up, (x, y + 1))) }
            else if let Some(_) = map.get(&(x, y - 1)) { Some((Direction::Down, (x, y - 1))) }
            else if let Some(_) = map.get(&(x + 1, y)) { Some((Direction::Left, (x + 1, y))) }
            else if let Some(_) = map.get(&(x - 1, y)) { Some((Direction::Right, (x - 1, y))) }
            else { None } {
            let layer = 
                if x == outer_left || x == outer_right || y == outer_up || y == outer_down { Layer::Outer } 
                else { Layer::Inner };
            let label = match direction {
                Direction::Up => format!("{}{}", lines.get(y as usize - 1).unwrap().chars().nth(x as usize).unwrap(), lines.get(y as usize).unwrap().chars().nth(x as usize).unwrap()),
                Direction::Down => format!("{}{}", lines.get(y as usize).unwrap().chars().nth(x as usize).unwrap(), lines.get(y as usize + 1).unwrap().chars().nth(x as usize).unwrap()),
                Direction::Left => format!("{}{}", lines.get(y as usize).unwrap().chars().nth(x as usize - 1).unwrap(), lines.get(y as usize).unwrap().chars().nth(x as usize).unwrap()),
                Direction::Right => format!("{}{}", lines.get(y as usize).unwrap().chars().nth(x as usize).unwrap(), lines.get(y as usize).unwrap().chars().nth(x as usize + 1).unwrap()),
            };
            port_positions.insert(port_position);
            if label == "AA" {
                starting_position = port_position;
            }
            else if label == "ZZ" {
                finish_position = port_position;
            }
            else {
                pos_to_label_and_layer.insert((x, y), (label.to_string(), layer));
            
                if layer == Layer::Outer {
                    outer_layer_to_port_position.insert(label.to_string(), port_position);
                }
                else {
                    inner_layer_to_port_position.insert(label.to_string(), port_position);
                }
            }
        }
    }

    map = map 
        .iter()
        .map(|((x, y), _)| {
            ((*x, *y), Node {
                up: if map.contains_key(&(*x, *y - 1)) { Some(((*x, *y - 1), 1))} else { None },
                down: if map.contains_key(&(*x, *y + 1)) { Some(((*x, *y + 1), 1))} else { None },
                left: if map.contains_key(&(*x - 1, *y)) { Some(((*x - 1, *y), 1))} else { None },
                right: if map.contains_key(&(*x + 1, *y)) { Some(((*x + 1, *y), 1))} else { None }
            })
        })
        .collect::<HashMap<(i32, i32), Node>>();
    
    // remove with only two endpoints
    let binary_nodes = 
        map
            .iter()
            .filter(|(pos, node)| !port_positions.contains(*pos) && removing_criteria(node, 2usize))
            .map(|(pos, _)| *pos)
            .collect::<Vec<(i32, i32)>>();
    
    for node_position in binary_nodes {
        let node = map.get(&node_position);
        if let Some(node) = node {
            let option = match (node.up, node.down, node.left, node.right) {
                (Some(t_0), Some(t_1), None, None)
                | (Some(t_0), None, Some(t_1), None)
                | (Some(t_0), None, None, Some(t_1))
                | (None, Some(t_0), Some(t_1), None)
                | (None, Some(t_0), None, Some(t_1))
                | (None, None, Some(t_0), Some(t_1)) => Some((t_0, t_1)),
                _ => None
            };
            if let Some(((node_0_position, dist_0), (node_1_position, dist_1))) = option {
                fn replace_proxy (
                    node: &Node,
                    proxy_position: (i32, i32),
                    other_position: (i32, i32),
                    new_distance: i32) -> Node {
                    fn replace_if_equal_to_proxy (
                        before: Option<((i32, i32), i32)>, 
                        proxy_position: (i32, i32),
                        new_position: (i32, i32),
                        new_distance: i32) -> Option<((i32, i32), i32)> {
                        if let Some((before_position, _)) = before {
                            if before_position == proxy_position { Some((new_position, new_distance))}
                            else { before }
                        } 
                        else { None }
                    }
                    Node {
                        up: replace_if_equal_to_proxy(node.up, proxy_position, other_position, new_distance),
                        down: replace_if_equal_to_proxy(node.down, proxy_position, other_position, new_distance),
                        left: replace_if_equal_to_proxy(node.left, proxy_position, other_position, new_distance),
                        right: replace_if_equal_to_proxy(node.right, proxy_position, other_position, new_distance),
                        ..*node
                    }
                }
                let dist = dist_0 + dist_1;
                if let Some(node_0) = map.get(&node_0_position) {
                    let new_node_0 = replace_proxy(node_0, node_position, node_1_position, dist);
                    map.insert(node_0_position, new_node_0);
                }
                if let Some(node_1) = map.get(&node_1_position) {
                    let new_node_1 = replace_proxy(node_1, node_position, node_0_position, dist);
                    map.insert(node_1_position, new_node_1);
                }
            }
        }
        map.remove(&node_position);
    }

    // remove dead ends
    let dead_end_nodes = 
    map
        .iter()
        .filter(|(pos, node)| !port_positions.contains(*pos) && removing_criteria(node, 1usize))
        .map(|(pos, _)| *pos)
        .collect::<Vec<(i32, i32)>>();

    for node_position in dead_end_nodes {
        let node = map.get(&node_position);
        if let Some(node) = node {
            let option = match (node.up, node.down, node.left, node.right) {
                (Some(t), None, None, None)
                | (None, Some(t), None, None)
                | (None, None, Some(t), None)
                | (None, None, None, Some(t)) => Some(t),
                _ => None
            };
            if let Some((node_0_position, _)) = option {
                fn replace_proxy (
                    node: &Node,
                    proxy_position: (i32, i32)) -> Node {
                    fn replace_if_equal_to_proxy (
                        before: Option<((i32, i32), i32)>, 
                        proxy_position: (i32, i32)) -> Option<((i32, i32), i32)> {
                        if let Some((before_position, _)) = before {
                            if before_position == proxy_position { None }
                            else { before }
                        } 
                        else { None }
                    }
                    Node {
                        up: replace_if_equal_to_proxy(node.up, proxy_position),
                        down: replace_if_equal_to_proxy(node.down, proxy_position),
                        left: replace_if_equal_to_proxy(node.left, proxy_position),
                        right: replace_if_equal_to_proxy(node.right, proxy_position),
                        ..*node
                    }
                }
                if let Some(node_0) = map.get(&node_0_position) {
                    let new_node_0 = replace_proxy(node_0, node_position);
                    map.insert(node_0_position, new_node_0);
                }
            }
        }
        map.remove(&node_position);
    }

    Maze { 
        map: map, 
        start: starting_position, 
        finish: finish_position, 
        pos_to_label_and_layer: pos_to_label_and_layer, 
        inner_layer_to_port_position: inner_layer_to_port_position,
        outer_layer_to_port_position: outer_layer_to_port_position
    }
}

fn parse (input: &String) -> (HashMap<(i32, i32), Node>, (i32, i32), (i32, i32)) {
    fn removing_criteria (node: &Node, endpoint_count: usize) -> bool {
        [node.up, node.down, node.left, node.right]
            .iter()
            .filter_map(|x| *x)
            .count() == endpoint_count 
    }

    let lines = input.lines().collect::<Vec<_>>();

    let outer_up = 1;
    let outer_down = lines.len() as i32 - 2;
    let outer_left = 1;
    let outer_right = lines.iter().map(|line| line.len()).max().expect("no lines in input") as i32 - 2;

    let mut starting_position = (0, 0);
    let mut finish_position = (0, 0);

    let mut pos_to_label_and_layer: HashMap<(i32, i32), (String, Layer)> = HashMap::new();
    let mut outer_layer_to_port_position: HashMap<String, (i32, i32)> = HashMap::new();
    let mut inner_layer_to_port_position: HashMap<String, (i32, i32)> = HashMap::new();

    let mut map = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| 
            line
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '.')
                .map(move |(x, _)| (x, y))
        )
        .map(|(x, y)| {
            let node = Node {
                up: None,
                down: None,
                left: None,
                right: None };
            ((x as i32, y as i32), node)
        })
        .collect::<HashMap<(i32, i32), Node>>();

    for (x, y) in lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| 
            line
                .chars()
                .enumerate()
                .filter(|(_, c)| match *c { 'A'..='Z' => true, _ => false })
                .map(move |(x, _)| (x, y))) {
        let (x, y) = (x as i32, y as i32);
        if let Some((direction, port_position)) = 
            if let Some(_) = map.get(&(x, y + 1)) { Some((Direction::Up, (x, y + 1))) }
            else if let Some(_) = map.get(&(x, y - 1)) { Some((Direction::Down, (x, y - 1))) }
            else if let Some(_) = map.get(&(x + 1, y)) { Some((Direction::Left, (x + 1, y))) }
            else if let Some(_) = map.get(&(x - 1, y)) { Some((Direction::Right, (x - 1, y))) }
            else { None } {
            let layer = 
                if x == outer_left || x == outer_right || y == outer_up || y == outer_down { Layer::Outer } 
                else { Layer::Inner };
            let label = match direction {
                Direction::Up => format!("{}{}", lines.get(y as usize - 1).unwrap().chars().nth(x as usize).unwrap(), lines.get(y as usize).unwrap().chars().nth(x as usize).unwrap()),
                Direction::Down => format!("{}{}", lines.get(y as usize).unwrap().chars().nth(x as usize).unwrap(), lines.get(y as usize + 1).unwrap().chars().nth(x as usize).unwrap()),
                Direction::Left => format!("{}{}", lines.get(y as usize).unwrap().chars().nth(x as usize - 1).unwrap(), lines.get(y as usize).unwrap().chars().nth(x as usize).unwrap()),
                Direction::Right => format!("{}{}", lines.get(y as usize).unwrap().chars().nth(x as usize).unwrap(), lines.get(y as usize).unwrap().chars().nth(x as usize + 1).unwrap()),
            };
            if label == "AA" {
                starting_position = port_position;
            }
            else if label == "ZZ" {
                finish_position = port_position;
            }
            else {
                pos_to_label_and_layer.insert((x, y), (label.to_string(), layer));
            
                if layer == Layer::Outer {
                    outer_layer_to_port_position.insert(label.to_string(), port_position);
                }
                else {
                    inner_layer_to_port_position.insert(label.to_string(), port_position);
                }
            }
        }
    }

    map = map 
        .iter()
        .map(|((x, y), _)| {
            ((*x, *y), Node {
                up: if map.contains_key(&(*x, *y - 1)) { Some(((*x, *y - 1), 1))} 
                else if let Some((label, layer)) = pos_to_label_and_layer.get(&(*x, *y - 1)) {
                    if *layer == Layer::Outer {
                        inner_layer_to_port_position.get(label).map(|pos| (*pos, 1))
                    }
                    else { outer_layer_to_port_position.get(label).map(|pos| (*pos, 1)) }
                }
                else { None },
                down: if map.contains_key(&(*x, *y + 1)) { Some(((*x, *y + 1), 1))}
                else if let Some((label, layer)) = pos_to_label_and_layer.get(&(*x, *y + 1)) {
                    if *layer == Layer::Outer {
                        inner_layer_to_port_position.get(label).map(|pos| (*pos, 1))
                    }
                    else { outer_layer_to_port_position.get(label).map(|pos| (*pos, 1)) }
                } else { None },
                left: if map.contains_key(&(*x - 1, *y)) { Some(((*x - 1, *y), 1))}
                else if let Some((label, layer)) = pos_to_label_and_layer.get(&(*x - 1, *y)) {
                    if *layer == Layer::Outer {
                        inner_layer_to_port_position.get(label).map(|pos| (*pos, 1))
                    }
                    else { outer_layer_to_port_position.get(label).map(|pos| (*pos, 1)) }
                } else { None },
                right: if map.contains_key(&(*x + 1, *y)) { Some(((*x + 1, *y), 1))}
                else if let Some((label, layer)) = pos_to_label_and_layer.get(&(*x + 1, *y)) {
                    if *layer == Layer::Outer {
                        inner_layer_to_port_position.get(label).map(|pos| (*pos, 1))
                    }
                    else { outer_layer_to_port_position.get(label).map(|pos| (*pos, 1)) }
                } else { None }
            })
        })
        .collect::<HashMap<(i32, i32), Node>>();
    
    // remove with only two endpoints
    let binary_nodes = 
        map
            .iter()
            .filter(|(pos, node)| **pos != starting_position && **pos != finish_position && removing_criteria(node, 2usize))
            .map(|(pos, _)| *pos)
            .collect::<Vec<(i32, i32)>>();
    
    for node_position in binary_nodes {
        let node = map.get(&node_position);
        if let Some(node) = node {
            let option = match (node.up, node.down, node.left, node.right) {
                (Some(t_0), Some(t_1), None, None)
                | (Some(t_0), None, Some(t_1), None)
                | (Some(t_0), None, None, Some(t_1))
                | (None, Some(t_0), Some(t_1), None)
                | (None, Some(t_0), None, Some(t_1))
                | (None, None, Some(t_0), Some(t_1)) => Some((t_0, t_1)),
                _ => None
            };
            if let Some(((node_0_position, dist_0), (node_1_position, dist_1))) = option {
                fn replace_proxy (
                    node: &Node,
                    proxy_position: (i32, i32),
                    other_position: (i32, i32),
                    new_distance: i32) -> Node {
                    fn replace_if_equal_to_proxy (
                        before: Option<((i32, i32), i32)>, 
                        proxy_position: (i32, i32),
                        new_position: (i32, i32),
                        new_distance: i32) -> Option<((i32, i32), i32)> {
                        if let Some((before_position, _)) = before {
                            if before_position == proxy_position { Some((new_position, new_distance))}
                            else { before }
                        } 
                        else { None }
                    }
                    Node {
                        up: replace_if_equal_to_proxy(node.up, proxy_position, other_position, new_distance),
                        down: replace_if_equal_to_proxy(node.down, proxy_position, other_position, new_distance),
                        left: replace_if_equal_to_proxy(node.left, proxy_position, other_position, new_distance),
                        right: replace_if_equal_to_proxy(node.right, proxy_position, other_position, new_distance),
                        ..*node
                    }
                }
                let dist = dist_0 + dist_1;
                if let Some(node_0) = map.get(&node_0_position) {
                    let new_node_0 = replace_proxy(node_0, node_position, node_1_position, dist);
                    map.insert(node_0_position, new_node_0);
                }
                if let Some(node_1) = map.get(&node_1_position) {
                    let new_node_1 = replace_proxy(node_1, node_position, node_0_position, dist);
                    map.insert(node_1_position, new_node_1);
                }
            }
        }
        map.remove(&node_position);
    }

    // remove dead ends
    let dead_end_nodes = 
    map
        .iter()
        .filter(|(pos, node)|  **pos != starting_position && **pos != finish_position && removing_criteria(node, 1usize))
        .map(|(pos, _)| *pos)
        .collect::<Vec<(i32, i32)>>();

    for node_position in dead_end_nodes {
        let node = map.get(&node_position);
        if let Some(node) = node {
            let option = match (node.up, node.down, node.left, node.right) {
                (Some(t), None, None, None)
                | (None, Some(t), None, None)
                | (None, None, Some(t), None)
                | (None, None, None, Some(t)) => Some(t),
                _ => None
            };
            if let Some((node_0_position, _)) = option {
                fn replace_proxy (
                    node: &Node,
                    proxy_position: (i32, i32)) -> Node {
                    fn replace_if_equal_to_proxy (
                        before: Option<((i32, i32), i32)>, 
                        proxy_position: (i32, i32)) -> Option<((i32, i32), i32)> {
                        if let Some((before_position, _)) = before {
                            if before_position == proxy_position { None }
                            else { before }
                        } 
                        else { None }
                    }
                    Node {
                        up: replace_if_equal_to_proxy(node.up, proxy_position),
                        down: replace_if_equal_to_proxy(node.down, proxy_position),
                        left: replace_if_equal_to_proxy(node.left, proxy_position),
                        right: replace_if_equal_to_proxy(node.right, proxy_position),
                        ..*node
                    }
                }
                if let Some(node_0) = map.get(&node_0_position) {
                    let new_node_0 = replace_proxy(node_0, node_position);
                    map.insert(node_0_position, new_node_0);
                }
            }
        }
        map.remove(&node_position);
    }

    (map, starting_position, finish_position)
}

use crate::day_tasks;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;
use std::i32;

pub struct Day18;

struct Node {
    up: Option<((i32, i32), i32)>,
    down: Option<((i32, i32), i32)>,
    left: Option<((i32, i32), i32)>,
    right: Option<((i32, i32), i32)>,
    node_type: NodeType
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

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
enum NodeType {
    Normal,
    StartingPoint,
    Key(char),
    Door(char)
}

impl day_tasks::DayTasks for Day18 {
    fn day_number (&self) -> String {
        "18".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let map = parse(input);
        determine_shortest_round_trip_single(&map).to_string()
    }
    fn task_1 (&self, input: &String) -> String {
        let map = parse(input);
        determine_shortest_round_trip_quadriple(&map).to_string()
    }
}

fn determine_shortest_round_trip_quadriple (whole_map: &HashMap<(i32, i32), Node>) -> i32 {
    fn is_key (node_type: &NodeType) -> Option<char> {
        match node_type { NodeType::Key(c) => Some(*c), _ => None }
    }

    fn shortest_path (
        node: (i32, i32), 
        target: (i32, i32), 
        whole_map: &HashMap<(i32, i32), Node>) -> Option<i32> {
        
        let mut heap: BinaryHeap<PathState> = BinaryHeap::new();
        heap.push(PathState { position: node, distance: 0 });
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
    
        while let Some(current_state) = heap.pop() {
            visited.insert(current_state.position);
            let current_node = if let Some(node) = whole_map.get(&current_state.position) { node } else { panic!() };
            if current_state.position == target {
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

    let sequence = ['w', 'j', 'q', 'd', 's', 'r', 'o', 'f', 'b', 'a', 'm'];

    let keys = whole_map
        .iter()
        .filter_map(|(position, node)| is_key(&node.node_type).map(|c| (*position, c)))
        .map(|(position, c)| (c, position))
        .collect::<HashMap<char, (i32, i32)>>();

    let starting_point = whole_map
        .iter()
        .filter(|(_, node)| node.node_type == NodeType::StartingPoint)
        .map(|(position, _)| *position)
        .nth(0)
        .expect("starting point not found");

    let mut current_position = starting_point;

    let mut distance = - 2;

    for next_c in &sequence {
        if let Some(target_position) = keys.get(&next_c) {
            distance = distance + shortest_path(current_position, *target_position, whole_map).expect("couldn't calculate shortest path");
            current_position = *target_position;
        }
        else { panic!("key not found")}
    } 

    let sequence = ['v', 'y', 'c', 'k'];

    current_position = starting_point;

    distance = distance - 2;

    for next_c in &sequence {
        if let Some(target_position) = keys.get(&next_c) {
            distance = distance + shortest_path(current_position, *target_position, whole_map).expect("couldn't calculate shortest path");
            current_position = *target_position;
        }
        else { panic!("key not found")}
    } 

    let sequence = ['p', 'z', 'g', 'n',  'x'];

    current_position = starting_point;

    distance = distance - 2;

    for next_c in &sequence {
        if let Some(target_position) = keys.get(&next_c) {
            distance = distance + shortest_path(current_position, *target_position, whole_map).expect("couldn't calculate shortest path");
            current_position = *target_position;
        }
        else { panic!("key not found")}
    } 

    let sequence = ['i', 't', 'u', 'l',  'e', 'h'];

    current_position = starting_point;

    distance = distance - 2;

    for next_c in &sequence {
        if let Some(target_position) = keys.get(&next_c) {
            distance = distance + shortest_path(current_position, *target_position, whole_map).expect("couldn't calculate shortest path");
            current_position = *target_position;
        }
        else { panic!("key not found")}
    } 

    distance
}

fn determine_shortest_round_trip_single (whole_map: &HashMap<(i32, i32), Node>) -> i32 {
    fn is_key (node_type: &NodeType) -> Option<char> {
        match node_type { NodeType::Key(c) => Some(*c), _ => None }
    }

    fn shortest_path (
        node: (i32, i32), 
        target: (i32, i32), 
        whole_map: &HashMap<(i32, i32), Node>) -> Option<i32> {
        
        let mut heap: BinaryHeap<PathState> = BinaryHeap::new();
        heap.push(PathState { position: node, distance: 0 });
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
    
        while let Some(current_state) = heap.pop() {
            visited.insert(current_state.position);
            let current_node = if let Some(node) = whole_map.get(&current_state.position) { node } else { panic!() };
            if current_state.position == target {
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

    let sequence = ['w', 'i', 'v', 'y',  'j', 'q', 'd', 's', 'r', 'o', 't', 'u', 'l', 'e', 'p', 'z', 'g', 'n', 'x', 'f', 'b', 'h', 'c', 'k', 'a', 'm'];

    let keys = whole_map
        .iter()
        .filter_map(|(position, node)| is_key(&node.node_type).map(|c| (*position, c)))
        .map(|(position, c)| (c, position))
        .collect::<HashMap<char, (i32, i32)>>();

    let mut current_position: (i32, i32) = whole_map
        .iter()
        .filter(|(_, node)| node.node_type == NodeType::StartingPoint)
        .map(|(position, _)| *position)
        .nth(0)
        .expect("starting point not found");

    let mut distance = 0;

    for next_c in &sequence {
        if let Some(target_position) = keys.get(&next_c) {
            distance = distance + shortest_path(current_position, *target_position, whole_map).expect("couldn't calculate shortest path");
            current_position = *target_position;
        }
        else { panic!("key not found")}
    } 

    distance
}

fn parse (input: &String) -> HashMap<(i32, i32), Node> {
    fn removing_criteria (node: &Node, endpoint_count: usize) -> bool {
        if node.node_type != NodeType::Normal { false }
        else {
            (&[node.up, node.down, node.left, node.right])
                .iter()
                .filter_map(|x| *x)
                .count() == endpoint_count }
    }
    let mut map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| 
            line
                .chars()
                .enumerate()
                .filter(|(_, c)| *c != '#')
                .map(move |(x, c)| (x, y, c))
        )
        .map(|(x, y, c)| {
            let node = Node {
                up: None,
                down: None,
                left: None,
                right: None,
                node_type: match c {
                    'a'..='z' => NodeType::Key(c),
                    'A'..='Z' => NodeType::Door(c.to_lowercase().to_string().chars().nth(0).unwrap()),
                    '@' => NodeType::StartingPoint,
                    _ => NodeType::Normal
                }
            };
            ((x as i32, y as i32), node)
        })
        .collect::<HashMap<(i32, i32), Node>>();

    map = map
        .iter()
        .map(|((x, y), node)| {
            ((*x, *y), Node {
                up: if map.contains_key(&(*x, *y - 1)) { Some(((*x, *y - 1), 1))} else { None },
                down: if map.contains_key(&(*x, *y + 1)) { Some(((*x, *y + 1), 1))} else { None },
                left: if map.contains_key(&(*x - 1, *y)) { Some(((*x - 1, *y), 1))} else { None },
                right: if map.contains_key(&(*x + 1, *y)) { Some(((*x + 1, *y), 1))} else { None },
                ..*node
            })
        })
        .collect::<HashMap<(i32, i32), Node>>();
    
    // remove with only two endpoints
    let binary_nodes = 
        map
            .iter()
            .filter(|(_, node)| removing_criteria(node, 2usize))
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
        .filter(|(_, node)| removing_criteria(node, 1usize))
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

    map
}

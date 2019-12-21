use crate::day_tasks;
use std::collections::{HashMap, HashSet};
use std::ops::Sub;

pub struct Day18;

struct Node {
    up: Option<((i32, i32), i32)>,
    down: Option<((i32, i32), i32)>,
    left: Option<((i32, i32), i32)>,
    right: Option<((i32, i32), i32)>,
    node_type: NodeType
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
        determine_shortest_round_trip(&map).to_string()
    }
    fn task_1 (&self, input: &String) -> String {
        "".to_string()
    }
}

fn determine_shortest_round_trip (whole_map: &HashMap<(i32, i32), Node>) -> i32 {
    fn is_non_normal (node_type: &NodeType) -> bool {
        match node_type { NodeType::Normal | NodeType::Door(_) => false, _ => true }
    }

    fn distances_between_non_normals (
        non_normals: &HashMap<NodeType, (i32, i32)>, 
        whole_map: &HashMap<(i32, i32), Node>) -> HashMap<(NodeType, NodeType), i32> {

        fn shortest_paths (
            node: (i32, i32), 
            distance_so_far: i32,
            distances_to_non_normals_map: &mut HashMap<NodeType, i32>, 
            distances_map: &mut HashMap<(i32, i32), i32>, 
            whole_map: &HashMap<(i32, i32), Node>) {
            fn check_path (
                neighbor: Option<((i32, i32), i32)>,
                distance_so_far: i32,
                distances_to_non_normals_map: &mut HashMap<NodeType, i32>, 
                distances_map: &mut HashMap<(i32, i32), i32>, 
                whole_map: &HashMap<(i32, i32), Node>) {
                if let Some((neighbor, distance)) = neighbor {
                    let distance_so_far = distance_so_far + distance;
                    match distances_map.get(&neighbor) {
                        Some(dist) if *dist > distance_so_far => shortest_paths(neighbor, distance_so_far, distances_to_non_normals_map, distances_map, whole_map),
                        None => shortest_paths(neighbor, distance_so_far, distances_to_non_normals_map, distances_map, whole_map),
                        _ => ()
                    }
                }
            }
            if distance_so_far > 0 { 
                distances_map.insert(node, distance_so_far);
                if let Some(node) = whole_map.get(&node) {
                    if is_non_normal(&node.node_type) {
                        distances_to_non_normals_map.insert(node.node_type.clone(), distance_so_far);
                    }
                }
            }
            if let Some(node) = whole_map.get(&node) {
                check_path(node.up, distance_so_far, distances_to_non_normals_map, distances_map, whole_map);
                check_path(node.down, distance_so_far, distances_to_non_normals_map, distances_map, whole_map);
                check_path(node.left, distance_so_far, distances_to_non_normals_map, distances_map, whole_map);
                check_path(node.right, distance_so_far, distances_to_non_normals_map, distances_map, whole_map);
            }
        }

        let mut shortest_paths_between_non_normals_map: HashMap<(NodeType, NodeType), i32> = HashMap::new();
        for (non_normal, node_position) in non_normals {
            let mut shortest_paths_to_non_normals_map: HashMap<NodeType, i32> = HashMap::new();
            shortest_paths(*node_position, 0, &mut shortest_paths_to_non_normals_map, &mut HashMap::new(), whole_map);
            for (other_non_normal, distance) in shortest_paths_to_non_normals_map {
                shortest_paths_between_non_normals_map.insert((non_normal.clone(), other_non_normal), distance);
                shortest_paths_between_non_normals_map.insert((other_non_normal, non_normal.clone()), distance);
            }
        }
        shortest_paths_between_non_normals_map
    }

    fn get_shortest_path (
        current_non_normal: &NodeType,
        distance_so_far: i32,
        current_owned_keys: &HashSet<NodeType>,
        non_normal_positions: &HashMap<NodeType, (i32, i32)>,
        distances_between_non_normals: &HashMap<(NodeType, NodeType), i32>,
        whole_map: &HashMap<(i32, i32), Node>) -> i32 {
        fn get_reachable_keys (
            current_position: (i32, i32), 
            mut visited: &mut HashSet<(i32, i32)>, 
            current_owned_keys: &HashSet<NodeType>,
            whole_map: &HashMap<(i32, i32), Node>, 
            mut reachable_keys: &mut HashSet<NodeType>) {
            visited.insert(current_position);
            if let Some(node) = whole_map.get(&current_position) {
                match node.node_type {
                    NodeType::Key(c) => { reachable_keys.insert(NodeType::Key(c)); },
                    NodeType::Door(c) if !current_owned_keys.contains(&NodeType::Door(c)) => { return; }
                    _  => ()
                }

                for next_position in &[node.up, node.down, node.left, node.right]
                    .iter()
                    .filter_map(|x| *x)
                    .filter(|(position, _)| !visited.contains(position))
                    .map(|(position, _)| position)
                    .collect::<Vec<(i32, i32)>>() {
                    get_reachable_keys(
                        *next_position,
                        &mut visited,
                        current_owned_keys,
                        whole_map,
                        &mut reachable_keys);
                }
            }
        }
        let mut reachable_keys: HashSet<NodeType> = HashSet::new();
        get_reachable_keys(
            *non_normal_positions.get(current_non_normal).unwrap(),
            &mut HashSet::new(),
            current_owned_keys,
            whole_map,
            &mut reachable_keys);
        let next_keys = reachable_keys.sub(current_owned_keys);
        println!("reachable {:?} next {:?} owned {:?}", reachable_keys, next_keys, current_owned_keys);
        if next_keys.len() > 0 {
            next_keys
            .iter()
            .map(|key| {
                let distance = distances_between_non_normals.get(&(current_non_normal.clone(), key.clone())).unwrap();
                let as_hash_set: HashSet<NodeType> = [key.clone()].iter().map(|key| key.clone()).collect();
                let new_current_owned_keys = current_owned_keys.union(&as_hash_set);
                get_shortest_path(key, distance_so_far + distance, &new_current_owned_keys.map(|key| key.clone()).collect(), non_normal_positions, distances_between_non_normals, whole_map)
            })
            .min()
            .unwrap_or(-1)
        }
        else { distance_so_far }
    }

    let non_normals = whole_map
        .iter()
        .filter(|(_, node)| is_non_normal(&node.node_type))
        .map(|(position, node)| (node.node_type.clone(), *position))
        .collect::<HashMap<NodeType, (i32, i32)>>();

    let distances_between_non_normals_map = distances_between_non_normals(&non_normals, whole_map);
    get_shortest_path(
        &NodeType::StartingPoint, 
        0,
        &HashSet::new(), 
        &non_normals, 
        &distances_between_non_normals_map,
        whole_map)
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
                    'A'..='Z' => NodeType::Door(c),
                    '@' => NodeType::StartingPoint,
                    _ => NodeType::Normal
                }
            };
            ((x as i32, y as i32), node)
        })
        .collect::<HashMap<(i32, i32), Node>>();

    println!("{:?}", map.len());

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
    println!("{:?}", map.len());

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
    println!("{:?}", map.len());

    map
}

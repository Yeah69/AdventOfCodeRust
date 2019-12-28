use crate::day_tasks;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;
use std::i32;
use colored::Colorize;

pub struct Day18;

struct Node {
    up: Option<((i32, i32), i32)>,
    down: Option<((i32, i32), i32)>,
    left: Option<((i32, i32), i32)>,
    right: Option<((i32, i32), i32)>,
    node_type: NodeType
}

#[derive(Debug)]
struct NodeOfInterest {
    node_type: NodeType,
    distances: HashMap<NodeType, Vec<(HashSet<char>, i32)>>
}

#[derive(PartialEq, Eq)]
struct RouteState {
    node_type: NodeType,
    distance: i32,
    owned_keys: HashSet<char>
}

impl Ord for RouteState {
    fn cmp(&self, other: &RouteState) -> Ordering {
        //(self.owned_keys.len() as f32 * self.owned_keys.len() as f32  / self.distance as f32).partial_cmp(&(other.owned_keys.len() as f32 * other.owned_keys.len() as f32  / other.distance as f32)).unwrap()
        self.owned_keys.len().cmp(&other.owned_keys.len())
            .then_with(|| other.distance.cmp(&self.distance))
    }
}

impl PartialOrd for RouteState {
    fn partial_cmp(&self, other: &RouteState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
struct PathStateDirectAcyclicGraph {
    position: (i32, i32),
    distance: i32,
    last_required_door: Option<char>
}

impl Ord for PathStateDirectAcyclicGraph {
    fn cmp(&self, other: &PathStateDirectAcyclicGraph) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for PathStateDirectAcyclicGraph {
    fn partial_cmp(&self, other: &PathStateDirectAcyclicGraph) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl NodeOfInterest {
    fn get_min_distance (&self) -> i32 {
        self.distances.iter().flat_map(|(_, vector)| vector.iter().map(|(_, dist)| *dist)).min().unwrap()
    }

    fn get_reachable_nodes_of_interest (&self, available_keys: &HashSet<char>) -> HashMap<NodeType, i32> {
        self.distances.iter().filter_map(|(node_type, vector)| {
            vector.iter().filter_map(|(key_set, dist)| if key_set.is_subset(available_keys) { Some((node_type.clone(), *dist)) } else { None }).min_by_key(|(_, dist)| *dist)
        }).collect::<HashMap<NodeType, i32>>()
    }

    fn get_best_distance (&self, target: &NodeType, available_keys: &HashSet<char>) -> Option<i32> {
        if let Some(vector) = self.distances.get(target) {
            vector.iter().filter_map(|(key_set, dist)| if key_set.is_subset(available_keys) { Some(*dist) } else { None }).min()
        }
        else { None }
    }
}

fn create_node_of_interest (node_type: &NodeType, node_position: (i32, i32), whole_map: &HashMap<(i32, i32), Node>) -> NodeOfInterest {

    fn get_shortest_distances (
        current_position: (i32, i32), 
        current_distance: i32,
        current_keys_set: &HashSet<char>,
        whole_map: &HashMap<(i32, i32), Node>, 
        distances_to_positions: &mut HashMap<(i32, i32), Vec<(HashSet<char>, i32)>>, 
        distances: &mut HashMap<NodeType, Vec<(HashSet<char>, i32)>>) {

        fn insert_new_key_sets_to_distances (
            key_sets_to_distances: Vec<(HashSet<char>, i32)>, 
            current_node: &Node,
            current_position: (i32, i32),
            current_distance: i32,
            distances_to_positions: &mut HashMap<(i32, i32), Vec<(HashSet<char>, i32)>>, 
            distances: &mut HashMap<NodeType, Vec<(HashSet<char>, i32)>>) {
            distances_to_positions.insert(current_position, key_sets_to_distances.clone());
            if current_distance != 0 {
                match current_node.node_type {
                    NodeType::Key(_) => { distances.insert(current_node.node_type.clone(), key_sets_to_distances); },
                    _ => ()
                }
            }
        }

        let current_node = whole_map.get(&current_position).unwrap();
        
        // set current position into distance HashMaps
        if distances_to_positions.contains_key(&current_position) {
            let key_sets_to_distances = distances_to_positions.get(&current_position).unwrap();
            if key_sets_to_distances.iter().any(|(set, dist)| { set.is_subset(current_keys_set) && *dist < current_distance }) {
                return;
            }
            let new_key_sets_to_distances = key_sets_to_distances.clone();
            insert_new_key_sets_to_distances(new_key_sets_to_distances, current_node, current_position, current_distance, distances_to_positions, distances);
        }
        else {
            let key_sets_to_distances = vec![(current_keys_set.clone(), current_distance)];
            insert_new_key_sets_to_distances(key_sets_to_distances, current_node, current_position, current_distance, distances_to_positions, distances);
        }

        // update key set if on key 
        let current_keys_set = match current_node.node_type {
                NodeType::Door(c) => {
                    let mut new_keys_set = current_keys_set.clone();
                    new_keys_set.insert(c);
                    new_keys_set
                },
                _ => current_keys_set.clone()
            };

        // traverse next positions
        for (next_position, next_distance) in (&[current_node.up, current_node.down, current_node.left, current_node.right]).iter().filter_map(|opt| *opt) {
            
            get_shortest_distances(next_position, current_distance + next_distance, &current_keys_set, whole_map, distances_to_positions, distances);
        }
    }

    let mut distances: HashMap<NodeType, Vec<(HashSet<char>, i32)>> = HashMap::new();
    get_shortest_distances(node_position, 0, &HashSet::new(), whole_map, &mut HashMap::new(), &mut distances);


    NodeOfInterest {
        node_type: node_type.clone(),
        distances: distances
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
        determine_shortest_round_trip_4(&map).to_string()
    }
    fn task_1 (&self, input: &String) -> String {
        let map = parse(input);
        determine_shortest_round_trip_6(&map).to_string()
    }
}

fn determine_shortest_round_trip_6 (whole_map: &HashMap<(i32, i32), Node>) -> i32 {
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

fn determine_shortest_round_trip_5 (whole_map: &HashMap<(i32, i32), Node>) -> i32 {
    fn is_key (node_type: &NodeType) -> Option<char> {
        match node_type { NodeType::Key(c) => Some(*c), _ => None }
    }

    fn create_key_to_door (
        node: (i32, i32), 
        whole_map: &HashMap<(i32, i32), Node>) -> HashMap<char, Option<char>> {
        
        let mut heap: BinaryHeap<PathStateDirectAcyclicGraph> = BinaryHeap::new();
        heap.push(PathStateDirectAcyclicGraph { position: node, distance: 0, last_required_door: None});
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut result: HashMap<char, Option<char>> = HashMap::new();
    
        while let Some(current_state) = heap.pop() {
            visited.insert(current_state.position);
            let current_node = if let Some(node) = whole_map.get(&current_state.position) { node } else { panic!() };
            let mut current_last_required_door = current_state.last_required_door;
            match current_node.node_type {
                NodeType::Door(c) => current_last_required_door = Some(c),
                NodeType::Key(c) => { result.insert(c, current_last_required_door); },
                _ => ()
            }
            for (node_position, distance) in [current_node.up, current_node.down, current_node.left, current_node.right]
                .iter()
                .filter_map(|opt| *opt)
                .filter(|(pos, _)| !visited.contains(pos)){
                heap.push(PathStateDirectAcyclicGraph { position: node_position, distance: current_state.distance + distance, last_required_door: current_last_required_door});
            };
        }
    
        result
    }

    fn create_door_to_door (
        node: (i32, i32), 
        whole_map: &HashMap<(i32, i32), Node>) -> HashMap<char, Option<char>> {
        
        let mut heap: BinaryHeap<PathStateDirectAcyclicGraph> = BinaryHeap::new();
        heap.push(PathStateDirectAcyclicGraph { position: node, distance: 0, last_required_door: None});
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut result: HashMap<char, Option<char>> = HashMap::new();
    
        while let Some(current_state) = heap.pop() {
            visited.insert(current_state.position);
            let current_node = if let Some(node) = whole_map.get(&current_state.position) { node } else { panic!() };
            let mut current_last_required_door = current_state.last_required_door;
            match current_node.node_type {
                NodeType::Door(c) => { result.insert(c, current_last_required_door); current_last_required_door = Some(c);  },
                _ => ()
            }
            for (node_position, distance) in [current_node.up, current_node.down, current_node.left, current_node.right]
                .iter()
                .filter_map(|opt| *opt)
                .filter(|(pos, _)| !visited.contains(pos)){
                heap.push(PathStateDirectAcyclicGraph { position: node_position, distance: current_state.distance + distance, last_required_door: current_last_required_door});
            };
        }
    
        result
    }

    fn get_count_of_topological_sortings (available_keys: &mut HashSet<char>, last_key: Option<char>, key_to_door: &HashMap<char, Option<char>>, door_to_door: &HashMap<char, Option<char>>) -> i128 {
        if let Some(c) = last_key {
            available_keys.insert(c);
        }
        if available_keys.len() == 16 {
            available_keys.remove(&last_key.expect("impossibroh"));
            1
        }
        else {
            let mut count = 0;

            for c in (('a' as u8)..=('p' as u8)).map(|i| i as char) {
                let temp = key_to_door.get(&c).expect("not found in key to door");
                if !available_keys.contains(&c) && (temp.is_none() || available_keys.contains(&temp.expect("not found in available_keys"))) {
                    count = count + get_count_of_topological_sortings(available_keys, Some(c), key_to_door, door_to_door);
                }
            }

            if let Some(c) = last_key {
                available_keys.remove(&c);
            }
            count
        }
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

    let sequence = ['a', 'g', 'd', 'h',  'b', 'j', 'f', 'o', 'c', 'i', 'e', 'p', 'k', 'n', 'l', 'm'];

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

    let key_to_door = create_key_to_door(current_position, whole_map);

    let door_to_door = create_door_to_door(current_position, whole_map);

    println!("Key to Door: {:?}", key_to_door);
    println!("Door to Door: {:?}", door_to_door);
    println!("Topological Sorting: {}", get_count_of_topological_sortings(&mut HashSet::new(), None, &key_to_door, &door_to_door));

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

fn determine_shortest_round_trip_4 (whole_map: &HashMap<(i32, i32), Node>) -> i32 {
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

fn determine_shortest_round_trip_3 (whole_map: &HashMap<(i32, i32), Node>) -> i32 {
    fn is_non_normal (node_type: &NodeType) -> bool {
        match node_type { NodeType::Normal | NodeType::Door(_) => false, _ => true }
    }

    fn shortest_paths (
        node: (i32, i32), 
        current_owned_keys: &HashSet<char>, 
        whole_map: &HashMap<(i32, i32), Node>) -> HashMap<NodeType, i32> {
        
        let mut heap: BinaryHeap<PathState> = BinaryHeap::new();
        heap.push(PathState { position: node, distance: 0 });
        let mut distances_to_non_normals_map: HashMap<NodeType, i32> = HashMap::new();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
    
        while let Some(current_state) = heap.pop() {
            visited.insert(current_state.position);
            let current_node = whole_map.get(&current_state.position).unwrap();
            if is_non_normal(&current_node.node_type) {
                distances_to_non_normals_map.insert(current_node.node_type.clone(), current_state.distance);
            }
            if match current_node.node_type { NodeType::Door(c) => current_owned_keys.contains(&c), _ => true } {
                for (node_position, distance) in [current_node.up, current_node.down, current_node.left, current_node.right]
                    .iter()
                    .filter_map(|opt| *opt)
                    .filter(|(pos, _)| !visited.contains(pos)){
                    heap.push(PathState { position: node_position, distance: current_state.distance + distance });
                };
            }
        }
    
        distances_to_non_normals_map
    }

    let non_normals = whole_map
        .iter()
        .filter(|(_, node)| is_non_normal(&node.node_type))
        .map(|(position, node)| (node.node_type.clone(), *position))
        .collect::<HashMap<NodeType, (i32, i32)>>();

    let only_keys: HashSet<char> = non_normals.iter().filter_map(|(node_type, _)| match node_type { NodeType::Key(c) => Some(*c), _ => None }).collect();

    //println!("{:?}", non_normals);

    let mut heap: BinaryHeap<RouteState> = BinaryHeap::new();
    heap.push(RouteState {node_type: NodeType::StartingPoint, distance: 0, owned_keys: HashSet::new()});
    let mut best_distance = i32::MAX;
    let mut i = 0;

    while !heap.is_empty() {
        i = i + 1;
        let current_state_maybe = heap.pop();
        if let Some(current_state) = current_state_maybe {
            //println!("count of keys {}", current_state.owned_keys.len());
            let mut new_owned_keys = current_state.owned_keys.clone();
            match current_state.node_type { NodeType::Key(c) => { new_owned_keys.insert(c); }, _ => () }
            if only_keys == new_owned_keys {
                if current_state.distance < best_distance {
                    best_distance = current_state.distance;
                    println!("new best result {}", best_distance);
                }
            }
            else if let Some(current_position) = non_normals.get(&current_state.node_type) {
                let reachables: HashMap<NodeType, i32> = shortest_paths(
                    *current_position,
                    &new_owned_keys,
                    whole_map);
                for (new_node_type, dist) in reachables.iter().filter(|(node_type, _)| match node_type { NodeType::Key(c) => !new_owned_keys.contains(&c), _ => false }) {
                    let new_distance = current_state.distance + dist;
                    if new_distance < best_distance {
                        heap.push(RouteState { node_type: new_node_type.clone(), distance: new_distance, owned_keys: new_owned_keys.clone() });
                    }
                }
            }
        }
    }

    best_distance
}

fn determine_shortest_round_trip_2 (whole_map: &HashMap<(i32, i32), Node>) -> i32 {
    fn is_non_normal (node_type: &NodeType) -> bool {
        match node_type { NodeType::Normal | NodeType::Door(_) => false, _ => true }
    }

    fn shortest_paths (
        node: (i32, i32), 
        distance_so_far: i32,
        current_owned_keys: &HashSet<char>, 
        distances_map: &mut HashMap<(i32, i32), i32>, 
        whole_map: &HashMap<(i32, i32), Node>,
        distances_to_non_normals_map: &mut HashMap<NodeType, i32>) {
        fn check_path (
            neighbor: Option<((i32, i32), i32)>,
            distance_so_far: i32,
            current_owned_keys: &HashSet<char>, 
            distances_map: &mut HashMap<(i32, i32), i32>, 
            whole_map: &HashMap<(i32, i32), Node>, 
            distances_to_non_normals_map: &mut HashMap<NodeType, i32>) {
            if let Some((neighbor, distance)) = neighbor {
                if let Some(node) = whole_map.get(&neighbor) {
                    match node.node_type {
                        NodeType::Door(c) if !current_owned_keys.contains(&c) => { return; }
                        _ => ()
                    }
                }
                let distance_so_far = distance_so_far + distance;
                match distances_map.get(&neighbor) {
                    Some(dist) if *dist > distance_so_far => shortest_paths(neighbor, distance_so_far, current_owned_keys, distances_map, whole_map, distances_to_non_normals_map),
                    None => shortest_paths(neighbor, distance_so_far, current_owned_keys, distances_map, whole_map, distances_to_non_normals_map),
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
            check_path(node.up, distance_so_far, current_owned_keys, distances_map, whole_map, distances_to_non_normals_map);
            check_path(node.down, distance_so_far, current_owned_keys, distances_map, whole_map, distances_to_non_normals_map);
            check_path(node.left, distance_so_far, current_owned_keys, distances_map, whole_map, distances_to_non_normals_map);
            check_path(node.right, distance_so_far, current_owned_keys, distances_map, whole_map, distances_to_non_normals_map);
        }
    }

    let non_normals = whole_map
        .iter()
        .filter(|(_, node)| is_non_normal(&node.node_type))
        .map(|(position, node)| (node.node_type.clone(), *position))
        .collect::<HashMap<NodeType, (i32, i32)>>();

    let only_keys: HashSet<char> = non_normals.iter().filter_map(|(node_type, _)| match node_type { NodeType::Key(c) => Some(*c), _ => None }).collect();

    //println!("{:?}", non_normals);

    let mut heap: BinaryHeap<RouteState> = BinaryHeap::new();
    heap.push(RouteState {node_type: NodeType::StartingPoint, distance: 0, owned_keys: HashSet::new()});
    let mut best_distance = i32::MAX;
    let mut i = 0;

    while !heap.is_empty() {
        i = i + 1;
        let current_state_maybe = heap.pop();
        if let Some(current_state) = current_state_maybe {
            //println!("count of keys {}", current_state.owned_keys.len());
            let mut new_owned_keys = current_state.owned_keys.clone();
            match current_state.node_type { NodeType::Key(c) => { new_owned_keys.insert(c); }, _ => () }
            if only_keys == new_owned_keys {
                if current_state.distance < best_distance {
                    best_distance = current_state.distance;
                    println!("new best result {}", best_distance);
                }
            }
            else if let Some(current_position) = non_normals.get(&current_state.node_type) {
                let mut reachables: HashMap<NodeType, i32> = HashMap::new();
                shortest_paths(
                    *current_position,
                    0,
                    &new_owned_keys,
                    &mut HashMap::new(),
                    whole_map,
                    &mut reachables);
                for (new_node_type, dist) in reachables.iter().filter(|(node_type, _)| match node_type { NodeType::Key(c) => !new_owned_keys.contains(&c), _ => false }) {
                    let new_distance = current_state.distance + dist;
                    if new_distance < best_distance {
                        heap.push(RouteState { node_type: new_node_type.clone(), distance: new_distance, owned_keys: new_owned_keys.clone() });
                    }
                }
            }
        }
    }

    best_distance
}

fn determine_shortest_round_trip_1 (whole_map: &HashMap<(i32, i32), Node>) -> i32 {
    fn is_non_normal (node_type: &NodeType) -> bool {
        match node_type { NodeType::Normal | NodeType::Door(_) => false, _ => true }
    }

    let non_normals = whole_map
        .iter()
        .filter(|(_, node)| is_non_normal(&node.node_type))
        .map(|(position, node)| (node.node_type.clone(), create_node_of_interest(&node.node_type, *position, whole_map)))
        .collect::<HashMap<NodeType, NodeOfInterest>>();

    let min_dist = non_normals.iter().map(|(_, node_of_interest)| node_of_interest.get_min_distance()).min().unwrap();
    println!("min dist ist {}", min_dist);
    let only_keys: HashSet<char> = non_normals.iter().filter_map(|(node_type, _)| match node_type { NodeType::Key(c) => Some(*c), _ => None }).collect();

    //println!("{:?}", non_normals);

    let mut heap: BinaryHeap<RouteState> = BinaryHeap::new();
    heap.push(RouteState {node_type: NodeType::StartingPoint, distance: 0, owned_keys: HashSet::new()});
    let mut best_distance = i32::MAX;

    while !heap.is_empty() {
        let current_state_maybe = heap.pop();
        if let Some(current_state) = current_state_maybe {
            let mut new_owned_keys = current_state.owned_keys.clone();
            match current_state.node_type { NodeType::Key(c) => { new_owned_keys.insert(c); }, _ => () }
            if only_keys == new_owned_keys {
                if current_state.distance < best_distance {
                    best_distance = current_state.distance;
                    println!("new best result {}", best_distance);
                }
            }
            else if let Some(current_node_of_interest) = non_normals.get(&current_state.node_type) {
                let reachables = current_node_of_interest.get_reachable_nodes_of_interest(&new_owned_keys);
                for (new_node_type, dist) in reachables.iter().filter(|(node_type, _)| match node_type { NodeType::Key(c) => !new_owned_keys.contains(&c), _ => false }) {
                    let new_distance = current_state.distance + dist;
                    if new_distance + (only_keys.len() - new_owned_keys.len()) as i32 * min_dist < best_distance {
                        heap.push(RouteState { node_type: new_node_type.clone(), distance: new_distance, owned_keys: new_owned_keys.clone() });
                    }
                }
            }
        }
    }

    best_distance
}

fn determine_shortest_round_trip_0 (whole_map: &HashMap<(i32, i32), Node>) -> i32 {
    fn is_non_normal (node_type: &NodeType) -> bool {
        match node_type { NodeType::Normal | NodeType::Door(_) => false, _ => true }
    }

    fn get_shortest_route (
        current_node_of_interest: &NodeOfInterest, 
        current_distance: i32,
        current_keys_set: &HashSet<char>,
        current_needed_to_visit: &HashSet<NodeType>,
        node_of_interest_map: &HashMap<NodeType, NodeOfInterest>,
        current_best_result: &mut Option<i32>) {

        if let Some(current_best_result) = current_best_result {
            if *current_best_result < current_distance { return; }
        }

        let mut current_needed_to_visit = current_needed_to_visit.clone();
        current_needed_to_visit.remove(&current_node_of_interest.node_type);
        
        if current_needed_to_visit.len() == 0 {
            if let Some(current_best_result_number) = current_best_result {
                if *current_best_result_number > current_distance { println!("new best result {}", current_distance); *current_best_result = Some(current_distance) }
            }
            else { println!("new best result {}", current_distance); *current_best_result = Some(current_distance) }
            return;
        }

        let current_keys_set = match current_node_of_interest.node_type {
            NodeType::Key(c) => {
                let mut new_keys_set = current_keys_set.clone();
                new_keys_set.insert(c);
                new_keys_set
            },
            _ => current_keys_set.clone()
        };
        
        let mut next_nodes = current_node_of_interest
            .get_reachable_nodes_of_interest(&current_keys_set)
            .iter()
            .filter(|(node_type, _)| current_needed_to_visit.contains(&node_type))
            .map(|(node_type, dist)| (node_type.clone(), *dist))
            .collect::<Vec<(NodeType, i32)>>();

        next_nodes.sort_by_key(|(_, dist)| *dist);

        if next_nodes.len() > 0 {
            for (next_node_type, next_dist) in next_nodes.iter() {
                if let Some(next_node_of_interest) = node_of_interest_map.get(next_node_type) {
                    get_shortest_route(next_node_of_interest, current_distance + *next_dist, &current_keys_set, &current_needed_to_visit, node_of_interest_map, current_best_result)
                }
            }
        }        
    }

    let non_normals = whole_map
        .iter()
        .filter(|(_, node)| is_non_normal(&node.node_type))
        .map(|(position, node)| (node.node_type.clone(), create_node_of_interest(&node.node_type, *position, whole_map)))
        .collect::<HashMap<NodeType, NodeOfInterest>>();

    //println!("{:?}", non_normals);

    let mut result: Option<i32> = None;
    get_shortest_route(
        non_normals.get(&NodeType::StartingPoint).expect("can't find starting point"),
        0,
        &HashSet::new(),
        &non_normals.iter().map(|(node_type, _)| node_type.clone()).collect::<HashSet<NodeType>>(),
        &non_normals,
        &mut result);

    result.expect("no result")
}

fn determine_shortest_round_trip (whole_map: &HashMap<(i32, i32), Node>) -> i32 {
    fn is_non_normal (node_type: &NodeType) -> bool {
        match node_type { NodeType::Normal | NodeType::Door(_) => false, _ => true }
    }
    fn get_shortest_route (
        current_non_normal: &NodeType,
        distance_so_far: i32,
        current_owned_keys: &HashSet<NodeType>,
        non_normal_positions: &HashMap<NodeType, (i32, i32)>,
        whole_map: &HashMap<(i32, i32), Node>,
        best_result_sofar: &mut Option<i32>) {
        fn shortest_paths (
            node: (i32, i32), 
            distance_so_far: i32,
            current_owned_keys: &HashSet<NodeType>, 
            distances_map: &mut HashMap<(i32, i32), i32>, 
            whole_map: &HashMap<(i32, i32), Node>,
            distances_to_non_normals_map: &mut HashMap<NodeType, i32>) {
            fn check_path (
                neighbor: Option<((i32, i32), i32)>,
                distance_so_far: i32,
                current_owned_keys: &HashSet<NodeType>, 
                distances_map: &mut HashMap<(i32, i32), i32>, 
                whole_map: &HashMap<(i32, i32), Node>, 
                distances_to_non_normals_map: &mut HashMap<NodeType, i32>) {
                if let Some((neighbor, distance)) = neighbor {
                    if let Some(node) = whole_map.get(&neighbor) {
                        match node.node_type {
                            NodeType::Door(c) if !current_owned_keys.contains(&NodeType::Key(c)) => { return; }
                            _ => ()
                        }
                    }
                    let distance_so_far = distance_so_far + distance;
                    match distances_map.get(&neighbor) {
                        Some(dist) if *dist > distance_so_far => shortest_paths(neighbor, distance_so_far, current_owned_keys, distances_map, whole_map, distances_to_non_normals_map),
                        None => shortest_paths(neighbor, distance_so_far, current_owned_keys, distances_map, whole_map, distances_to_non_normals_map),
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
                check_path(node.up, distance_so_far, current_owned_keys, distances_map, whole_map, distances_to_non_normals_map);
                check_path(node.down, distance_so_far, current_owned_keys, distances_map, whole_map, distances_to_non_normals_map);
                check_path(node.left, distance_so_far, current_owned_keys, distances_map, whole_map, distances_to_non_normals_map);
                check_path(node.right, distance_so_far, current_owned_keys, distances_map, whole_map, distances_to_non_normals_map);
            }
        }

        /*fn get_reachable_keys (
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
        }*/

        if let Some(best_distance_so_far) = best_result_sofar {
            if *best_distance_so_far <= distance_so_far { return; }
        }

        let mut reachable_keys: HashMap<NodeType, i32> = HashMap::new();
        shortest_paths(
            *non_normal_positions.get(current_non_normal).unwrap(),
            0,
            current_owned_keys,
            &mut HashMap::new(),
            whole_map,
            &mut reachable_keys);
        let mut next_keys = reachable_keys.iter().filter(|(node_type, _)| **node_type != NodeType::StartingPoint && !current_owned_keys.contains(&node_type)).map(|(node_type, _)| node_type.clone()).collect::<Vec<NodeType>>();
        
        if next_keys.len() > 0 {
            &next_keys.sort_by_key(|key| reachable_keys.get(&key).expect("key not found"));
            for key in next_keys {
                let distance = reachable_keys.get(&key).unwrap();
                let mut new_current_owned_keys: HashSet<NodeType> = current_owned_keys.clone();
                new_current_owned_keys.insert(key.clone());
                get_shortest_route(&key, distance_so_far + distance, &new_current_owned_keys, non_normal_positions, whole_map, best_result_sofar);
            }
        }
        else { println!("best so far {}", distance_so_far); *best_result_sofar = Some(distance_so_far) }
    }

    let non_normals = whole_map
        .iter()
        .filter(|(_, node)| is_non_normal(&node.node_type))
        .map(|(position, node)| (node.node_type.clone(), *position))
        .collect::<HashMap<NodeType, (i32, i32)>>();

    let mut result: Option<i32> = None; 
    get_shortest_route(
        &NodeType::StartingPoint, 
        0,
        &HashSet::new(), 
        &non_normals, 
        whole_map,
        &mut result);
    result.expect("No result!")
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

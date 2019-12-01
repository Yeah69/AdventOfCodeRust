use crate::day_tasks;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use num_integer::lcm;

pub struct Day12;

impl day_tasks::DayTasks for Day12 {
    fn day_number (&self) -> String {
        "12".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let moons = input
            .lines()
            .filter_map(parse_to_moon)
            .collect::<Vec<Moon>>();
        let mut moons = Moons { moons: moons};
        for _ in 0..1_000 {
            moons.apply_gravity();
            moons.apply_velocity();
        }
        moons.calculate_energy().to_string()
    }
    fn task_1 (&self, input: &String) -> String {
        let moons = input
            .lines()
            .filter_map(parse_to_moon)
            .collect::<Vec<Moon>>();
        let mut moons = Moons { moons: moons};
        while !moons.check_for_sequence() {
            moons.apply_gravity();
            moons.apply_velocity();
        }
        let mut rates = HashSet::new();
        moons.collect_rates(&mut rates);
        let mut set = rates.iter().map(|i| *i as i128).collect::<HashSet<i128>>();
        lowest_common_multiple(&mut set);
        
        set.iter().next().map(|i| i.to_string()).unwrap_or("- Something went wrong -".to_string())
    }
}

fn lowest_common_multiple (set: &mut HashSet<i128>) {
    while set.len() != 1 {
        *set = set
            .iter()
            .zip(set
                .iter()
                .skip(1))
            .map(|(x, y)| {
                let x = *x;
                let y = *y;
                lcm(x, y)})
            .collect::<HashSet<i128>>();
    }
}

struct Moons {
    moons: Vec<Moon>
}

impl Moons {
    fn apply_gravity (&mut self) {
        let for_other = self.moons.to_vec();
        for moon in &mut self.moons {
            for other in &for_other {
                moon.apply_gravity(&other);
            }
        }
    }

    fn apply_velocity (&mut self) {
        for moon in &mut self.moons {
            moon.apply_velocity();
        }
    }

    fn calculate_energy (&self) -> i32 {
        self.moons.iter().map(Moon::calculate_energy).sum()
    }

    fn check_for_sequence (&mut self) -> bool {
        let mut check = true;
        for moon in &mut self.moons {
            check = moon.check_for_sequence() && check;
        }
        check
    }

    fn collect_rates (&self, set: &mut HashSet<i32>) {
        for moon in &self.moons {
            moon.collect_rates(set);
        }
    }
}

#[derive(Clone)]
struct Moon {
    position: (i32, i32, i32),
    velocity: (i32, i32, i32),
    x_sequence: Vec<i32>,
    y_sequence: Vec<i32>,
    z_sequence: Vec<i32>,
    x_rate: Option<i32>,
    y_rate: Option<i32>,
    z_rate: Option<i32>
}

impl Moon {
    fn apply_gravity (&mut self, other: &Moon) {
        fn logic (self_coordinate: i32, other_coordinate: i32) -> i32 {
            if self_coordinate < other_coordinate { 1 }
            else if self_coordinate > other_coordinate  { -1 }
            else { 0 }
        }
        self.velocity = (self.velocity.0 + logic(self.position.0, other.position.0), 
            self.velocity.1 + logic(self.position.1, other.position.1), 
            self.velocity.2 + logic(self.position.2, other.position.2));
    }

    fn apply_velocity (&mut self) {
        self.position = (self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
            self.position.2 + self.velocity.2)
    }

    fn calculate_energy (&self) -> i32 {
        (self.position.0.abs() + self.position.1.abs() + self.position.2.abs()) *
        (self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs())
    }

    fn check_for_sequence (&mut self) -> bool {
        fn check (new_value: i32, sequence: &mut Vec<i32>, rate: &mut Option<i32>) {
            if rate.is_none() {
                sequence.push(new_value);
                if sequence.len() % 2 == 0
                {
                    let first_half = &sequence[0..(sequence.len()/2)];
                    let second_half = &sequence[(sequence.len()/2)..sequence.len()];

                    let mut same = true;

                    for i in 0..first_half.len() {
                        if first_half[i] != second_half[i] {
                            same = false;
                            break;
                        }
                    }
                    
                    if same {
                        *rate = Some(first_half.len() as i32);
                        sequence.clear();
                    }
                }
            }
        }

        check(self.position.0, &mut self.x_sequence, &mut self.x_rate);
        check(self.position.1, &mut self.y_sequence, &mut self.y_rate);
        check(self.position.2, &mut self.z_sequence, &mut self.z_rate);
        self.x_rate.is_some() && self.y_rate.is_some() && self.z_rate.is_some()
    }

    fn collect_rates (&self, set: &mut HashSet<i32>) {
        for i in [self.x_rate, self.y_rate, self.z_rate].iter().filter_map(|x| *x) {
            set.insert(i);
        }
    }
}

fn parse_to_moon(text: &str) -> Option<Moon> {
    lazy_static! {
        static ref RE: Regex = Regex::new("<x=(.*), y=(.*), z=(.*)>").unwrap();
    }
    RE
        .captures(text)
        .map(|captures| {
            let x = captures
                .get(1)
                .map(|text_x| text_x.as_str().parse::<i32>().unwrap_or(0))
                .unwrap();
            let y = captures
                .get(2)
                .map(|text_y| text_y.as_str().parse::<i32>().unwrap_or(0))
                .unwrap();
            let z = captures
                .get(3)
                .map(|text_z| text_z.as_str().parse::<i32>().unwrap_or(0))
                .unwrap();
            Moon{
                position: (x, y, z), 
                velocity: (0, 0, 0),
                x_sequence: Vec::new(),
                y_sequence: Vec::new(),
                z_sequence: Vec::new(),
                x_rate: None,
                y_rate: None,
                z_rate: None
            }
        })
}

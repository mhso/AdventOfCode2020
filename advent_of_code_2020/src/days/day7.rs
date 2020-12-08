extern crate util;
extern crate queues;
use queues::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn print_graph(map: &HashMap<String, Vec<&str>>) {
    for (k, v) in map {
        print!("{}: [", k);
        for x in v {
            print!("{}, ", x);
        }
        println!("]");
    }
}

fn bfs(map: &HashMap<String, Vec<&str>>, initial: &str) -> usize {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut queue:Queue<&str> = queue![initial];

    while queue.size() > 0 {
        let node = queue.remove().unwrap();
        if !visited.contains(node) {
            visited.insert(node);
            match map.get(node) {
                Some(neighbors) => {
                    for neighbor in neighbors {
                        queue.add(neighbor).unwrap();
                    }
                },
                None => continue
            }
        }
    }
    return visited.len() - 1;
}

pub fn part_one() -> () {
	let lines = util::read_input(7);
    let list: Vec<&str> = lines.split("\n").collect();

    let mut map: HashMap<String, Vec<&str>> = HashMap::new();

    for i in 0..list.len()-1 {
        let line = list[i].trim();
        let split: Vec<&str> = line.split(" contain ").collect();

        let inner_bags : Vec<&str> = split[1].split(", ").collect();

        for i in 0..inner_bags.len() {
            let split_inner: Vec<&str> = inner_bags[i].split(" ").collect();
            if split_inner[0] != "no" {
                let inner_bag = split_inner[1..split_inner.len()-1].join(" ").trim().to_string();
                if !map.contains_key::<String>(&inner_bag) {
                    map.insert(inner_bag, Vec::new());
                }
            }
        }
    }

    for i in 0..list.len()-1 {
        let line = list[i].trim();
        let split: Vec<&str> = line.split(" contain ").collect();

        let outer_bags : Vec<&str> = split[0].split("bags").collect();
        let outer_bag = outer_bags[0].trim();

        let inner_bags : Vec<&str> = split[1].split(", ").collect();

        for i in 0..inner_bags.len() {
            let split_inner: Vec<&str> = inner_bags[i].split(" ").collect();
            if split_inner[0] != "no" {
                let inner_bag = &split_inner[1..split_inner.len()-1].join(" ").trim().to_string();
                let list = map.get_mut::<str>(inner_bag).unwrap();
                list.push(outer_bag);
            }
        }
    }
    
    let visits = bfs(&map, "shiny gold");
    println!("{}", visits);
}

fn bfs_again(map: &HashMap<&str, Vec<(String, u32)>>, initial: &str) -> u32 {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut queue:Queue<&str> = queue![initial];
    let mut count = 0;

    while queue.size() > 0 {
        let node = queue.remove().unwrap();
        if !visited.contains(node) {
            visited.insert(node);
            match map.get(node) {
                Some(neighbors) => {
                    for (neighbor, amount) in neighbors {
                        count += amount + 1;
                        queue.add(neighbor).unwrap();
                    }
                },
                None => continue
            }
        }
    }
    return count;
}

fn size_recurse(map: &HashMap<&str, Vec<(String, u32)>>, elem: &str) -> u32 {
    let mut count = 0;
    return match map.get(elem) {
        Some(neighbors) => {
            for (neighbor, amount) in neighbors {
                count += amount + amount * size_recurse(map, neighbor);
            }
            count
        },
        None => 0
    };
}

pub fn part_two() -> () {
	let lines = util::read_input(7);
    let list: Vec<&str> = lines.split("\n").collect();
    
    let mut map: HashMap<&str, Vec<(String, u32)>> = HashMap::new();

    for i in 0..list.len()-1 {
        let line = list[i].trim();
        let split: Vec<&str> = line.split(" contain ").collect();

        let outer_bags : Vec<&str> = split[0].split("bags").collect();
        let outer_bag = outer_bags[0].trim();

        map.insert(outer_bag, Vec::new());
    }

    for i in 0..list.len()-1 {
        let line = list[i].trim();
        let split: Vec<&str> = line.split(" contain ").collect();

        let outer_bags : Vec<&str> = split[0].split("bags").collect();
        let outer_bag = outer_bags[0].trim();
        let list = map.get_mut::<str>(outer_bag).unwrap();

        let inner_bags : Vec<&str> = split[1].split(", ").collect();

        for i in 0..inner_bags.len() {
            let split_inner: Vec<&str> = inner_bags[i].split(" ").collect();
            if split_inner[0] != "no" {
                let inner_bag = split_inner[1..split_inner.len()-1].join(" ").trim().to_string();
                let amount : u32 = split_inner[0].parse().unwrap();
                list.push((inner_bag, amount));
            }
        }
    }

    println!("{}", map["shiny gold"].len());

    let bags = size_recurse(&map, "shiny gold");
    println!("{}", bags);
}
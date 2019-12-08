use std::collections::HashMap;

fn main() {
    let content = include_str!("input.txt");

    let universe = Universe::new(&content);

    let part1 = universe.total_num_orbits();
    println!("1: {}", part1);
    let part2 = universe.orbital_transfers();
    println!("2: {}", part2);
}

struct Universe {
    reverse: HashMap<String, String>,
}

impl Universe {
    fn new(content: &str) -> Universe {
        let mut universe = Universe { reverse: HashMap::new() };

        for l in content.lines() {
            let mut p = l.trim().split(')');
            let center = String::from(p.next().unwrap());
            let planet = String::from(p.next().unwrap());

            universe.reverse.insert(planet, center);
        }

        universe
    }

    fn num_orbits(&self, planet: &str) -> usize {
        if planet == "COM" {
            return 0;
        }
        let center = self.reverse.get(planet).unwrap();
        return self.num_orbits(center) + 1;
    }

    fn total_num_orbits(&self) -> usize {
        self.reverse.keys().map(|p| self.num_orbits(&p)).sum()
    }

    fn path(&self, planet: &str) -> Vec<String> {
        let mut pos = String::from(planet);
        let mut res = Vec::new();
        loop {
            pos = self.reverse.get(&pos).unwrap().to_string();
            res.push(pos.clone());
            if pos == "COM" {
                return res;
            }
        }
    }

    fn orbital_transfers(&self) -> usize {
        let mut path1 = self.path("YOU");
        let mut path2 = self.path("SAN");

        loop {
            let last1 = path1.pop().unwrap();
            let last2 = path2.pop().unwrap();

            if last1 != last2 {
                break;
            }
        }
       
        path1.len() + path2.len() + 2
    }
}

#[test]
fn test_part1() {
    let universe = Universe::new("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\n");
    assert_eq!(universe.total_num_orbits(), 42);
}

#[test]
fn test_part2() {
    let universe = Universe::new("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN\n");
    assert_eq!(universe.orbital_transfers(), 4);
}
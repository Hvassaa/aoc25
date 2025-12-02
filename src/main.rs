use std::fs;
use std::ops::Add;
use std::ops::Sub;

fn main() {
    first();
    second();
}

#[derive(Clone)]
struct Field100(u32);

impl Field100 {
    fn new(v: u32) -> Self {
        Field100(v % 100)
    }
}

impl Add<Field100> for Field100 {
    type Output = (Field100, bool);

    fn add(self, rhs: Field100) -> Self::Output {
        let a = self.0 + rhs.0;
        (Field100(a % 100), a > 99)
    }
}

impl Sub<Field100> for Field100 {
    type Output = (Field100, bool);

    fn sub(self, rhs: Field100) -> Self::Output {
        if rhs.0 > self.0 {
            let a = (rhs.0 - self.0) % 100;
            (Field100(100 - a), true)
        } else {
            (Field100(self.0 - rhs.0), false)
        }
    }
}

fn first() {
    let lines = read_lines_of_file("1.txt");

    let res = lines.iter().fold((0, Field100(50)), |acc, l| {
        let current_0s = acc.0;
        let current_field_value = acc.1;

        let (dir, val_str) = l.split_at(1);
        let unsigned_val: u32 = val_str.parse().unwrap();
        let new_field_value = Field100::new(unsigned_val);

        let (new_rotation, _) = match dir {
            "L" => current_field_value - new_field_value,
            "R" => current_field_value + new_field_value,
            _ => panic!(),
        };

        let is_0 = (new_rotation.0 == 0) as u32;
        (current_0s + is_0, new_rotation)
    });

    println!("{}", res.0);
}

fn second() {
    let lines = read_lines_of_file("1.txt");

    let res = lines.iter().fold((0, Field100(50)), |acc, l| {
        let current_0s = acc.0;
        let current_field_value = acc.1;

        let (dir, val_str) = l.split_at(1);
        let unsigned_val: u32 = val_str.parse().unwrap();
        let new_field_value = Field100::new(unsigned_val);

        let full_rotations = unsigned_val / 100;

        let (new_rotation, overflow) = match dir {
            "L" => current_field_value.clone() - new_field_value,
            "R" => current_field_value.clone() + new_field_value,
            _ => panic!(),
        };

        let current_rotation = current_field_value.0;
        let final_passed_0 = (current_rotation != 0 && (overflow || new_rotation.0 == 0)) as u32;
        (current_0s + full_rotations + final_passed_0, new_rotation)
    });

    println!("{}", res.0);
}

fn read_lines_of_file(path: &str) -> Vec<String> {
    let file = fs::read_to_string(path).unwrap();
    let lines = file.lines();
    return lines.map(|line| line.to_string()).collect();
}

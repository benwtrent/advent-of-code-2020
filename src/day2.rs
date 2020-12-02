#[derive(Debug, Eq, PartialEq)]
pub struct GiftBox {
    l: usize,
    w: usize,
    h: usize,
} 

impl From<&str> for GiftBox {
    fn from(s: &str) -> Self {
        let coordinates: Vec<usize> = s.split("x").map(|s| s.parse().unwrap()).collect();
        if coordinates.len() < 3 {
            panic!(format!("Unexpected input {}", s));
        }
        GiftBox {
            l: coordinates[0],
            w: coordinates[1],
            h: coordinates[2],
        }
    }
}

impl GiftBox {
    pub fn new() -> Self {
        GiftBox {
            l: 0,
            w: 0,
            h: 0
        }
    }
    pub fn merge(mut self, other: &GiftBox) -> Self {
        self.h += other.h;
        self.w += other.w;
        self.l += other.l;
        self
    }
    
    pub fn paper(&self) -> usize {
        let v = vec![self.l*self.w, self.w*self.h, self.h*self.l];
        let min = v.iter().min().unwrap();
        let sum = v.iter().map(|i| i*2).sum::<usize>();
        sum + min
    }
    
    pub fn side_perimeters(&self) -> Vec<usize> {
        vec![
        self.l * 2 + self.h * 2,
        self.w * 2 + self.h * 2,
        self.l * 2 + self.w * 2,
        ]
    }
    
    pub fn volume(&self) -> usize {
        self.l * self.w * self.h
    }
}

#[aoc_generator(day2)]
fn input_to_vec(input: &str) -> Vec<GiftBox> {
    input.lines().map(|i| i.into()).collect()
}


#[aoc(day2, part2)]
fn needed_ribbon(input: &Vec<GiftBox>) -> usize {
    let mut total = 0;
    for i in input {
        total += i.side_perimeters().iter().min().unwrap() + i.volume();
    }
    total
}

#[aoc(day2, part1)]
fn needed_paper(input: &Vec<GiftBox>) -> usize {
    let mut total = 0;
    for i in input {
        total += i.paper();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn parser_test() {
        let g: GiftBox = "2x5x4".into();
        assert_eq!(g, GiftBox{l: 2, w: 5, h: 4})
    }
    
    #[test]
    fn test_paper() {
        let g = GiftBox{l: 2, w: 3, h: 4};
        assert_eq!(g.paper(), 58);
        let g = GiftBox{l: 1, w: 1, h: 10};
        assert_eq!(g.paper(), 43);
    }
    
    #[test]
    fn smallest_perimeter() {
        let g = GiftBox{l: 2, w: 3, h: 4};
        assert_eq!(g.side_perimeters().iter().min().unwrap(), &(10 as usize));
        let g = GiftBox{l: 1, w: 1, h: 10};
        assert_eq!(g.side_perimeters().iter().min().unwrap(), &(4 as usize));
    }
    
}
fn main() {
    let content = include_str!("./input.txt");

    let mut image = Image::new(25, 6);
    image.fill(&content);

    println!("1: {}", image.checksum());
    println!("2: ");
    for line in image.render() {
      println!("{}", line);
    }
}

struct Image {
    w: usize,
    h: usize,
    layers: Vec<Vec<u32>>,
}

impl Image {
    fn new(w: usize, h: usize) -> Image {
        Image { w, h, layers: Vec::new() }
    }

    fn fill(&mut self, content: &str) {
        let mut ch = content.trim().chars();
        let mut layer: Vec<u32>;

        loop {
            layer = Vec::new();

            for _ in 0..(self.h) {
                for _ in 0..(self.w) {
                    let c = ch.next();
                    if c.is_none() {
                        return;
                    }
                    layer.push(c.unwrap().to_digit(10).unwrap())
                }
            }
            self.layers.push(layer);
        }
    }

    fn checksum(&self) -> usize {
        let (i,_layer) = self.layers.iter()
            .enumerate()
            .min_by_key(|(_i,layer)| layer.iter().filter(|&&pixel| pixel == 0).count())
            .unwrap();

        let ones = self.layers.get(i).unwrap().iter().filter(|&&pixel| pixel == 1).count();
        let twos = self.layers.get(i).unwrap().iter().filter(|&&pixel| pixel == 2).count();

        ones * twos
    }

    fn render(&self) -> Vec<String> {
        let mut result = Vec::new();

        for y in 0..(self.h) {
            let mut string = String::new();
            for x in 0..(self.w) {
                for l in &self.layers {
                    let pixel = l.get(y * self.w + x).unwrap();
                    match pixel {
                        0 => { string.push_str("."); break; },
                        1 => { string.push_str("W"); break; },
                        _ => {},
                    }
                }
            }
            result.push(string);
        }

        result
    }
}
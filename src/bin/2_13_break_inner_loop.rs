fn main() {
    'outer: for x in 0.. {
        for y in 0.. {
            for z in 0.. {
                if x + y + z > 1000 {
                    println!("x: {}, y: {}, z: {}", x, y, z);
                    break 'outer;
                }
            }
        }
    }
}

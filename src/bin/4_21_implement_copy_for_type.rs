#[derive(Debug)]
struct CubeSat {
    id: u32,
}

impl Copy for CubeSat {}

impl Clone for CubeSat {
    fn clone(&self) -> Self {
        CubeSat { id: self.id }
    }
}

#[derive(Debug)]
enum StatusMessage {
    OK,
}

impl Copy for StatusMessage {}

impl Clone for StatusMessage {
    fn clone(&self) -> Self {
        *self
    }
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::OK
}

fn main() {
    let sat_a = CubeSat { id: 0 };

    let a_status = check_status(sat_a);
    let a_status = check_status(sat_a);
}

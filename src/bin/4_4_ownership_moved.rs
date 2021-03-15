#[derive(Debug)]
struct CubeSat {
    id: u32,
}

#[derive(Debug)]
enum StatusMessage {
    OK,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::OK
}

fn main() {
    let sat_a = CubeSat { id: 0 };

    let a_status = check_status(sat_a);
    let a_status = check_status(sat_a);
}

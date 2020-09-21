type Message = String;

#[derive(Debug)]
struct Mailbox {
  messages: Vec<Message>,
}

impl Mailbox {
  fn new() -> Mailbox {
    Mailbox {
      messages: Vec::new(),
    }
  }

  fn write(&mut self, msg: String) {
    self.messages.push(msg);
  }

  fn read(&mut self) -> Message {
    self.messages.pop().expect("No messages left!")
  }
}

#[derive(Debug)]
struct CubeSat {
  id: u64,
  mailbox: Mailbox,
}

impl CubeSat {
  fn new(id: u64) -> CubeSat {
    CubeSat {
      id,
      mailbox: Mailbox::new(),
    }
  }

  fn recv(&mut self) {
    println!(
      "Satelite {} received message!: {}",
      self.id,
      self.mailbox.read()
    )
  }
}

#[derive(Debug)]
struct GroundStation {}

impl GroundStation {
  fn new() -> GroundStation {
    GroundStation {}
  }

  fn connect(&self, sat_id: u64) -> CubeSat {
    CubeSat::new(sat_id)
  }

  fn send(&self, sat: &mut CubeSat, msg: Message) {
    sat.mailbox.write(msg)
  }
}

fn fetch_sat_ids() -> Vec<u64> {
  vec![1, 2, 3]
}

fn main() {
  let ids = fetch_sat_ids();
  let station = GroundStation::new();

  for id in ids {
    let mut satelite = station.connect(id);

    station.send(&mut satelite, String::from("Hello to the sattelite"));
    satelite.recv();
  }
}

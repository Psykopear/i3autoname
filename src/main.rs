use i3ipc::I3EventListener;
use i3ipc::Subscription;
use i3ipc::event::{Event, inner::WindowChange};

mod icons;

fn main() {
    // establish connection.
    let mut listener = I3EventListener::connect().unwrap();

    // subscribe to a couple events.
    let subs = [Subscription::Window];
    listener.subscribe(&subs).unwrap();

    // handle them
    for event in listener.listen() {
        match event.unwrap() {
            Event::WindowEvent(e) => {
                match e.change {
                    WindowChange::New => {
                        println!("{}", icons::TERMINAL);
                        println!("window created");
                    },
                    WindowChange::Close => {
                        println!("window closed");
                    },
                    WindowChange::Move => {
                        println!("window moved");
                    },
                    _ => (),
                }
            },
            _ => unreachable!()
        }
    }
}


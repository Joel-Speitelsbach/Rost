/*////////// Grundgerüst ///////////////
fn run (args &[String]) {
    let battlefield = Bf.new();
    loop {
        for serverMessage in serverMessages {
            for actions in serverMessage {
                battlefield.alter(action);
            }
            battlefield.stride();
        }
        let presenter = Presenter::new(&mut battlefield);
        presenter.present();
        for event in poll_events() {
            controller.useEvent(&event);
            presenter.useEvent(&event);
        }
        let actions = controller.takeActions();
        sendMessageToServer(actions);
    }
}
//////////////////////////////////// */

use std::thread::sleep;
use std::time::Duration;

use network;
use super::message::{ServerMessage,ClientMessage,PlayerID,ServerMessageInit};

pub fn run(opts: &[String]) {
    println!("opts: {:?}", opts);
    
    // connect to server
    let other = match network::Simple::connect_to_server("127.0.0.1:8080") {
        Ok(ok) => ok,
        Err(err) => {
            println!("failed to connect to server");
            println!("debug info: {}", err);
            return;
        },
    };
    
    // recieve init message
    other.set_nonblocking(false);
    let initMsg: ServerMessageInit = network::Simple::recieve(&other)
        .expect("failed to recieve init msg");
    other.set_nonblocking(true);
    println!("initMsg.player_id: {:?}", initMsg.player_id);
    let ServerMessageInit {player_id: my_player_id,..} = initMsg;
    
    // init client state
    let mut counter = 0;
    
    // main loop
    loop {
        
        // recieve
        if let Ok(msg) = network::Simple::recieve(&other) {
            let msg: ServerMessage = msg;
            println!("server: {:?}", &msg);
        }
        
        // send
        let msg = ClientMessage {
            actions: vec!(),
        };
        counter += 1;
        if let Err(err) = network::Simple::send(&other, &msg) {
            // connection lost
            println!("server disconnected");
            println!("debug info: {}", err);
            break;
        }
        
        sleep(Duration::from_millis(50));
    }
}

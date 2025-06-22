use mini_redis::{Connection,Frame};
use tokio::net::{TcpListener, TcpStream};
use bytes::Bytes;
use std::sync::{Arc,Mutex};
use std::collections::HashMap;

type Db = Arc<Mutex<HashMap<String,Bytes>>>;

#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop{
        let (socket,_) = listener.accept().await.unwrap();
        let db = db.clone();

        tokio::spawn(async move{
            process(socket,db).await;
        });
    }

    async fn process(socket: TcpStream, db: Db){
        use mini_redis::Command::{self,Get,Set};
        use std::collections::HashMap;

        let mut db = Arc::new(Mutex::new(HashMap::new()));
        let mut connection = Connection::new(socket);

        while let Some(frame) = connection.read_frame().await.unwrap() {
            let response = match Command::from_frame(frame).unwrap(){
                Set(set_cmd) => {
                    let mut db = db.lock().unwrap();
                    db.insert(
                        set_cmd.key().to_string(),
                        set_cmd.value().to_vec()
                    );
                    Frame::Simple(String::from("Ok"))
                }
                Get(get_cmd) => {
                    let mut db = db.lock().unwrap();
                    if let Some(val) = db.get(get_cmd.key()){
                        Frame::Bulk(val.clone().into())
                    } else {
                        Frame::Null
                    }
                }
                cmd => panic!("unimplemented : {:?}", cmd)
            };
            // e respond to sender
            connection.write_frame(&response).await.unwrap();
        }
    }
}
extern crate protobuf;

mod lib_types;
mod user_types;

use protobuf::Message;
use user_types::proto::mymsg::MyMsg;

fn main() {
    let hash = lib_types::Hash { data: [99; 8] };

    //setting pb message from our types
    let mut in_msg = MyMsg::default();
    in_msg.set_hash(lib_types::to_pb(hash));
    in_msg.set_my_field(88);

    //serializing
    let mut out = Vec::new();
    in_msg.write_to_vec(&mut out).unwrap();

    println!("Ser. msg: {:?}", out);

    //deserializing
    let mut out_msg = MyMsg::default();
    out_msg.merge_from_bytes(out.as_ref()).unwrap();

    println!("Deser. msg: {:?}", out_msg);

    //extract out type from pb msg
    let lib_hash = lib_types::from_pb(out_msg.take_hash()).unwrap();
    println!("Deser. lib hash: {:?}", lib_hash);
}

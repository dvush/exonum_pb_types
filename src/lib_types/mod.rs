pub mod proto;

use protobuf::Message;

pub struct Hash {
    data: [u32; 8]
}

pub fn to_pb(from: Hash) -> proto::hash::Hash {
    let mut out = proto::hash::Hash::default();
    out.set_data(from.data.to_vec());
    out
}

pub fn from_pb(from: proto::hash::Hash) -> Result<Hash,()> {
    if !from.is_initialized() {
        return Err(());
    }
    let data = from.get_data();
    if data.len() != 8 {
        return Err(());
    }
    let out = Hash { data: [data[0],data[1],data[2],data[3],data[4],data[5],data[6],data[7]] };
    Ok(out)
}

use crate::net::datatype::datatype::Angle;

//PacketID 0x02
//Clientbound

//这个包是在玩家进入可见范围时由服务器发送的，而不是在玩家加入的时候。
//这个包必须在 PlayerInfoUpdate（player_info_update.rs）包之后发送，该包添加了玩家数据，供客户端在生成玩家时使用。
//如果这个数据包生成的玩家信息在数据包到达时不存在，Notchian客户端将不会生成该玩家实体。玩家信息包包括皮肤/披风数据。
pub struct SpawnPlayer {
    entity_id: i32,
    player_uuid: String,
    x: f64,
    y: f64,
    z: f64,
    yaw: Angle,
    pitch: Angle,
}

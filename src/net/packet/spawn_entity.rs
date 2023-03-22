// Spawn Entity
// Sent by the server when a vehicle or other non-living entity is created.
// PacketID = 0x00
// Field Name	Field Type	Notes
// EntityID	    VarInt	A unique integer ID mostly used in the protocol to identify the entity.
// EntityUUID	UUID	A unique identifier that is mostly used in persistence and places where the uniqueness matters more.
// Type	        VarInt	The type of the entity (see "type" field of the list of Mob types).
// X	        Double
// Y	        Double
// Z	        Double
// Pitch	    Angle	To get the real pitch, you must divide this by (256.0F / 360.0F)
// Yaw	        Angle	To get the real yaw, you must divide this by (256.0F / 360.0F)
// Head Yaw	    Angle	Only used by living entities, where the head of the entity may differ from the general body rotation.
// Data	        VarInt	Meaning dependent on the value of the Type field, see Object Data for details.
// Velocity X	Short	Same units as Set Entity Velocity.
// Velocity Y	Short
// Velocity Z	Short

use crate::net::datatype::datatype::Angle;

pub struct SpawnEntity {
    entity_id: i32,
    entity_uuid: String,
    entity_type: i32,

    x: f64,
    y: f64,
    z: f64,

    pitch: Angle,
    yaw: Angle,
    head_yaw: Angle,

    data: i32,

    velocity_X: i16,
    velocity_y: i16,
    velocity_z: i16,
}

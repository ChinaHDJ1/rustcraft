pub enum Packet {
    //Clientbound
    SpawnEntity,
    SpawnExperienceOrb,
    SpawnPlayer,
    EntityAnimation,
    AwardStatistics,
    //Serverbound
}

pub fn match_packet() -> Packet {}

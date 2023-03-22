pub struct EntityAnimation {
    entity_id: i32,
    animation: i32,
}

pub enum Animation {
    SwingMainArm,   //摆动手臂
    TakeDamage,     //受伤
    LeaveBed,       //起床
    SwingOffhand,   //摇摆不定
    CriticalEffect, // unkown
    MagicCriticalEffect,
}

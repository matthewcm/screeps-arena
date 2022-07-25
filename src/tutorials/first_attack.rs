use screeps_arena::game::utils::get_objects_by_prototype;
use screeps_arena::{Creep, prototypes, ReturnCode};

#[allow(dead_code)]
pub fn run() {
    let my_creep: Creep = get_objects_by_prototype(prototypes::CREEP)
        .iter()
        .find(|creep| { creep.my()})
        .unwrap()
        .to_owned();

    let enemy_creep: Creep = get_objects_by_prototype(prototypes::CREEP)
        .iter()
        .find(|creep| { ! creep.my()})
        .unwrap()
        .to_owned();

    if my_creep.attack(&enemy_creep) == ReturnCode::NotInRange {
        my_creep.move_to(&enemy_creep, None);
    }
}

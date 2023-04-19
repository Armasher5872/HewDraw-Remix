use super::*;
use smash2::app::{FighterManager, BraveUpdateMenuEvent, BraveUpdateMenu2Event, BraveUpdateMenu3Event, BraveUpdateMenu4Event};

extern "C" {
    #[link_name = "_ZN3lib9SingletonIN3app21FighterParamAccessor2EE9instance_E"]
    static PARAM_INSTANCE: u64;
}

extern "C" {
    #[link_name = "_ZN3app24FighterSpecializer_Brave30get_special_lw_command_sp_costERKNS_26BattleObjectModuleAccessorENS_28FighterBraveSpecialLwCommandEb"]
    fn get_special_lw_command_sp_cost(boma: *mut BattleObjectModuleAccessor, command: i32, pass_false: bool) -> f32;
}
  
unsafe fn set_command_for_slot(fighter: &mut BattleObject, slot: usize, id: i32) {
    let hero_mana = fighter.get_float(0x53);
    let mana = if id == 10 {
        hero_mana.max(1.0)
    }
    else {
        get_special_lw_command_sp_cost(fighter.module_accessor, id, false)
    };

    FighterManager::instance().unwrap().send_event(BraveUpdateMenuEvent::new(
        fighter.get_int(0x1000000) as u32, // ENTRY_ID
        (slot + 1) as u32,
        (id + 1) as i32,
        mana
    ));
    FighterManager::instance().unwrap().send_event(BraveUpdateMenu2Event::new(
        fighter.get_int(0x1000000) as u32,
        (slot + 1) as u32,
        hero_mana >= mana
    ));

    if hero_mana >= mana {
        fighter.set_int(id, 0x10000d4 + slot as i32);
    }
    else {
        fighter.set_int(-1, 0x10000d4 + slot as i32);
    }

    VarModule::set_int(fighter, vars::brave::instance::SPELL_SLOT_1 + slot as i32, id);
    *(*(fighter as *mut _ as *const u64).add(0xd8 / 8) as *mut u8).add(slot) = id as u8;
}

pub unsafe fn roll_spells(fighter: &mut BattleObject, vals: &mut Vec<i32>) {
    loop {
        // get history of last two rolls
        let mut used_vals = vec![];
        used_vals.push(VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_USED_1_1));
        used_vals.push(VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_USED_1_2));
        used_vals.push(VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_USED_1_3));
        used_vals.push(VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_USED_1_4));
        used_vals.push(VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_USED_2_1));
        used_vals.push(VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_USED_2_2));
        used_vals.push(VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_USED_2_3));
        used_vals.push(VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_USED_2_4));
        let val = smash::app::sv_math::rand(smash::hash40("fighter"), 0x15);

        if val == 0xF || vals.contains(&val) || used_vals.contains(&val) {  // remove Zoom (0xF)
            continue;
        }
        vals.push(val);
        if vals.len() >= 4 {
            break;
        }
    }

    // store new roll history
    VarModule::set_int(fighter, vars::brave::instance::SPELL_SLOT_USED_2_1, vars::brave::instance::SPELL_SLOT_USED_1_1);
    VarModule::set_int(fighter, vars::brave::instance::SPELL_SLOT_USED_2_2, vars::brave::instance::SPELL_SLOT_USED_1_2);
    VarModule::set_int(fighter, vars::brave::instance::SPELL_SLOT_USED_2_3, vars::brave::instance::SPELL_SLOT_USED_1_3);
    VarModule::set_int(fighter, vars::brave::instance::SPELL_SLOT_USED_2_4, vars::brave::instance::SPELL_SLOT_USED_1_4);
    VarModule::set_int(fighter, vars::brave::instance::SPELL_SLOT_USED_1_1, vals[0]);
    VarModule::set_int(fighter, vars::brave::instance::SPELL_SLOT_USED_1_2, vals[1]);
    VarModule::set_int(fighter, vars::brave::instance::SPELL_SLOT_USED_1_3, vals[2]);
    VarModule::set_int(fighter, vars::brave::instance::SPELL_SLOT_USED_1_4, vals[3]);
}

#[no_mangle]
pub unsafe extern "C" fn hero_rng_hook_impl(fighter: &mut BattleObject) {
    // println!("b8: {:#x}", *(fighter as *mut _ as *const u64).add(0xb8 / 8));
    // println!("c8: {:#x}", *(fighter as *mut _ as *const u32).add(0xc8 / 4));
    // println!("d0: {:#x}", *(fighter as *mut _ as *const u64).add(0xd0 / 8));
    // println!("c0: {:#x}", *(fighter as *mut _ as *const u64).add(0xc0 / 8));
    // println!("d8: {:#x}", *(fighter as *mut _ as *const u64).add(0xd8 / 8));
    // 0x15 = *FIGHTER_BRAVE_SPECIAL_LW_COMMAND_NUM
    // *(*(fighter as *mut _ as *const u64).add(0xd8 / 8) as *mut u32) = 0x0E0E0E0E;

    *(fighter as *mut _ as *mut u64).add(0xc0 / 8) = 4; // controls how many commands are active
    *(fighter as *mut _ as *mut u64).add(0xd0 / 8) = 4;
    fighter.set_int(3, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_WINDOW_STATE);
    fighter.set_int(0, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_WINDOW_OVERWRITE_FRAME);
    fighter.off_flag(*FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_ENABLE_COMMAND_WINDOW_OVERWRITE);
    let mut index = VarModule::get_int(fighter, vars::brave::instance::CURSOR_SLOT);

    if !VarModule::is_flag(fighter, vars::brave::instance::PERSIST_RNG) {
        VarModule::on_flag(fighter, vars::brave::instance::PERSIST_RNG);
        index = 0;
        let we_ball = smash::app::sv_math::rand(smash::hash40("fighter"), 100);
        if we_ball == 1 {
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_level_up"), Hash40::new("top"), &Vector3f::new(0.0, 10.0, 0.0), &Vector3f::new(0.0, 0.0, 0.0), 0.8, false, 0, 0, 0, 0, 0, false, false);
            let mut rand: i32;
            loop {
                rand = smash::app::sv_math::rand(smash::hash40("fighter"), 0x15);
                if rand == 0xF {
                    continue;
                }
                else {
                    break;
                }
            }
            
            set_command_for_slot(fighter, 0, rand);
            set_command_for_slot(fighter, 1, rand);
            set_command_for_slot(fighter, 2, rand);
            set_command_for_slot(fighter, 3, rand);
        }
        else {
            let mut vals = vec![];
            roll_spells(fighter, &mut vals);

            set_command_for_slot(fighter, 0, vals[0]);
            set_command_for_slot(fighter, 1, vals[1]);
            set_command_for_slot(fighter, 2, vals[2]);
            set_command_for_slot(fighter, 3, vals[3]);
        }
    }
    else {
        let slot_1 = VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_1);
        let slot_2 = VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_2);
        let slot_3 = VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_3);
        let slot_4 = VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_4);
        set_command_for_slot(fighter, 0, slot_1);
        set_command_for_slot(fighter, 1, slot_2);
        set_command_for_slot(fighter, 2, slot_3);
        set_command_for_slot(fighter, 3, slot_4);
    }

    fighter.set_int(index, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
    FighterManager::instance().unwrap().send_event(BraveUpdateMenu3Event::new(
        fighter.get_int(0x1000000) as u32
    ));
    FighterManager::instance().unwrap().send_event(BraveUpdateMenu4Event::new(
        fighter.get_int(0x1000000) as u32,
        (index as u32) + 1
    ));
    
}
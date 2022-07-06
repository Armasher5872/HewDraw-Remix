use super::*;

#[status_script(agent = "jack_doyle", status = WEAPON_JACK_DOYLE_STATUS_KIND_ENTRY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn entry_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id =
        LinkModule::get_parent_id(weapon.module_accessor, *LINK_NO_CONSTRAINT, true) as u32;
    let owner = utils::util::get_battle_object_from_id(owner_id);
    let rebel_gauge =
        VarModule::get_float(owner, vars::jack::instance::REBEL_GAUGE_ON_SUMMON_DISPATCH);
    WorkModule::mul_float(weapon.module_accessor, rebel_gauge / 100.0, 0x6);
    original!(weapon)
}

#[status_script(agent = "jack_doyle", status = WEAPON_JACK_DOYLE_STATUS_KIND_LEAVE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn leave_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id =
        LinkModule::get_parent_id(weapon.module_accessor, *LINK_NO_CONSTRAINT, true) as u32;
    let owner = utils::util::get_battle_object_from_id(owner_id);
    let arsene_meter = weapon.get_float(0x6);
    VarModule::set_float(
        owner,
        vars::jack::instance::REBEL_GAUGE_ON_SUMMON_DISPATCH,
        arsene_meter * 100.0 / (*owner).get_param_int("param_private", "doyle_frame") as f32,
    );
    original!(weapon)
}

pub fn install() {
    smashline::install_status_scripts!(entry_main, leave_main);
}

// status imports
use super::*;
use globals::*;
// This file contains code for DACUS, DACDS, and gatling attacks

pub fn install() {
    install_status_scripts!(
        //status_pre_AttackDash,
        status_AttackDash
    );
    install_hooks!(
        sub_attack_dash_uniq,
        status_AttackDash_Main
    );
}

#[common_status_script(status = FIGHTER_STATUS_KIND_ATTACK_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE,
    symbol = "_ZN7lua2cpp16L2CFighterCommon21status_pre_AttackDashEv")]
unsafe fn status_pre_AttackDash(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ground_correct_kind = if ParamModule::get_flag(fighter.module_accessor, ParamType::Shared, "attack_dash_cliff_stop") {
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP
    } else {
        *GROUND_CORRECT_KIND_KEEP
    } as u32;
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        ground_correct_kind,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_DASH as u32,
        0
    );
    L2CValue::I32(0)
}

#[common_status_script(status = FIGHTER_STATUS_KIND_ATTACK_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon17status_AttackDashEv")]
unsafe fn status_AttackDash(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_dash"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
    VarModule::off_flag(fighter.battle_object, vars::common::ATTACK_DASH_CANCEL_DISABLE);
    let cancel_frame = ParamModule::get_int(fighter.battle_object, ParamType::Shared, "attack_dash_cancel_frame");
    VarModule::set_int(fighter.battle_object, vars::common::ATTACK_DASH_CANCEL_FRAME, cancel_frame);
    let catch_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("catch_dash_frame"));
    //VarModule::off_flag(fighter.battle_object, vars::common::ATTACK_DASH_SLIDEOFF);
    WorkModule::set_int(fighter.module_accessor, catch_dash_frame, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_CATCH_FRAME);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
        let sha_enable = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("jump_mini_attack_enable_frame"));
        WorkModule::set_int(fighter.module_accessor, sha_enable + 1, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
    let log_attack_kind = fighter.status_attack()["log_infos"]["attack_dash"].get_i64();
    WorkModule::set_int64(fighter.module_accessor, log_attack_kind, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let sha_enable = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    if sha_enable == 0
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
        if log_attack_kind != 0 {
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log_attack_kind as u64);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_attack_dash_uniq(L2CValue::Bool(false));
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
    VarModule::off_flag(fighter.battle_object, vars::common::IS_DACUS);
    fighter.global_table[SUB_STATUS] = L2CValue::Ptr(sub_attack_dash_uniq as *const () as _);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_AttackDash_Main as *const () as _))
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon20sub_attack_dash_uniqEN3lib8L2CValueE")]
unsafe extern "C" fn sub_attack_dash_uniq(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    if arg.get_bool() {
        let catch_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_CATCH_FRAME);
        if 0 <= catch_frame {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_CATCH_FRAME);
            if catch_frame == 0 {
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK)
            || WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME, 0) != 0 {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        }
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_FRAME);
        if !VarModule::is_flag(fighter.battle_object, vars::common::ATTACK_DASH_CANCEL_DISABLE)
            && VarModule::countdown_int(fighter.battle_object, common::ATTACK_DASH_CANCEL_FRAME, 0) {
            VarModule::on_flag(fighter.battle_object, vars::common::ATTACK_DASH_CANCEL_DISABLE);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
        }
    } else {
        let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_FRAME);
        let item_catch_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("item_catch_frame_attack_dash"));
        if frame <= item_catch_frame {
            fighter.sub_GetLightItemImm(false.into());
        }
    }
    L2CValue::I32(0)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon22status_AttackDash_MainEv")]
unsafe extern "C" fn status_AttackDash_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) && fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() { return L2CValue::I32(0); }
    /*if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR && !VarModule::is_flag(fighter.battle_object, vars::common::ATTACK_DASH_SLIDEOFF) {
        if ParamModule::get_flag(fighter.module_accessor, ParamType::Shared, "attack_dash_cliff_stop") {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
                L2CValue::Bool(false)
            );
            return L2CValue::I32(0);
        } else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            VarModule::on_flag(fighter.battle_object, vars::common::ATTACK_DASH_SLIDEOFF);
        }
    }*/
    let sha_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    if sha_frame > 0 && !StopModule::is_stop(fighter.module_accessor) {
        if fighter.sub_check_button_jump().get_bool() {
            let script_name = fighter.status_attack()[0x10f40d7b92u64]["attack_dash"].get_hash();
            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, script_name, -1);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            fighter.change_status_jump_mini_attack(L2CValue::Bool(true));
            return L2CValue::I32(1);
        }
    }
    let log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if sha_frame == 1 && !fighter.global_table[8].get_bool() && log_attack_kind > 0 {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log_attack_kind);
        WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if sub_attack_dash_is_attackhi4_cancel(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::common::IS_DACUS);
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_ATTACK_HI4_START),
            L2CValue::Bool(true)
        );
        return L2CValue::I32(1);
    }
    if sub_attack_dash_is_attacklw4_cancel(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::common::IS_DACUS);
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_ATTACK_LW4_START),
            L2CValue::Bool(true)
        );
        return L2CValue::I32(1);
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) {
        let stick_x = fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor);
        if stick_x <= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x"))
            && catch_check(fighter.module_accessor, &fighter.global_table[SITUATION_KIND]) {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_CATCH_TURN),
                L2CValue::Bool(true)
            );
            return L2CValue::I32(0);
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH)
        && catch_check(fighter.module_accessor, &fighter.global_table[SITUATION_KIND]) {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_CATCH_DASH),
            L2CValue::Bool(true)
        );
        return L2CValue::I32(0);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::get_param_int(fighter.module_accessor, 0x17e10662a4, 0) == *FIGHTER_ATTACK_DASH_TYPE_SQUAT_WAIT {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_SQUAT_WAIT),
                L2CValue::Bool(false)
            );
        } else {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_WAIT),
                L2CValue::Bool(false)
            );
        }
    }
    return L2CValue::I32(0);

    unsafe fn sub_attack_dash_is_attackhi4_cancel(fighter: &mut L2CFighterCommon) -> bool {
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
        let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_FRAME);
        let is_catch = sub_attack_dash_is_cancel_catch(fighter);
        let is_tilt_input = frame > 1 && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0 && pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;
        let is_attackhi4 = stick_y >= 0.7 && is_catch || is_tilt_input || cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0;
        !VarModule::is_flag(fighter.battle_object, vars::common::ATTACK_DASH_CANCEL_DISABLE) && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START) && is_attackhi4 && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)
    }

    unsafe fn sub_attack_dash_is_attacklw4_cancel(fighter: &mut L2CFighterCommon) -> bool {
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
        let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_FRAME);
        let is_catch = sub_attack_dash_is_cancel_catch(fighter);
        let is_tilt_input = frame > 1 && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 && pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;
        let is_attacklw4 = stick_y <= -0.7 && is_catch || is_tilt_input || cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 != 0;
        !VarModule::is_flag(fighter.battle_object, vars::common::ATTACK_DASH_CANCEL_DISABLE) && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START) && is_attacklw4 && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)
    }
    
    unsafe fn sub_attack_dash_is_cancel_catch(fighter: &mut L2CFighterCommon) -> bool {
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH)
    }

    unsafe fn catch_check(boma: *mut app::BattleObjectModuleAccessor, situation_kind: &L2CValue) -> bool {
        ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) && *situation_kind == SITUATION_KIND_GROUND && !ItemModule::is_have_item(boma, 0)
    }
}
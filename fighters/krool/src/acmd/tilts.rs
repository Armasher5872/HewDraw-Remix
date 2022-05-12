
use super::*;


#[acmd_script( agent = "krool", script = "game_attacks3hi" , category = ACMD_GAME , low_priority)]
unsafe fn krool_attack_s3_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        //FT_MOTION_RATE(fighter, 0.818);
        VarModule::off_flag(boma.object(), vars::common::IS_HEAVY_ATTACK);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            VarModule::on_flag(boma.object(), vars::common::IS_HEAVY_ATTACK);
        }
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
            FT_MOTION_RATE(fighter, 20.0/(9.5-8.0));
        }
        else{
            FT_MOTION_RATE(fighter, 1.0/(10.0-8.0));
        }
    }
    frame(lua_state, 9.5);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            FT_MOTION_RATE(fighter, 2.0/(11.0-9.5));
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            ATTACK(fighter, 0, 0, Hash40::new("waist"), 13.0, 361, 75, 0, 63, 7.0, 0.0, 0.0, 0.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.0, 361, 75, 0, 63, 4.0, 0.0, 1.0, 2.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("havel"), 13.0, 42, 75, 0, 60, 7.0, 0.0, 0.0, 0.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("waist"), 11.0, 361, 75, 0, 63, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 11.0, 361, 75, 0, 63, 4.0, 0.0, 1.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("havel"), 13.0, 42, 75, 0, 60, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            FT_MOTION_RATE(fighter, 1.2);
        }
    }
    
}

#[acmd_script( agent = "krool", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn krool_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        //FT_MOTION_RATE(fighter, 0.818);
        VarModule::off_flag(boma.object(), vars::common::IS_HEAVY_ATTACK);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            VarModule::on_flag(boma.object(), vars::common::IS_HEAVY_ATTACK);
        }
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
            FT_MOTION_RATE(fighter, 20.0/(9.5-8.0));
        }
        else{
            FT_MOTION_RATE(fighter, 1.0/(10.0-8.0));
        }
    }
    frame(lua_state, 9.5);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            FT_MOTION_RATE(fighter, 2.0/(11.0-9.5));
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            ATTACK(fighter, 0, 0, Hash40::new("waist"), 13.0, 361, 75, 0, 63, 7.0, 0.0, 0.0, 0.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.0, 361, 75, 0, 63, 4.0, 0.0, 1.0, 2.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("havel"), 13.0, 42, 75, 0, 60, 7.0, 0.0, 0.0, 0.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("waist"), 11.0, 361, 75, 0, 63, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 11.0, 361, 75, 0, 63, 4.0, 0.0, 1.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("havel"), 13.0, 42, 75, 0, 60, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            FT_MOTION_RATE(fighter, 1.2);
        }
    }
    
}

#[acmd_script( agent = "krool", script = "effect_attacks3hi" , category = ACMD_EFFECT , low_priority)]
unsafe fn krool_attack_s3_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 16, 16, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_level_up"), Hash40::new("waist"), 0, 0.0, 0.0, 0, 0, 0, 0.65, true);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("krool_attack_s3_arc"), Hash40::new("krool_attack_s3_arc"), Hash40::new("top"), -4, 16.5, 5, -10, -55, 16, 1.4, true, *EF_FLIP_YZ, 0.4);
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("krool_attack_s3_arc"), Hash40::new("krool_attack_s3_arc"), Hash40::new("top"), 4, 18, 5, -351, -55, 185, 1.4, true, *EF_FLIP_YZ, 0.4);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("krool_attack_s3_impact"), Hash40::new("top"), 0, 20, 20, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 360, true, 1);
    }
}

#[acmd_script( agent = "krool", script = "effect_attacks3" , category = ACMD_EFFECT , low_priority)]
unsafe fn krool_attack_s3_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 16, 16, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_level_up"), Hash40::new("waist"), 0, 0.0, 0.0, 0, 0, 0, 0.65, true);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("krool_attack_s3_arc"), Hash40::new("krool_attack_s3_arc"), Hash40::new("top"), -4, 13, 7, 24, -55, -7, 1.4, true, *EF_FLIP_YZ, 0.4);
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("krool_attack_s3_arc"), Hash40::new("krool_attack_s3_arc"), Hash40::new("top"), 4, 16, 7, -4, -55, -139, 1.4, true, *EF_FLIP_YZ, 0.4);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("krool_attack_s3_impact"), Hash40::new("top"), 0, 12, 23, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 360, true, 1);
    }
}

#[acmd_script( agent = "krool", script = "game_attacks3lw" , category = ACMD_GAME , low_priority)]
unsafe fn krool_attack_s3_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        //FT_MOTION_RATE(fighter, 0.818);
        VarModule::off_flag(boma.object(), vars::common::IS_HEAVY_ATTACK);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            VarModule::on_flag(boma.object(), vars::common::IS_HEAVY_ATTACK);
        }
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            FT_MOTION_RATE(fighter, 20.0/(9.5-8.0));
            WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
        }
        else{
            FT_MOTION_RATE(fighter, 1.0/(10.0-8.0));
        }
    }
    frame(lua_state, 9.5);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            FT_MOTION_RATE(fighter, 2.0/(11.0-9.5));
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            ATTACK(fighter, 0, 0, Hash40::new("waist"), 13.0, 361, 75, 0, 63, 7.0, 0.0, 0.0, 0.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.0, 361, 75, 0, 63, 4.0, 0.0, 1.0, 2.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("havel"), 13.0, 42, 75, 0, 60, 7.0, 0.0, 0.0, 0.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("waist"), 11.0, 361, 75, 0, 63, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 11.0, 361, 75, 0, 63, 4.0, 0.0, 1.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("havel"), 13.0, 42, 75, 0, 60, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            FT_MOTION_RATE(fighter, 1.2);
        }
    }
    
}

#[acmd_script( agent = "krool", script = "effect_attacks3lw" , category = ACMD_EFFECT , low_priority)]
unsafe fn krool_attack_s3_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 16, 16, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_level_up"), Hash40::new("waist"), 0, 0.0, 0.0, 0, 0, 0, 0.65, true);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("krool_attack_s3_arc"), Hash40::new("krool_attack_s3_arc"), Hash40::new("top"), -4, 11, 5, 30, -49, -19, 1.4, true, *EF_FLIP_YZ, 0.4);
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("krool_attack_s3_arc"), Hash40::new("krool_attack_s3_arc"), Hash40::new("top"), 4, 10, 5, -15, -48, -124, 1.4, true, *EF_FLIP_YZ, 0.4);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("krool_attack_s3_impact"), Hash40::new("top"), 0, 6, 20, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 360, true, 1);
    }
}

#[acmd_script( agent = "krool", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn krool_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("arml"), 12.0, 84, 45, 0, 76, 4.0, 2.2, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 12.0, 84, 45, 0, 76, 4.0, -1.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 12.0, 84, 45, 0, 76, 5.0, 7.5, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 3.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 3.0, false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("arml"), 7.0, 84, 45, 0, 60, 4.0, 2.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 7.0, 84, 45, 0, 60, 4.0, -1.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 9.0, 84, 45, 0, 60, 5.0, 7.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 3.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 3.0, false);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    
}
#[acmd_script( agent = "krool", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn krool_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(boma.object(), vars::common::IS_HEAVY_ATTACK);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            VarModule::on_flag(boma.object(), vars::common::IS_HEAVY_ATTACK);
        }
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            FT_MOTION_RATE(fighter, 20.0/(11.0-9.0));
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 10.0, 85, 50, 0, 100, 4.5, 2.5, -1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 50, 0, 30, 6.0, 0.0, 3.5, 16.5, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 85, 50, 0, 100, 5.0, 0.0, 3.5, 16.5, Some(0.0), Some(3.5), Some(18.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 60, 50, 0, 80, 7.0, 0.0, 6.0, 13.0, Some(0.0), Some(6.0), Some(24.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            //SlowModule::set_whole(boma, 2, 30);
        }
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            //SlowModule::set_whole(boma, 2, 30);
        }
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            FT_MOTION_RATE(fighter, 1.2);
        }
    }
}

#[acmd_script( agent = "krool", script = "effect_attacklw3" , category = ACMD_EFFECT , low_priority)]
unsafe fn krool_attack_lw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 16, 16, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_level_up"), Hash40::new("waist"), 0, 0.0, 0.0, 0, 0, 0, 0.65, true);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), -3, 24, 12, 70, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.6);
        EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -3, 24, 12, 70, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.6);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("krool_smash_lw_ground"), Hash40::new("krool_smash_lw_ground"), Hash40::new("top"), 0, 0, 20, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 20, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        krool_attack_s3_hi_game,
        krool_attack_s3_hi_effect,
        krool_attack_s3_s_game,
        krool_attack_s3_s_effect,
        krool_attack_s3_lw_game,
        krool_attack_s3_lw_effect,
        krool_attack_hi3_game,
        krool_attack_lw3_game,
        krool_attack_lw3_effect,
    );
}


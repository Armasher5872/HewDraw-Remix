use super::*;
use globals::*;

// This prevents projectiles from dying on platforms/ground
#[skyline::hook(replace=GroundModule::is_touch)]
unsafe fn is_touch_hook(boma: &mut BattleObjectModuleAccessor, ground_touch_flags: u32) -> bool {
    let mut ground_touch_flags = ground_touch_flags;
    let normal_y = GroundModule::get_touch_normal_y(boma, *GROUND_TOUCH_FLAG_DOWN as u32);
    let prev_pos = *PostureModule::prev_pos(boma);
    let pos = *PostureModule::pos(boma);

    if boma.is_weapon()
    && (normal_y >= 0.99 && normal_y <= 1.01)  // if touching a near-flat platform/ground
    && prev_pos.y <= pos.y  // if the projectile wasn't moving downwards
    && [*WEAPON_KIND_SAMUS_CSHOT,
        *WEAPON_KIND_RYU_HADOKEN,
        *WEAPON_KIND_LUCAS_PK_FIRE,
        *WEAPON_KIND_EDGE_FLARE1,
        *WEAPON_KIND_WOLF_BLASTER_BULLET,
        *WEAPON_KIND_MASTER_ARROW1,
        *WEAPON_KIND_MASTER_ARROW2,
        *WEAPON_KIND_LUIGI_FIREBALL,
        *WEAPON_KIND_FOX_BLASTER_BULLET,
        *WEAPON_KIND_SAMUSD_CSHOT,
        *WEAPON_KIND_BRAVE_FIREBALL,
        *WEAPON_KIND_FALCO_BLASTER_BULLET,
        *WEAPON_KIND_KEN_HADOKEN,
        *WEAPON_KIND_REFLET_THUNDER,
        *WEAPON_KIND_SZEROSUIT_PARALYZER_BULLET,
        *WEAPON_KIND_MIIGUNNER_ATTACKAIRF_BULLET,
        *WEAPON_KIND_WIIFIT_SUNBULLET,
        *WEAPON_KIND_SAMUS_SUPERMISSILE,
        *WEAPON_KIND_SAMUSD_SUPERMISSILE,
        *WEAPON_KIND_MEWTWO_SHADOWBALL,
        *WEAPON_KIND_LUCARIO_AURABALL,
        *WEAPON_KIND_MIIGUNNER_SUPERMISSILE,
        *WEAPON_KIND_MIIGUNNER_GUNNERCHARGE,
        *WEAPON_KIND_GEKKOUGA_SHURIKEN,
        *WEAPON_KIND_PICHU_DENGEKIDAMA,
        *WEAPON_KIND_BRAVE_DEATHBALL,
        *WEAPON_KIND_KAMUI_RYUSENSYA,
        *WEAPON_KIND_EDGE_FIRE
    ].contains(&boma.kind())
    {
        if ground_touch_flags == *GROUND_TOUCH_FLAG_ALL as u32
        && !original!()(boma, *GROUND_TOUCH_FLAG_LEFT as u32 | *GROUND_TOUCH_FLAG_RIGHT as u32)  // if not touching a wall
        {
            // Ignore ground collision
            return false;
        }
        if ground_touch_flags & *GROUND_TOUCH_FLAG_DOWN as u32 != 0 {
            // Ignore ground collision
            ground_touch_flags -= *GROUND_TOUCH_FLAG_DOWN as u32;
        }
    }
    original!()(boma, ground_touch_flags)
}

#[skyline::hook(replace=GroundModule::is_floor_touch_line)]
unsafe fn is_floor_touch_line_hook(boma: &mut BattleObjectModuleAccessor, ground_touch_flags: u32) -> bool {
    let mut ground_touch_flags = ground_touch_flags;
    let normal_y = GroundModule::get_touch_normal_y(boma, *GROUND_TOUCH_FLAG_DOWN as u32);
    let prev_pos = *PostureModule::prev_pos(boma);
    let pos = *PostureModule::pos(boma);

    if boma.is_weapon()
    && (normal_y >= 0.99 && normal_y <= 1.01)  // if touching a near-flat platform/ground
    && prev_pos.y <= pos.y  // if the projectile wasn't moving downwards
    && ( ([*WEAPON_KIND_KROOL_IRONBALL, *WEAPON_KIND_KROOL_SPITBALL].contains(&boma.kind())
            && boma.is_status(*WEAPON_KROOL_IRONBALL_STATUS_KIND_SHOOT))
        || (boma.kind() == *WEAPON_KIND_KOOPAJR_CANNONBALL
            && boma.is_status(*WEAPON_KOOPAJR_CANNONBALL_STATUS_KIND_SHOOT)
            && !KineticModule::is_enable_energy(boma, *WEAPON_KOOPAJR_CANNONBALL_KINETIC_ENERGY_ID_GRAVITY)) )  // if Jr.'s cannonball is flying straight horizontally
    {
        if ground_touch_flags == *GROUND_TOUCH_FLAG_ALL as u32
        && !original!()(boma, *GROUND_TOUCH_FLAG_LEFT as u32 | *GROUND_TOUCH_FLAG_RIGHT as u32)  // if not touching a wall
        {
            // Ignore ground collision
            return false;
        }
        if ground_touch_flags & *GROUND_TOUCH_FLAG_DOWN as u32 != 0 {
            // Ignore ground collision
            ground_touch_flags -= *GROUND_TOUCH_FLAG_DOWN as u32;
        }
    }
    original!()(boma, ground_touch_flags)
}

#[skyline::hook(replace=GroundModule::get_touch_flag)]
unsafe fn get_touch_flag_hook(boma: &mut BattleObjectModuleAccessor) -> i32 {
    let normal_y = GroundModule::get_touch_normal_y(boma, *GROUND_TOUCH_FLAG_DOWN as u32);

    if boma.is_weapon()
    && (normal_y >= 0.99 && normal_y <= 1.01)  // if touching a near-flat platform/ground
    && boma.kind() == *WEAPON_KIND_DEDEDE_GORDO
    && boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_THROW)  // when Dedede first tosses Gordo
    && boma.status_frame() == 0  // on frame 1 of the toss
    {
        if original!()(boma) == *GROUND_TOUCH_FLAG_DOWN {
            // Ignore ground collision
            return *GROUND_TOUCH_FLAG_NONE;
        }
    }
    original!()(boma)
}


// This function is used to calculate the following:
//      param_2: Left ECB point's horizontal offset from Top bone (negative number)
//      param_3: Bottom ECB point's vertical offset from Top bone (positive number, 0.0 in vanilla)
//      param_4: Right ECB point's horizontal offset from Top bone (positive number)
//      param_5: Top ECB point's vertical offset from Top bone (positive number)

// All of your character's map_coll_data bones, found in vl.prc, are stored in param_1's Hash40 pointer

// Not sure what param_6 is, but when 0, it skips calculations for your ECB's bottom point and just sets it to 0.0, which "locks" your ECB's bottom point to your Top bone
// when 1, it calculates your bottom ECB point normally, like the other 3 points
// Vanilla passes 0 by default, so we have to forcibly pass in 1
#[skyline::hook(offset = 0x45f420)]
unsafe fn ground_module_ecb_point_calc_hook(ground_module: u64, param_1: *mut *mut Hash40, param_2: *mut f32, param_3: *mut f32, param_4: *mut f32, param_5: *mut f32, param_6: u32) {
    let boma = *((ground_module + 0x20) as *mut *mut BattleObjectModuleAccessor);
    let mut param_6 = param_6.clone();
    if (*boma).is_fighter()
    && !VarModule::is_flag((*boma).object(), vars::common::status::DISABLE_ECB_SHIFT)
    && !(*boma).is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEMO,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_THROWN])
    && (*boma).is_situation(*SITUATION_KIND_AIR)
    && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) >= ParamModule::get_int((*boma).object(), ParamType::Common, "ecb_shift_air_trans_frame") {
        // This check passes after 9 frames of airtime, if not in a grabbed/thrown state
        param_6 = 1;
    }
    // The original function calls model_module_joint_global_position_with_offset_hook
    if (*boma).is_fighter() { VarModule::on_flag((*boma).object(), vars::common::instance::IS_GETTING_POSITION_FOR_ECB); }
    call_original!(ground_module, param_1, param_2, param_3, param_4, param_5, param_6);
    if (*boma).is_fighter() { VarModule::off_flag((*boma).object(), vars::common::instance::IS_GETTING_POSITION_FOR_ECB); }
}


// This function calculates the coordinates of the passed bone relative to the Top bone (PostureModule::pos)
// It stores these x/y/z coordinates in param_3's Vector3f

// ground_module_ecb_point_calc_hook will pass each bone from your character's map_coll_data list in vl.prc, one by one, into this func
// If param_6 in ground_module_ecb_point_calc_hook is 1, it will then pass the Trans bone once all of the map_coll_data bones have been processed
// The game will use your Trans bone's distance from your Top bone to determine where to place your bottom ECB point, which will pretty much always be {0, 0, 0}
// This is why your ECB bottom point is always "locked" to your Top bone

// By returning once the Trans bone is passed into this func, we can ignore it and thus use your map_coll_data bones to calculate your bottom ECB point, like the other 3 points
#[skyline::hook(offset = 0x48fc40)]
unsafe fn model_module_joint_global_position_with_offset_hook(model_module: u64, bone: Hash40, param_3: *mut Vector3f, param_4: *mut Vector3f, param_5: bool) {
    let boma = *(model_module as *mut *mut BattleObjectModuleAccessor).add(1);
    if (*boma).is_fighter()
    && VarModule::is_flag((*boma).object(), vars::common::instance::IS_GETTING_POSITION_FOR_ECB)
    && bone == Hash40::new("trans")
    {
        return;
    }
    call_original!(model_module, bone, param_3, param_4, param_5);
}

// Unused for now
#[skyline::hook(offset = 0x523a60)]
unsafe fn groundcollision__processgroundcollisioninfo(groundcollisioninfo: *mut f32, groundcollision: *mut u64) {
    call_original!(groundcollisioninfo, groundcollision)
}

// Performs ground correct
#[skyline::hook(offset = 0x53fe30)]
unsafe fn groundcollision__processgroundcollisioninfo_check_landing(groundcollisioninfo: *mut f32, groundcollision: *mut u64) {
    let flags = *(groundcollisioninfo.add(0x5d8 / 4) as *mut u32);
    let is_fighter = flags >> 27 & 1 == 0;

    call_original!(groundcollisioninfo, groundcollision);

    let prev_touch_pos_y = *groundcollisioninfo.add(0x1A4 / 4);
    let touch_pos_y = *groundcollisioninfo.add(0xB4 / 4);
    let ecb_offset_y = *groundcollisioninfo.add(0x3d4 / 4);
    //println!("situation kind {}", *groundcollisioninfo.add(0x5a0 / 4));

    if is_fighter
    && prev_touch_pos_y == 0.0
    && touch_pos_y != 0.0
    && ecb_offset_y != 0.0
    {
        // When landing, sets your position to the coordinates of the surface you are landing on
        *groundcollisioninfo.add(0x634 / 4) = touch_pos_y;
        // Reset ECB offset to 0.0
        *groundcollisioninfo.add(0x3d4 / 4) = 0.0;
    }
}
// Unused for now
// Sets GroundCollisionLine
#[skyline::hook(offset = 0x52d900)]
unsafe fn groundcollision__processgroundcollisioninfo_check_landing_sub(groundcollision: u64, arg2: *mut u64, prev_ecb_bottom_pos: *mut smash::phx::Vector2f, ecb_bottom_translation: *mut smash::phx::Vector2f, arg5: u64, arg6: u64, arg7: *mut u64) -> *mut GroundCollisionLine {
    if arg5 == 2048 && arg6 == 1048576 {
        let groundcollisionline = *((groundcollision + 0x320) as *mut u64) as *mut GroundCollisionLine;
        let groundcollisionline_prev = *((groundcollision + 0x328) as *mut u64) as *mut GroundCollisionLine;

        let groundcollisionline_next = *(groundcollisionline as *mut *mut GroundCollisionLine);
        let normal = *(((groundcollisionline_next as u64) + 0xA0) as *mut smash::phx::Vector2f);
        let vertex_1_y = *(((groundcollisionline_next as u64) + 0x24) as *mut f32);
        let vertex_2_y = *(((groundcollisionline_next as u64) + 0x34) as *mut f32);

    }
    call_original!(groundcollision, arg2, prev_ecb_bottom_pos, ecb_bottom_translation, arg5, arg6, arg7)
}

pub fn install() {
    unsafe {
        // Removes 0.3 unit leniency above ECB bottom when deciding whether to land
        // which reduces frequency of platform cancels
        skyline::patching::Patch::in_text(0x540dd8).data(0x529ae148);
        skyline::patching::Patch::in_text(0x540ddc).data(0x72a78468);
    }
    skyline::install_hooks!(
        is_touch_hook,
        is_floor_touch_line_hook,
        get_touch_flag_hook,
        groundcollision__processgroundcollisioninfo_check_landing,
        ground_module_ecb_point_calc_hook,
        model_module_joint_global_position_with_offset_hook
    );
}
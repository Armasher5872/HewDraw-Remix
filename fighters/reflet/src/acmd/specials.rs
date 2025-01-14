use super::*;

unsafe extern "C" fn game_specialntronstart(agent: &mut L2CAgentBase) {
    //let lua_state = agent.lua_state_agent;
    //let boma = agent.boma();
    //17 -> 20
}

unsafe extern "C" fn game_specialairntronstart(agent: &mut L2CAgentBase) {
    //let lua_state = agent.lua_state_agent;
    //let boma = agent.boma();
    //17 -> 20
}

unsafe extern "C" fn game_specialntronend(agent: &mut L2CAgentBase) {
    //let lua_state = agent.lua_state_agent;
    //let boma = agent.boma();
    //FAF is frame 80/84
}

unsafe extern "C" fn game_specialairntronend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE_RANGE(agent, 1.0, 65.0, 47.0);  //FAF is frame 80/84
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 8.0, 8.0);
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    }
    frame(lua_state, 12.0);
    MotionModule::set_rate(boma, 2.0);
    for _ in 0..30 {
        if agent.is_button_trigger(Buttons::Special) {
            agent.on_flag(*FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    }
    frame(lua_state, 12.0);
    MotionModule::set_rate(boma, 2.0);
    for _ in 0..30 {
        if agent.is_button_on(Buttons::Special) {
            agent.on_flag(*FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
        }
        wait(lua_state, 1.0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialntronstart", game_specialntronstart, Priority::Low);
    agent.acmd( "game_specialairntronstart",game_specialairntronstart, Priority::Low);
    agent.acmd("game_specialntronend", game_specialntronend, Priority::Low);
    agent.acmd("game_specialairntronend", game_specialairntronend, Priority::Low);
    
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialairhi, Priority::Low);
}

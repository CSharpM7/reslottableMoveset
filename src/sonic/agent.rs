use crate::imports::imports_agent::*;

unsafe fn agent_start(fighter: &mut L2CFighterCommon)
{
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind != *FIGHTER_KIND_SONIC {
        return;
    }
    if !is_Modded(fighter.module_accessor) {
        return;
    }
}

#[smashline::fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}

#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}

pub fn install() {
    smashline::install_agent_init_callbacks!(
        agent_init
    );
    install_agent_resets!(
        agent_reset
    );
}
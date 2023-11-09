use crate::imports::imports_agent::*;

unsafe fn sonic_update(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let boma = fighter.module_accessor;
    if is_Modded(boma) {
        
    }
}

#[fighter_frame( agent = FIGHTER_KIND_SONIC )]
fn dk_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        sonic_update(fighter);
    }
}
#[smashline::fighter_frame_callback]
fn global_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe{
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let category = smash::app::utility::get_category(boma);
        let kind = smash::app::utility::get_kind(boma);
        if category == BATTLE_OBJECT_CATEGORY_FIGHTER && kind == FIGHTER_KIND_SONIC  {
            sonic_update(fighter);
        }
    }
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(
      global_fighter_frame
    );
}
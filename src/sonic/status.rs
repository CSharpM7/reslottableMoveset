use crate::imports::imports_agent::*;


#[status_script(agent = "sonic", status = FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn sonic_speciallw_hold_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_Modded(fighter.module_accessor) {
        return original!(fighter);
    }
    else{
        println!("Drop Dash Exec");
        return 0.into();
    }
}
pub fn install() {
    install_status_scripts!(
        sonic_speciallw_hold_exec,
    );
}
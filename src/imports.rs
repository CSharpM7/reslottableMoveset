pub mod imports_acmd {
    pub use {
        smash::{
            lib::{
                L2CValue,
                LuaConst,
                lua_const::*
            },
            app::{
                *,
                self,
                sv_animcmd::{
                    frame,
                    wait
                },
                lua_bind::*
            },
            hash40,
            lua2cpp::*,
            phx::*
        },
        smash_script::{
            *,
            macros::*
        },
        sharpsmashlinesuite::{
            *,
            /* 
            util::{
                *,
                self
            },*/
            ext::*,
            getvar::*
        },
        smashline::*,
        crate::data::gamemode::*,
        crate::singleslot::*,
    };
}

pub mod imports_agent {
    pub use {
        smash::{
            lib::{
                L2CValue,
                L2CAgent,
                lua_const::*
            },
            app::{
                *,
                self,
                lua_bind::*,
            },
            hash40,
            lua2cpp::*,
            phx::*
        },
        smash_script::{
            *,
            macros::*
        },
        sharpsmashlinesuite::{
            *,
            /* 
            util::{
                *,
                self
            },*/
            ext::*,
            getvar::*
        },
        smashline::*,
        crate::data::gamemode::*,
        crate::singleslot::*,
    };
}
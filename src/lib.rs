#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_sonic_case,
    unused
)]
#![deny(
    deprecated
)]

use smash::{
    lib::{
        L2CValue,
        LuaConst,
        lua_const::*
    },
    //lib::,
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
};
use smash_script::{
    *,
    macros::*
};
use smashline::*;

#[macro_use]
extern crate lazy_static;

mod sonic;
mod imports;
mod installer;
pub mod data;
use data::gamemode::*;

pub use skyline::libc::c_char;

pub mod singleslot;

#[skyline::main(name = "smashline_sonicoc")]
pub fn main() {
    println!("[smashline_sonicoc::main] Loading...");
    //custom_vars::install();
    data::gamemode::set_gamemode();

    #[cfg(not(feature = "dev"))]{ 
        data::install();
        //Add hooks here
        //hook::install();
        #[cfg(feature = "devhook")]{
        println!("[smashline_sonicoc::main] Dev Hook Installed");
        return;
        }
        installer::install();
    }
    #[cfg(feature = "dev")]{
        installer::installer();
    }

    println!("[smashline_sonicoc::main] Loaded!");
}
//use arcropolis_api;
use std::sync::RwLock;
//use once_cell::sync::Lazy;
use lazy_static::lazy_static;
use std::{
    io,
    io::BufReader,
    io::BufRead,
    io::BufWriter,
    io::Write,
    env,
    collections::HashSet,
    path::Path,
    convert::TryInto,
    str::FromStr,
    fs,
    fs::File,
    iter::FromIterator,
};
use crate::imports::imports_agent::*;

lazy_static! {
    pub static ref MOD_SLOTS: RwLock<HashSet<i32>> = RwLock::new({
        let mut vec = HashSet::new();
        vec
    });
}
pub unsafe fn is_Modded(boma: *mut BattleObjectModuleAccessor) -> bool
{
    let kind = utility::get_kind(&mut *boma);
    if kind != *FIGHTER_KIND_SONIC {
        return false;
    }
    //Assume true for dev plugin
    #[cfg(feature = "dev")]
    return true;

    let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let is_modded = (*MOD_SLOTS.read().unwrap()).contains(&color);
    return is_modded;
}


pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    //std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            if entry.file_name().to_str().unwrap() == "vl.prcxml" {continue;}
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            println!("[smashline_dk::data] copied {} to {}",entry.file_name().to_str().unwrap(),dst.as_ref().to_str().unwrap());
        }
    }
    Ok(())
}

pub fn install(modFolder: &str){
    assign_slots(modFolder);
    install_params(modFolder);
}
fn assign_slots(modFolder: &str) {
    let motionFolder = format!("{}/fighter/sonic/motion/body",modFolder);
    let SOURCE_SLOT = "00";
    
    //Populate list of modded slots based on current mod folder's motion folders
    for entry in std::fs::read_dir(motionFolder.as_str()).unwrap() {
        let entry = entry.unwrap();
        let mut path = entry.path();
        if path.is_dir() {
            let slotfolder = format!("{}",path.display());

            let mut last: Vec<char> = slotfolder.chars().rev().take(4).collect();
            last.reverse();
            let lastString = String::from_iter(last);
            let laststr = lastString.as_str().replace("/", "").replace("c", "");
            let num : i32 = laststr.parse().unwrap();
            println!("Modded Slot: {num}");
            (*MOD_SLOTS.write().unwrap()).insert(num);

            //Remove on release
            /* 
            let source_num : i32 = SOURCE_SLOT.parse().unwrap();
            if num != source_num {
                let file = "motion_list.motdiff";
                let sourceFolder = format!("{}/c{}/",motionFolder.as_str(),SOURCE_SLOT);
                let sourceFile = format!("{}/c{}/{}",motionFolder.as_str(),SOURCE_SLOT,file);

                let destFile = format!("{}/{}",slotfolder,file);
                fs::copy(sourceFile.as_str(), destFile.as_str());

                if num > 7 {
                    copy_dir_all(sourceFolder, slotfolder);
                }
                else{
                    //println!("[smashline_mewtwoY::singleslot] copied motion files to {}",slotfolder);
                }
            }*/
            //
        }
    }
}
fn install_params(modFolder: &str) {
    let slot = &*MOD_SLOTS.read().unwrap();
    let mut slots: Vec<i32> = (*slot).clone().into_iter().collect();

    let paramFile = format!("{}/config_param.toml",modFolder);
    if Path::new(paramFile.as_str()).exists() {
        println!("[smashline_sonicoc::singleslot] config_param.toml already exists!");
        //return;
    }
    println!("[smashline_sonicoc::singleslot] Installing params");

    let mut param_ints: Vec<(u64,u64,i32)> = Vec::new();
    let mut param_floats: Vec<(u64,u64,f32)> = Vec::new();


    let mut param_ints: Vec<(u64,u64,i32)> = Vec::new();
    let mut param_floats: Vec<(u64,u64,f32)> = Vec::new();

    param_floats.push((hash40("run_speed_max"),0 as u64, 1.255));
    param_floats.push((hash40("air_accel_y"),0 as u64, 0.082));
    param_floats.push((hash40("jump_y"),0 as u64, 21.11));
    param_floats.push((hash40("jump_aerial_y"),0 as u64, 21.35));
    for p in param_ints {
        param_config::update_int_2(*FIGHTER_KIND_MEWTWO, slots.clone(), p);
    }
    for p in param_floats {
        param_config::update_float_2(*FIGHTER_KIND_MEWTWO, slots.clone(), p);
    }
}
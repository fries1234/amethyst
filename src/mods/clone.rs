use git2::Repository;
use std::{env, fs, path::Path, process::Command};
use crate::mods::strs::{err_unrec, inf};

pub fn clone(noconfirm: bool, pkg: &str) {
    let cachedir = format!("{}/.cache/ame", std::env::var("HOME").unwrap());
    let path = Path::new(&cachedir);
    let pkgdir = format!("{}/{}", &cachedir, &pkg);
    let pkgpath = Path::new(&pkgdir);
    let results = raur::search(&pkg).unwrap();

    if results.len() == 0 {
        err_unrec(format!("No matching AUR packages found"));
    }

    let url = format!("https://aur.archlinux.org/{}.git", results[0].name);

    if !path.is_dir() {
        let cache_result = fs::create_dir(&path);
        match cache_result {
        Ok(_) => {
            inf(format!("Created cache path (first run)"))
        }
        Err(_) => {
            err_unrec(format!("Could not create cache path"))
        }}
    }

    inf(format!("Cloning {} ...", pkg));

    if pkgpath.is_dir() {
        let rm_result = fs::remove_dir_all(&pkgpath);
        match rm_result {
        Ok(_) => {
            inf(format!("Package path for {} already found. Removing to reinstall", pkg))
        }
        Err(_) => {
            err_unrec(format!("Package path for {} already found, but could not remove to reinstall", pkg))
        }}
    }

    let dir_result = fs::create_dir(&pkgdir);
    match dir_result {
    Ok(_) => {
        inf(format!("Cloned {} to package directory", pkg))
    }
    Err(_) => {
        err_unrec(format!("Couldn't create package directory for {}", pkg))
    }}

    let cd_result = env::set_current_dir(&pkgdir);
    match cd_result {
    Ok(_) => {
        inf(format!("Entered package directory"))
    }
    Err(_) => {
        err_unrec(format!("Could not enter package directory"))
    }}

    Repository::clone(&url, &pkgpath).unwrap();

    let cd2_result = env::set_current_dir(&pkgpath);
    match cd2_result {
    Ok(_) => {
        inf(format!("Entering package directory for {}", pkg))
    }
    Err(_) => {
        err_unrec(format!("Couldn't enter package directory for {}", pkg))
    }}

    if noconfirm == true {
        inf(format!("Installing {} ...", pkg));
        let install_result = Command::new("makepkg")
                             .arg("-si")
                             .arg("--noconfirm")
                             .status();
        match install_result {
        Ok(_) => {
            inf(format!("Succesfully installed {}", pkg));
        }
        Err(_) => {
            err_unrec(format!("Couldn't install {}", pkg));
        }};
    } else {
        inf(format!("Installing {} ...", pkg));
        let install_result = Command::new("makepkg")
                             .arg("-si")
                             .status();
        match install_result {
        Ok(_) => {
            inf(format!("Succesfully installed {}", pkg));
        }
        Err(_) => {
            err_unrec(format!("Couldn't install {}", pkg));
        }};
    }
}

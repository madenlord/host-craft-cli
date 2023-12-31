use crate::server::ioutils::file;

const SAVE_PATH: &str = "conf/repo.conf";
const HOSTFILE_NAME: &str = ".host";
const HOSTFILE_PATH: &str = "mojang/.host";



pub fn download_updates() -> Result<bool, std::io::Error>{
    let mut update_found: bool = false;
    
    // Get latest changes in Git repo
    let update_res = git::fetch()?;

    // If updates in the world data was found
    if !update_res.stdout.is_empty() || !update_res.stderr.is_empty() {
        update_found = true;
        git::pull_no_ff()?;     // Merge update with local data
    }

    Ok(update_found)
}

pub fn upload_world_data() -> Result<(), std::io::Error> {
    git::add(vec!["*"])?;
    git::commit("Saving world data")?;
    git::push()?;

    Ok(())
}

pub fn update_host(user: &str) -> Result<(), std::io::Error> {
    // Current hostname is written in the hostfile and changes
    // are pushed to the Git repo
    file::write(HOSTFILE_PATH, user)?; 
    git::add(vec![HOSTFILE_NAME])?;
    git::commit(format!("New host: {}", user).as_str())?;
    git::push()?;

    Ok(())
}

pub fn get_hostfile_path() -> String {
    String::from(HOSTFILE_PATH)
}

pub fn get_current_host() -> Result<String, std::io::Error> {
    let hostfile_content = file::read(HOSTFILE_PATH)?;

    Ok(String::from(hostfile_content.trim()))
}

mod git {
    use std::process::Output;    

    use crate::server::ioutils::terminal;

    // =========== GET AND UPDATE STATE ===========
    pub fn fetch() -> Result<Output, std::io::Error>{
        execute_git_command("fetch", vec![])
    }

    pub fn pull_no_ff() -> Result<Output, std::io::Error>{
        execute_git_command("pull", vec!["--no-ff"])
    }



    // =========== PUSH ===========
    pub fn add(files: Vec<&str>) -> Result<Output, std::io::Error>{
        execute_git_command("add", files)
    }


    pub fn commit(message: &str) -> Result<Output, std::io::Error>{
        execute_git_command("commit", vec!["-m", message])
    }

    pub fn push() -> Result<Output, std::io::Error>{
        execute_git_command("push", vec![])
    }



    // =========== PRIVATE ===========
    fn execute_git_command<'a>(command: &'a str, mut args: Vec<&'a str>) -> Result<Output, std::io::Error>{ 
        let mut git_args: Vec<&str> = vec![command];
        git_args.append(&mut args);
        terminal::execute_command("git", git_args, "./mojang")
    }
}
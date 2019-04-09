use std::process::Command;

fn main() {
    let ls_test = Command::new("ls").arg("-lh").arg("/home/nmoore/").output();
    match ls_test {
        Ok(v) => println!("{}", String::from_utf8(v.stdout).unwrap()),
        Err(e) => println!("Something went wrong! {}", e)
    }

    loop {
        match check_ssh() {
            Some(o) => println!("{}", o),
            None => println!("Error calling SSH!")
        }
    }
}

fn check_ssh() -> Option<String> {
    let ssh_test = ssh_cmd("ls".to_string(), "nmoore".to_string(), "192.168.1.86".to_string());
    let result = match ssh_test {
        Err(e) => {
            println!("Something went wrong! {}", e);
            return None;
        },
        Ok(r) => r
    };

    match result.status.success() {
        true => Some(String::from_utf8(result.stdout).unwrap()),
        false => Some(String::from_utf8(result.stderr).unwrap())
    }
}

fn ssh_cmd(cmd: String, user: String, host: String) -> std::io::Result<std::process::Output> {
    Command::new("ssh")
        .arg("-o").arg("ConnectTimeout=1")
        .arg("-o").arg("UserKnownHostsFile=/dev/null")
        .arg("-l").arg(user)
        .arg(host).arg(cmd).output()
}

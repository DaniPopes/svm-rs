use std::{env, process::Command};

fn main() -> anyhow::Result<()> {
    let args = env::args().skip(1).collect::<Vec<String>>();

    let version = svm_rs::current_version()?.ok_or(svm_rs::SolcVmError::GlobalVersionNotSet)?;
    let mut version_path = svm_rs::version_path(version.to_string().as_str());
    version_path.push(format!("solc-{}", version.to_string().as_str()));

    let status = Command::new(version_path).args(args).status()?;
    let code = status.code().unwrap_or(-1);
    std::process::exit(code);
}
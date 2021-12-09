use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct CertPaths {
 pub cert: PathBuf,
 pub chain: PathBuf,
 pub fullchain: PathBuf,
 pub privkey: PathBuf,
}

pub fn get_cert_paths(
 email: &str,
 hostname: &str,
) -> Result<CertPaths, Box<dyn std::error::Error>> {
 use std::process::Command;

 let exitstatus = Command::new("apt").args(["install", "-y", "certbot"]).spawn()?.wait()?;
 if !exitstatus.success() {
  return Err(String::from("apt install certbot failed").into());
 };

 let exitstatus = Command::new("certbot")
  .args(["-n", "certonly", "--standalone", "--agree-tos", "--email", email, "-d", hostname])
  .spawn()?
  .wait()?;
 if !exitstatus.success() {
  return Err(String::from("certbot failed").into());
 };

 let ca_dir = PathBuf::from("/etc/letsencrypt/live/").join(hostname);
 let cert_paths = CertPaths {
  cert: ca_dir.join("cert.pem"),
  chain: ca_dir.join("chain.pem"),
  fullchain: ca_dir.join("fullchain.pem"),
  privkey: ca_dir.join("privkey.pem"),
 };

 Ok(cert_paths)
}

#[cfg(test)]
mod tests {
 #[test]
 fn get_cert() {
  use super::*;

  dbg!(get_cert_paths("trevyn-git@protonmail.com", "test3.turbonet.to").unwrap());
 }
}

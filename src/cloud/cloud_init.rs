pub fn generate_cloud_init_config(password: &str) -> String {
    let cloud_config = serde_json::json!({
      "runcmd": ["curl https://i.jpillora.com/chisel! | bash", "systemctl enable --now chisel"],
      "write_files": [{
        "path": "/etc/systemd/system/chisel.service",
        "content": r#"
      [Unit]
      Description=Chisel Tunnel
      Wants=network-online.target
      After=network-online.target
      StartLimitIntervalSec=0

      [Install]
      WantedBy=multi-user.target

      [Service]
      Restart=always
      RestartSec=1
      User=root
      # You can add any additional flags here
      # This example uses port 9090 for the tunnel socket. `--reverse` is required for our use case.
      ExecStart=/usr/local/bin/chisel server --port=9090 --reverse
      # Additional .env file for auth and secrets
      EnvironmentFile=-/etc/sysconfig/chisel
      "#
      }, {
        "path": "/etc/sysconfig/chisel",
        "content": format!("AUTH=chisel:{}\n", password)
      }]
    });

    "#cloud-config\n".to_string() + &cloud_config.to_string()
}

#[test]
fn test_generate_cloud_init_config() {
    let password = "test";
    let config = generate_cloud_init_config(password);
    println!("{}", config);
    assert!(config.contains("chisel:test"));
}
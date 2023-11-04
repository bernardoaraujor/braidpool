pub fn setup(
    bitcoin: String,
    rpc_port: u16,
    rpc_cookie_file: String,
) -> Result<bitcoincore_rpc::Client, bitcoincore_rpc::Error> {
    let rpc_url = format!("{}:{}", bitcoin, rpc_port);
    bitcoincore_rpc::Client::new(
        &rpc_url,
        bitcoincore_rpc::Auth::CookieFile(rpc_cookie_file.into()),
    )
}

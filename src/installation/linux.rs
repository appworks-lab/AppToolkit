pub async fn install() -> anyhow::Result<()> {
    Ok(())
}

#[cfg(target_os = "linux")]
#[cfg(test)]
mod test_install_fn_on_linux {}

fn main() {
    let windows_attributes = tauri_build::WindowsAttributes::new().app_manifest(
        r#"
    <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
      <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
          <security>
              <requestedPrivileges>
                  <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
              </requestedPrivileges>
          </security>
      </trustInfo>
    </assembly>
    "#,
    );
    let attributes = tauri_build::Attributes::new().windows_attributes(windows_attributes);
    tauri_build::try_build(attributes).unwrap();
}

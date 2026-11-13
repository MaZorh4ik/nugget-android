use idevice::{
    IdeviceService,
    mobilebackup2::MobileBackup2Client,
};
use tauri::State;
use crate::{
    device::{DeviceInfoMutex, get_provider},
    error::AppError,
};

#[tauri::command]
pub async fn apply_nugget_tweaks(
    device_state: State<'_, DeviceInfoMutex>,
) -> Result<(), AppError> {
    let device = {
        let device_guard = device_state.lock().unwrap();
        match &*device_guard {
            Some(d) => d.clone(),
            None => return Err(AppError::NoDeviceSelected),
        }
    };

    let provider = get_provider(&device.info).await?;

    let mut _backup_client = MobileBackup2Client::connect(&provider).await.map_err(|e| {
        AppError::DeviceComsWithMessage("Failed to connect to mobilebackup2".into(), e.to_string())
    })?;

    Ok(())
}

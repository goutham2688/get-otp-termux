use bindings::{
    Windows::Data::Xml::Dom::XmlDocument,
    Windows::UI::Notifications::ToastNotification,
    Windows::UI::Notifications::ToastNotificationManager,
};
// use windows::*;

pub fn display_win_toast_notification(display_data: &str)  -> Result<bool, &'static str> {
    
    match do_toast(stich_data_into_xml(display_data)) {
        Ok(..) => return Ok(true),
        Err(..) => return Err("ERROR in creating toast notification"),
    }
}

fn stich_data_into_xml(display_data: &str) -> String{
    let xml_data = r#"<toast duration="long">
        <visual>
            <binding template="ToastGeneric">
                <text hint-align="Center">holder</text>
                <image placement="appLogoOverride" hint-crop="circle" src="file:///c:/po.jpg" alt="po inner peace" />
                <text placement="attribution">Via Termux</text>
            </binding>
        </visual>
        <audio src="ms-winsoundevent:Notification.SMS" />
    </toast>"#;
    return xml_data.replace("holder", display_data);
}

fn do_toast(toast_data: String)  -> windows::Result<()> {
    let toast_xml = XmlDocument::new()?;
    // println!("do_toast: {:#?}",toast_data);
    // taost design styling guide
    // https://docs.microsoft.com/en-us/windows/uwp/design/shell/tiles-and-notifications/adaptive-interactive-toasts?tabs=xml
    toast_xml.LoadXml(windows::HSTRING::from(toast_data)).expect("the xml is malformed");

    // Create the toast and attach event listeners
    let toast_notification = ToastNotification::CreateToastNotification(toast_xml)?;

    // If you have a valid app id, (ie installed using wix) then use it here.
    let toast_notifier = ToastNotificationManager::CreateToastNotifierWithId(windows::HSTRING::from(
        "{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe",
    ))?;

    // Show the toast.
    // Note this returns success in every case, including when the toast isn't shown.
    toast_notifier.Show(&toast_notification)?;
    Ok(())
}

pub fn copy_otp_to_clipboard() {
    
}
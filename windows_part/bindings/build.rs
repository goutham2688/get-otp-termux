fn main() {
    windows::build!(
        Windows::Data::Xml::Dom::XmlDocument,
        Windows::UI::Notifications::ToastNotification,
        Windows::UI::Notifications::ToastNotificationManager,
        Windows::UI::Notifications::ToastNotifier,
    );
}
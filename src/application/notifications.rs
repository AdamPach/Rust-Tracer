use eframe::egui;
use eframe::egui::{RichText, TextWrapMode, Ui};
use std::time::SystemTime;
#[derive(Clone)]

enum NotificationType {
    Info,
    Ok,
    Error,
}
#[derive(Clone)]

enum NotificationTimeout {
    Temporary(SystemTime),
    Persistent,
}

#[derive(Clone)]
pub struct Notification {
    message: String,
    notification_type: NotificationType,
    timeout: NotificationTimeout,
}

impl Notification {
    pub fn ok(message: String) -> Notification {
        Notification {
            message,
            notification_type: NotificationType::Ok,
            timeout: NotificationTimeout::Temporary(SystemTime::now()),
        }
    }

    pub fn info(message: String) -> Notification {
        Notification {
            message,
            notification_type: NotificationType::Info,
            timeout: NotificationTimeout::Persistent,
        }
    }

    pub fn error(message: String) -> Notification {
        Notification {
            message,
            notification_type: NotificationType::Error,
            timeout: NotificationTimeout::Persistent,
        }
    }

    pub fn is_expired(&self) -> bool {
        match self.timeout {
            NotificationTimeout::Temporary(start_time) => {
                start_time.elapsed().unwrap_or_default() > std::time::Duration::from_secs(5)
            }
            NotificationTimeout::Persistent => false,
        }
    }
}

pub struct NotificationsView<'a> {
    notifications: &'a Vec<Notification>,
}

impl NotificationsView<'_> {
    pub fn new(notifications: &'_ Vec<Notification>) -> NotificationsView<'_> {
        NotificationsView { notifications }
    }

    pub fn ui<T: FnMut(usize)>(&self, ui: &mut Ui, mut on_click: T) {
        egui::CollapsingHeader::new("Notifications")
            .default_open(true)
            .show(ui, |ui| {
                for (i, n) in self.notifications.iter().enumerate() {
                    let text = RichText::new(&n.message).color(match n.notification_type {
                        NotificationType::Info => egui::Color32::DARK_BLUE,
                        NotificationType::Error => egui::Color32::RED,
                        NotificationType::Ok => egui::Color32::DARK_GREEN,
                    });

                    ui.add(egui::Label::new(text).wrap_mode(TextWrapMode::Wrap))
                        .clicked()
                        .then(|| {
                            println!("Clicked notification: {}", n.message);
                            on_click(i);
                        });
                }
            });
    }
}

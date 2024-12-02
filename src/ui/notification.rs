use bevy::prelude::*;

const NOTIFICATION_TIMER: f32 = 4.0;
const NOTIFICATION_SIZE: f32 = 20.0;

#[derive(Event)]
pub struct NotificationEvent(pub String);

#[derive(Component)]
pub struct NotificationZone;

#[derive(Component)]
pub struct Notification {
    pub timer: Timer,
}

pub fn read_notification_events(
    mut commands: Commands,
    mut notification_events: EventReader<NotificationEvent>,
    zone: Single<Entity, With<NotificationZone>>,
) {
    for event in notification_events.read() {
        commands.entity(*zone).with_children(|c| {
            c.spawn((
                Text::new(event.0.clone()),
                TextFont {
                    font_size: NOTIFICATION_SIZE,
                    ..default()
                },
                Notification {
                    timer: Timer::from_seconds(NOTIFICATION_TIMER, TimerMode::Once),
                },
            ));
        });
    }
}

pub fn update_notifications(
    time: Res<Time>,
    mut commands: Commands,
    mut q_notifications: Query<(Entity, &mut Notification)>,
) {
    for (entity, mut notification) in &mut q_notifications {
        if notification.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

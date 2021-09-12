use std::lazy::SyncLazy;

use serenity::{http::Http, model::id::ChannelId, prelude::RwLock};
use tokio::{runtime::Handle, task};
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::utils::send_embed;

pub const HORARIOS: &str = "https://club-de-algoritmia-acatlan-guapa.github.io/horarios/";
static DISCORD_CLIENT: SyncLazy<RwLock<Http>> = SyncLazy::new(|| {
    let token = std::env::var("DISCORD_TOKEN").unwrap();
    RwLock::new(Http::new_with_token(&token))
});

pub enum Channel {
    Basicos = 761049813373943808,
    Intermedios = 761049843518799913,
    General = 750731685632671766,
}

pub enum Sesion {
    Basicos,
    Intermedios,
    OfficeHours,
}

pub fn send_session_reminder(sesion: Sesion) {
    // <@&u64> is the syntax for role mentions, where u64 is the id of the role.

    let (channel, msg) = match sesion {
        Sesion::Basicos => (Channel::Basicos, "<@&750744271509782548>"),
        Sesion::Intermedios => (Channel::Intermedios, "<@&750743415150346253>"),
        Sesion::OfficeHours => (Channel::General, "Office Hours, ven a preguntar o convivir"),
    };

    task::block_in_place(|| {
        Handle::current().block_on(async move {
            let channel = ChannelId(channel as u64);
            let client = DISCORD_CLIENT.read().await;

            if let Err(e) = send_embed(&channel, client.as_ref(), "Sesión", HORARIOS, msg).await {
                eprintln!("{:?}", e);
            }
        });
    });
}

/// This will send a reminder of the GUAPA sessions at the appropriate time and channel.
pub async fn session_alerts() {
    // References:
    // - https://club-de-algoritmia-acatlan-guapa.github.io/horarios/
    // - https://crontab.guru

    let mut alerts = Vec::new();

    // Básicos
    alerts.push(Job::new("1 55 17 * * 3,4,5", |_, _| {
        send_session_reminder(Sesion::Basicos);
    }));

    // Intermedios
    alerts.push(Job::new("1 55 17 * * 2,6", |_, _| {
        send_session_reminder(Sesion::Intermedios);
    }));

    // Office Hours
    let office_hours = |_, _| send_session_reminder(Sesion::OfficeHours);
    alerts.push(Job::new("1 55 22 * * 3", office_hours)); //
    alerts.push(Job::new("1 55 18 * * 6", office_hours));

    let mut sched = JobScheduler::new();

    for alert in alerts {
        sched.add(alert.unwrap()).unwrap();
    }

    tokio::spawn(async move {
        if let Err(e) = sched.start().await {
            eprintln!("Error on alert {}", e);
        }
    });
}

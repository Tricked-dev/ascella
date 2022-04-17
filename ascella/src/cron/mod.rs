use futures::executor::block_on;
use twilight_model::id::ChannelId;

use crate::{
  database::queries::{get_images::delete_all, get_users_autodelete},
  prelude::{create_embed, HTTP},
};

pub async fn start_cron() {
  let mut sched = tokio_cron_scheduler::JobScheduler::new();

  sched
    .add(
      tokio_cron_scheduler::Job::new_repeated(
        //one day
        core::time::Duration::from_secs(86400000),
        |_, _| {
          tokio::spawn(async {
            //This removes the startup run but it isnt needed anyway!
            if let Some(client) = HTTP.get() {
              let users = get_users_autodelete::exec().await;
              if users.is_err() {
                return;
              }
              let summary: Vec<String> = users
                .unwrap()
                .iter()
                .filter_map(|x| {
                  block_on(async {
                    let amount = delete_all(x.0, x.1).await;
                    if let Ok(amount) = amount {
                      if amount == 0 {
                        None
                      } else {
                        Some(format!("{}: `{}`", x.2, amount))
                      }
                    } else {
                      None
                    }
                  })
                })
                .collect();
              if summary.is_empty() {
                return;
              }

              let embed = create_embed().title("Deleted images summary").description(&summary.join("\n")).build().unwrap();
              client.create_message(ChannelId::new(929698255300882522u64).unwrap()).embeds(&vec![embed]).unwrap().exec().await.ok();
            }
          });
        },
      )
      .unwrap(),
    )
    .unwrap();

  sched.start().await.unwrap();
}

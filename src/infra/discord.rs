use crate::domain::{models::{Event as AppEvent, Platform, Proposal}, ports::Driver};
use serenity::all::*;
use std::sync::Arc;
use tokio::sync::mpsc;
use anyhow::Result;

struct Handler {
    tx: mpsc::Sender<AppEvent>,
    guild: GuildId,
    staff: RoleId,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        let opts = || vec![
            CreateCommandOption::new(CommandOptionType::User, "target", "Target User").required(true),
            CreateCommandOption::new(CommandOptionType::String, "reason", "Action Reason").required(true)
        ];
        let _ = self.guild.set_commands(&ctx, vec![
            CreateCommand::new("ban").description("Propose ban").set_options(opts()),
            CreateCommand::new("kick").description("Propose kick").set_options(opts())
        ]).await;
    }

    async fn interaction_create(&self, ctx: Context, int: Interaction) {
        let (has_role, author) = match &int {
            Interaction::Command(c) => (c.user.has_role(&ctx, self.guild, self.staff).await.unwrap_or(false), c.user.name.clone()),
            Interaction::Component(c) => (c.user.has_role(&ctx, self.guild, self.staff).await.unwrap_or(false), c.user.name.clone()),
            _ => return,
        };

        if !has_role { return; }

        match int {
            Interaction::Command(cmd) => {
                let (mut target, mut reason) = (String::new(), String::new());
                for opt in &cmd.data.options() {
                    match opt.value {
                        ResolvedValue::User(u, _) => target = u.id.to_string(),
                        ResolvedValue::String(s) => reason = s.to_string(),
                        _ => {}
                    }
                }
                let _ = self.tx.send(AppEvent::Propose { action: cmd.data.name.clone(), target, author, origin: Platform::Discord, channel: cmd.channel_id.to_string(), reason }).await;
                let _ = cmd.create_response(&ctx, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("⏳ Proposed.").ephemeral(true))).await;
            }
            Interaction::Component(cmd) => {
                if let Some(target) = cmd.data.custom_id.strip_prefix("ok:") {
                    let _ = self.tx.send(AppEvent::Approve { target: target.into(), approver: author }).await;
                    let _ = cmd.create_response(&ctx, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("✅ Approved.").ephemeral(true))).await;
                }
            }
            _ => {}
        }
    }
}

pub struct Discord {
    http: Arc<Http>,
    log: ChannelId,
}

impl Discord {
    pub async fn new(token: &str, guild: u64, staff: u64, log: u64, tx: mpsc::Sender<AppEvent>) -> Result<Self> {
        let mut client = Client::builder(token, GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MEMBERS)
            .event_handler(Handler { tx, guild: GuildId::new(guild), staff: RoleId::new(staff) }).await?;
        tokio::spawn(async move { let _ = client.start().await; });
        Ok(Self { http: Arc::new(Http::new(token)), log: ChannelId::new(log) })
    }
}

#[async_trait]
impl Driver for Discord {
    async fn notify(&self, p: &Proposal) -> Result<()> {
        let ch = if p.origin == Platform::Discord { ChannelId::new(p.channel.parse()?) } else { self.log };
        let target_display = format!("<@{}>", p.target.discord.as_deref().unwrap_or(&p.target.raw));
        let embed = CreateEmbed::new().title(format!("{} Proposal", p.action)).field("Target", target_display, true).field("Reason", &p.reason, false);
        let btn = CreateButton::new(format!("ok:{}", p.target.raw)).label("Confirm").style(ButtonStyle::Danger);
        let _ = ch.send_message(&self.http, CreateMessage::new().embed(embed).components(vec![CreateActionRow::Buttons(vec![btn])])).await;
        Ok(())
    }

    async fn execute(&self, p: &Proposal, app: &str) -> Result<()> {
        let ch = if p.origin == Platform::Discord { ChannelId::new(p.channel.parse()?) } else { self.log };
        if let Some(did) = &p.target.discord {
            let uid = UserId::new(did.parse()?);
            if let Ok(channel) = ch.to_channel(&self.http).await {
                if let Some(guild_info) = channel.guild() {
                    let audit = format!("Req: {} App: {}", p.author, app);
                    if p.action == "ban" { let _ = guild_info.guild_id.ban_with_reason(&self.http, uid, 0, &audit).await; }
                    if p.action == "kick" { let _ = guild_info.guild_id.kick_with_reason(&self.http, uid, &audit).await; }
                }
            }
        }
        let _ = ch.say(&self.http, format!("✅ Executed {} on {}", p.action, p.target.raw)).await;
        Ok(())
    }
}
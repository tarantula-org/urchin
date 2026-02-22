use crate::config::AppConfig;
use crate::domain::{models::{Event as AppEvent, Platform, Proposal}, ports::Driver};
use ::anyhow::Result;
use ::serenity::all::*;
use ::std::sync::Arc;
use ::tokio::sync::mpsc;

struct Handler { tx: mpsc::Sender<AppEvent>, guild: GuildId, staff: RoleId }

#[::async_trait::async_trait]
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
                let (mut target, mut reason) = (::std::string::String::new(), ::std::string::String::new());
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
                let mut msg = cmd.message.clone();
                if let Some(target) = cmd.data.custom_id.strip_prefix("ok:") {
                    let _ = self.tx.send(AppEvent::Approve { target: target.into(), approver: author }).await;
                    let _ = cmd.create_response(&ctx, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("✅ Processing...").ephemeral(true))).await;
                } else if let Some(target) = cmd.data.custom_id.strip_prefix("no:") {
                    let _ = self.tx.send(AppEvent::Cancel { target: target.into(), author }).await;
                    let _ = msg.edit(&ctx, EditMessage::new().components(vec![])).await;
                    let _ = cmd.create_response(&ctx, CreateInteractionResponse::Acknowledge).await;
                }
            }
            _ => {}
        }
    }
}

pub struct Discord { http: Arc<Http>, log: ChannelId, config: Arc<AppConfig> }

impl Discord {
    pub async fn new(token: &str, guild: u64, staff: u64, log: u64, tx: mpsc::Sender<AppEvent>, config: Arc<AppConfig>) -> Result<Self> {
        let mut client = Client::builder(token, GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MEMBERS)
            .event_handler(Handler { tx, guild: GuildId::new(guild), staff: RoleId::new(staff) }).await?;
        ::tokio::spawn(async move { let _ = client.start().await; });
        Ok(Self { http: Arc::new(Http::new(token)), log: ChannelId::new(log), config })
    }
}

#[::async_trait::async_trait]
impl Driver for Discord {
    async fn notify(&self, p: &Proposal) -> Result<()> {
        let ch = if p.origin == Platform::Discord { ChannelId::new(p.channel.parse().unwrap_or(self.log.get())) } else { self.log };
        let embed = CreateEmbed::new().title(format!("{} Proposal", p.action))
            .field("Target", format!("<@{}>", p.target.discord.as_deref().unwrap_or(&p.target.raw)), true)
            .field("Reason", &p.reason, false)
            .field("Approvals", format!("{}/{}", p.approvers.len(), self.config.required_approvals), true);
        let btn_ok = CreateButton::new(format!("ok:{}", p.target.raw)).label("Confirm").style(ButtonStyle::Success);
        let btn_no = CreateButton::new(format!("no:{}", p.target.raw)).label("Cancel").style(ButtonStyle::Danger);
        let _ = ch.send_message(&self.http, CreateMessage::new().embed(embed).components(vec![CreateActionRow::Buttons(vec![btn_ok, btn_no])])).await;
        Ok(())
    }

    async fn execute(&self, p: &Proposal, app: &str) -> Result<()> {
        let ch = if p.origin == Platform::Discord { ChannelId::new(p.channel.parse().unwrap_or(self.log.get())) } else { self.log };
        if let Some(did) = &p.target.discord {
            if let Ok(uid_val) = did.parse::<u64>() {
                if let Ok(c) = ch.to_channel(&self.http).await {
                    if let Some(g) = c.guild() {
                        let audit = format!("Req: {} App: {}", p.author, app);
                        let uid = UserId::new(uid_val);
                        if p.action == "ban" { let _ = g.guild_id.ban_with_reason(&self.http, uid, 0, &audit).await; }
                        if p.action == "kick" { let _ = g.guild_id.kick_with_reason(&self.http, uid, &audit).await; }
                    }
                }
            }
        }
        let _ = ch.say(&self.http, format!("✅ Executed {} on {}", p.action, p.target.raw)).await;
        Ok(())
    }

    async fn discard(&self, p: &Proposal, reason: &str) -> Result<()> {
        let ch = if p.origin == Platform::Discord { ChannelId::new(p.channel.parse().unwrap_or(self.log.get())) } else { self.log };
        let _ = ch.say(&self.http, format!("🚫 {} proposal for {} discarded: {}", p.action, p.target.raw, reason)).await;
        Ok(())
    }
}
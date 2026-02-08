use async_trait::async_trait;
use serenity::all::{
    GatewayIntents, Client, Http, ChannelId, UserId, GuildId,
    CreateMessage, CreateEmbed, CreateActionRow, CreateButton, ButtonStyle,
    EventHandler, Context, Interaction, RoleId, Ready, Command,
    CreateCommand, CreateCommandOption, CommandOptionType, ResolvedValue, CreateInteractionResponse, CreateInteractionResponseMessage
};
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::domain::ports::{PlatformNotifier, BanProposal, UrchinEvent};

struct DiscordHandler {
    tx: mpsc::Sender<UrchinEvent>,
    guild_id: GuildId,
    staff_role_id: RoleId,
}

#[async_trait]
impl EventHandler for DiscordHandler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let ban_cmd = CreateCommand::new("ban")
            .description("Propose a ban for a user")
            .add_option(CreateCommandOption::new(CommandOptionType::User, "target", "The user to ban").required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::String, "reason", "Reason for the ban").required(true));

        let kick_cmd = CreateCommand::new("kick")
            .description("Propose a kick for a user")
            .add_option(CreateCommandOption::new(CommandOptionType::User, "target", "The user to kick").required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::String, "reason", "Reason for the kick").required(true));

        let _ = Command::set_global_commands(&ctx.http, vec![]).await;
        let _ = self.guild_id.set_commands(&ctx.http, vec![ban_cmd, kick_cmd]).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => {
                let has_role = command.user.has_role(&ctx.http, command.guild_id.unwrap(), self.staff_role_id).await.unwrap_or(false);
                if !has_role {
                    let _ = command.create_response(&ctx.http, CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content("❌ Access Denied.").ephemeral(true)
                    )).await;
                    return;
                }

                let kind = command.data.name.clone(); 
                let mut target_id = String::new();
                let mut reason = String::from("No reason provided");

                for option in &command.data.options() {
                    match option.value {
                        ResolvedValue::User(u, _) => target_id = u.id.to_string(),
                        ResolvedValue::String(s) => reason = s.to_string(),
                        _ => {},
                    }
                }

                let _ = self.tx.send(UrchinEvent::RequestAction {
                    kind,
                    target: target_id.clone(),
                    requester: command.user.name.clone(),
                    channel_id: command.channel_id.to_string(),
                    reason,
                }).await;

                let _ = command.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(format!("⏳ Proposal Logged. Target: <@{}>", target_id))
                        .ephemeral(true)
                    )).await;
            },
            Interaction::Component(component) => {
                if component.data.custom_id.starts_with("confirm_") {
                    let has_role = component.user.has_role(&ctx.http, component.guild_id.unwrap(), self.staff_role_id).await.unwrap_or(false);
                    if !has_role {
                        let _ = component.create_response(&ctx.http, CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new().content("❌ Access Denied.").ephemeral(true)
                        )).await;
                        return;
                    }

                    let parts: Vec<&str> = component.data.custom_id.split(':').collect();
                    if parts.len() < 2 { return; }
                    
                    let target_id = parts[1].to_string();
                    
                    let _ = self.tx.send(UrchinEvent::ConfirmAction {
                        target: target_id,
                        approver: component.user.name.clone(),
                    }).await;

                    let _ = component.create_response(&ctx.http, CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content("✅ Consensus Verified. Executing...").ephemeral(true)
                    )).await;
                }
            },
            _ => {}
        }
    }
}

pub struct DiscordAdapter {
    http: Arc<Http>,
}

impl DiscordAdapter {
    pub async fn new(token: &str, guild_id: u64, staff_role_id: u64, tx: mpsc::Sender<UrchinEvent>) -> anyhow::Result<Self> {
        let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MEMBERS; 
        
        let handler = DiscordHandler { 
            tx,
            guild_id: GuildId::new(guild_id),
            staff_role_id: RoleId::new(staff_role_id),
        };
        
        let mut client = Client::builder(token, intents).event_handler(handler).await?;

        tokio::spawn(async move {
            let _ = client.start().await;
        });

        Ok(Self { http: Arc::new(Http::new(token)) })
    }
}

#[async_trait]
impl PlatformNotifier for DiscordAdapter {
    async fn notify_proposal(&self, proposal: &BanProposal) -> anyhow::Result<()> {
        let channel = ChannelId::new(proposal.channel_id.parse::<u64>()?);

        let color = if proposal.kind == "ban" { 0xFF0000 } else { 0xFFA500 }; 
        let emoji = if proposal.kind == "ban" { '🔨' } else { '👢' };

        let embed = CreateEmbed::new()
            .title(format!("{} {} Proposal", emoji, proposal.kind.to_uppercase()))
            .color(color)
            .field("Target", format!("<@{}>", proposal.target_id), true)
            .field("Requester", &proposal.requester_id, true)
            .field("Reason", &proposal.reason, false)
            .footer(serenity::all::CreateEmbedFooter::new("Requires Consensus Approval"));

        let btn = CreateButton::new(format!("confirm_{}:{}", proposal.kind, proposal.target_id))
            .label(format!("Confirm {}", proposal.kind.to_uppercase()))
            .style(ButtonStyle::Danger);

        let msg = CreateMessage::new().embed(embed).components(vec![CreateActionRow::Buttons(vec![btn])]);
        channel.send_message(&self.http, msg).await?;
        Ok(())
    }

    async fn execute_action(&self, proposal: &BanProposal, approver: &str) -> anyhow::Result<()> {
        let channel = ChannelId::new(proposal.channel_id.parse::<u64>()?);
        
        if let Ok(ch) = channel.to_channel(&self.http).await {
            if let Some(guild_id) = ch.guild().map(|g| g.guild_id) {
                let user_id = UserId::new(proposal.target_id.parse::<u64>().unwrap_or(0));
                let audit_reason = format!("{} | Req: {} | App: {}", proposal.reason, proposal.requester_id, approver);

                if proposal.kind == "ban" {
                    guild_id.ban_with_reason(&self.http, user_id, 0, &audit_reason).await?; 
                    channel.say(&self.http, format!("🔨 **Banned** <@{}>\n**Reason:** {}", proposal.target_id, proposal.reason)).await?;
                } else if proposal.kind == "kick" {
                    guild_id.kick_with_reason(&self.http, user_id, &audit_reason).await?;
                    channel.say(&self.http, format!("👢 **Kicked** <@{}>\n**Reason:** {}", proposal.target_id, proposal.reason)).await?;
                }
            }
        }
        Ok(())
    }
}
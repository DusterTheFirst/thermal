use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::ReactionType,
        interactions::{
            application_command::{ApplicationCommand, ApplicationCommandOptionType},
            message_component::ButtonStyle,
            Interaction, InteractionApplicationCommandCallbackDataFlags,
        },
        prelude::{Activity, OnlineStatus, Ready},
    },
    utils::Color,
};
use tracing::{error, info, warn};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Err(err) = match interaction {
            Interaction::ApplicationCommand(command) => match command.data.name.as_str() {
                "print" => {
                    command
                        .create_interaction_response(&ctx, |response| {
                            response.interaction_response_data(|data| {
                                data.create_embed(|embed| {
                                    embed.color(Color::DARK_GREEN).title("New receipt")
                                })
                                .components(|components| {
                                    components
                                        .create_action_row(|row| {
                                            row.create_button(|button| {
                                                button
                                                    .label("Cut")
                                                    .style(ButtonStyle::Primary)
                                                    .custom_id("cut")
                                            })
                                        })
                                        .create_action_row(|row| {
                                            row.create_select_menu(|menu| {
                                                menu.custom_id("justify")
                                                    .placeholder("Justify")
                                                    .options(|options| {
                                                        options
                                                            .create_option(|option| {
                                                                option
                                                                    .label("Left")
                                                                    .value("left")
                                                                    .emoji(ReactionType::Unicode(
                                                                        "⬅️".into(),
                                                                    ))
                                                            })
                                                            .create_option(|option| {
                                                                option
                                                                    .label("Center")
                                                                    .value("center")
                                                                    .emoji(ReactionType::Unicode(
                                                                        "↔️".into(),
                                                                    ))
                                                            })
                                                            .create_option(|option| {
                                                                option
                                                                    .label("Right")
                                                                    .value("right")
                                                                    .emoji(ReactionType::Unicode(
                                                                        "➡️".into(),
                                                                    ))
                                                            })
                                                    })
                                            })
                                        })
                                        .create_action_row(|row| {
                                            row.create_button(|button| {
                                                button
                                                    .label("Submit")
                                                    .style(ButtonStyle::Success)
                                                    .custom_id("submit")
                                            })
                                            .create_button(|button| {
                                                button
                                                    .label("Cancel")
                                                    .style(ButtonStyle::Danger)
                                                    .custom_id("cancel")
                                            })
                                        })
                                })
                            })
                        })
                        .await
                }
                _ => {
                    command
                        .create_interaction_response(&ctx, |response| {
                            response.interaction_response_data(|data| {
                                data.create_embed(|embed| {
                                    embed
                                        .color(Color::DARK_RED)
                                        .title("Unimplemented")
                                        .description("This command is not implemented yet")
                                })
                                .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                            })
                        })
                        .await
                }
            },
            Interaction::MessageComponent(_) => todo!(),
            Interaction::Ping(_) => Ok(()),
        } {
            error!("Encountered an error responding to an interaction: {}", err);
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_presence(
            Some(Activity::playing("with a thermal printer :D")),
            OnlineStatus::Online,
        )
        .await;

        info!(user = ?ready.user.tag(), "Connected");

        let commands = ApplicationCommand::set_global_application_commands(&ctx, |commands| {
            commands
                .create_application_command(|command| {
                    command
                        .name("start-print")
                        .description("Create a print command to send to the thermal printer")
                })
                .create_application_command(|command| {
                    command
                        .name("justify")
                        .description("Set the justification for the print")
                })
                .create_application_command(|command| {
                    command
                        .name("paper-type")
                        .description("Select the paper type to print on")
                        .create_option(|option| {
                            option
                                .name("type")
                                .description("The type of paper to print on")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                                .add_string_choice("roll", "roll")
                                .add_string_choice("slip", "slip")
                                .add_string_choice("slip back", "slip_back")
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("finish-print")
                        .description("Finish and send print to the thermal printer")
                })
        })
        .await;

        match commands {
            Ok(commands) => {
                info!(
                    "Successfully setup {} slash commands: {:?}",
                    commands.len(),
                    commands.iter().map(|c| &c.name).collect::<Vec<_>>()
                )
            }
            Err(error) => {
                error!(?error, "Failed to setup the slash commands");
                warn!("This may cause problems with discord being out of sync with the supported slash commands");
            }
        }
    }
}

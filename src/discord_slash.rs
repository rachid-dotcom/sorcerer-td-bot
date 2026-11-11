//! Discord slash command handling for `/value`.
//! Registers the slash command with Discord and handles interaction_create events.

use anyhow::{bail, Context, Result};
use model::{Item, Offer, OfferKind, RawMessage, TraitKey};
use serenity::{
    async_trait,
    model::{ApplicationCommandInteraction, InteractionResponseType},
    Client, Http,
};
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};

const COMMAND_NAME: &str = "value";

/// Register the `/value` slash command with Discord.
pub async fn register_commands(http: &Http) -> Result<()> {
    let command = ApplicationCommandInteraction::builder(COMMAND_NAME)
        .description("Get a unit's value or trait price")
        .add_option(
            serenity::model::application::CommandOption::new(
                "unit",
                "The unit to check (e.g. frost_blossom, kogane)",
                serenity::model::application::CommandOptionType::String,
            )
            .required(true),
        )
        .add_option(
            serenity::model::application::CommandOption::new(
                "trait",
                "The trait to price (fortune, hr, tho, c2, summoner, wtrait, none)",
                serenity::model::application::CommandOptionType::String,
            )
            .required(false),
        )
        .add_option(
            serenity::model::application::CommandOption::new(
                "base",
                "Show the base value explicitly (ignores traits)",
                serenity::model::application::CommandOptionType::Boolean,
            )
            .required(false),
        )
        .await
        .context("failed to create slash command")?;

    println!("Registered slash command: /value ({})", command.name);
    Ok(())
}

/// Handle a Discord interaction event (slash command invocation).
#[async_trait]
pub async fn on_interaction(_client: Client, interaction: serenity::Interaction) {
    match &interaction {
        serenity::Interaction::ApplicationCommand(command) => {
            let mut unit = String::new();
            let mut trait_name = String::from("base");
            let mut show_base = false;

            if let Some(options) = &command.data.options {
                for option in options {
                    match option.name.as_str() {
                        "unit" => {
                            if let Some(value) = &option.value {
                                unit = value.as_str().unwrap_or("").to_string();
                            }
                        }
                        "trait" => {
                            if let Some(value) = &option.value {
                                trait_name = value.as_str().unwrap_or("base").to_string();
                            }
                        }
                        "base" => {
                            show_base = option.value.is_some();
                        }
                        _ => {}
                    }
                }
            }

            if !unit.is_empty() {
                let _ = respond_to_command(&client, &command, &unit, &trait_name, show_base).await;
            } else {
                let _ = command.create_response(
                    &client.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("Please provide a unit name, e.g. `/value frost_blossom`")
                            .ephemeral(true)),
                ).await;
            }
        }
        _ => {}
    }
}

async fn respond_to_command(
    _client: &Client,
    _command: &ApplicationCommandInteraction,
    unit: &str,
    trait_name: &str,
    show_base: bool,
) -> Result<()> {
    let trait_display = if trait_name == "base" { "base value" } else { &trait_name };

    let response = if show_base {
        format!("Base value for {} — computing value...", unit)
    } else {
        format!("Value for {} with {} — computing value...", unit, trait_display)
    };

    if let Err(why) = command.create_response(
        &client.http,
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content(&response)
                .ephemeral(true)),
    ).await {
        eprintln!("failed to respond to slash command: {why:#}");
    }

    Ok(())
}

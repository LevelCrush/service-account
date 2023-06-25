import dotenv from 'dotenv';
import { ParseArgsConfig, parseArgs } from 'node:util';
import * as discord from './discord';
import { CategoryChannel, CategoryChildChannel, ChannelType, Events, User, channelLink } from 'discord.js';
import RoleDecay, { RoleDecayManager, RoleMonitorCleanup } from './role_decay';
import { ChannelLogger, ChannelLoggerCleanup } from './channel_logger';
import { category_active_users, channel_active_users } from './api/query';

// import env settings into the process env
dotenv.config();

function generate_invite_link() {
    const client_id = process.env['DISCORD_CLIENT_ID'] || '';
    return (
        'https://discord.com/api/oauth2/authorize?client_id=' +
        encodeURIComponent(client_id) +
        '&permissions=0&scope=bot%20applications.commands'
    );
}

async function bot() {
    const client = discord.create();
    const commands = await discord.slash_commands();

    const guild_decays = new Map<string, RoleMonitorCleanup>();
    const guild_logs = new Map<string, ChannelLoggerCleanup>();

    // anything that should happen once the client has is ready
    client.on(Events.ClientReady, async () => {
        console.log('Client ready!');

        console.log('Setting up role decay and channel logs');
        const target_category = (process.env['ROLE_DECAY_CATEGORY'] || '').toLowerCase().split(',');
        const target_role = process.env['ROLE_NAME_DECAY'] || '';
        const target_decay_time = parseInt(process.env['ROLE_DECAY_TIME_SECONDS'] || '60');
        const target_decay_interval_check = parseInt(process.env['ROLE_DECAY_INTERVAL_CEHCK_SECS'] || '60');
        const target_channels = (process.env['ROLE_DECAY_CHANNEL'] || '').split(',');

        console.log(target_category);
        console.log(target_channels);
        const decay_manager = RoleDecay(target_role, target_decay_time, target_decay_time);
        const log_manager = ChannelLogger();
        if (target_role && !isNaN(target_decay_time) && !isNaN(target_decay_interval_check)) {
            const guilds = await client.guilds.cache;
            for (const [guild_id, guild] of guilds) {
                const unix_timestamp = Math.ceil(Date.now() / 1000);
                const timestamp = unix_timestamp - (target_decay_time + 1);
                console.log(timestamp);
                console.log('Getting category members for guild', guild.name, 'at category', target_category);
                const last_interaction_map = new Map<string, number>();
                for (const cat of target_category) {
                    const category_users = await category_active_users(guild.id, cat, timestamp);
                    for (const cat_user of category_users) {
                        last_interaction_map.set(cat_user.member_id, parseInt(cat_user.message_timestamp));
                    }
                }

                console.log('Getting category members for guild', guild.name, 'at channels', target_category);
                for (const chan of target_channels) {
                    const chan_users = await channel_active_users(guild.id, chan, timestamp);
                    for (const chan_user of chan_users) {
                        last_interaction_map.set(chan_user.member_id, parseInt(chan_user.message_timestamp));
                    }
                }

                console.log(last_interaction_map);

                decay_manager.set_dont_want(guild, new Map()); // for now empty
                decay_manager.set_last_interactions(guild, last_interaction_map); // for now empty
                guild_decays.set(guild.id, decay_manager.monitor(client, guild, target_category, target_channels));
                guild_logs.set(guild.id, log_manager.monitor(client, guild));
            }
        }
    });

    // handle auto complete commands
    client.on(Events.InteractionCreate, async (interaction) => {
        if (!interaction.isAutocomplete()) {
            return;
        }

        const command = commands.get(interaction.commandName);
        if (!command || !command.autocomplete) {
            console.log('Command not accepted: ', interaction.commandName);
        } else {
            try {
                await command.autocomplete(interaction);
            } catch (err) {
                console.log('Auto complete failed! ', err);
            }
        }
    });

    // handle slash commands
    client.on(Events.InteractionCreate, async (interaction) => {
        if (!interaction.isChatInputCommand()) {
            return;
        }

        const command = commands.get(interaction.commandName);
        if (!command) {
            console.log('Command not accepted: ', interaction.commandName);
        } else {
            try {
                await command.execute(interaction);
            } catch (err) {
                console.log('Command failed! ', err);

                if (interaction.replied || interaction.deferred) {
                    await interaction.followUp({
                        content: 'An internal error occurred during execution',
                        ephemeral: true,
                    });
                }

                await interaction.reply({
                    content: 'An internal error occurred during execution',
                    ephemeral: true,
                });
            }
        }
    });

    await discord.connect(client);
}

async function main() {
    const options = {
        invite: {
            type: 'boolean',
            short: 'i',
        },
    } as ParseArgsConfig['options'];

    const app_args = parseArgs({ options });
    if ((app_args.values as any)['invite']) {
        console.log('Generating an invite link. ');
        console.info(generate_invite_link());
    } else {
        await bot();
    }
}

main().catch((err) => console.log('An internal error occurred:', err));

import dotenv from 'dotenv';
import { ParseArgsConfig, parseArgs } from 'node:util';
import * as discord from './discord';
import { CategoryChannel, CategoryChildChannel, ChannelType, Events, channelLink } from 'discord.js';
import RoleDecay, { RoleDecayManager, RoleMonitorCleanup } from './role_decay';

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

    const role_decay_cleanups = [] as RoleMonitorCleanup[];

    // anything that should happen once the client has is ready
    client.on(Events.ClientReady, async () => {
        console.log('Client ready!');

        const target_category = process.env['ROLE_DECAY_CATEGORY'] || '';
        const target_role = process.env['ROLE_NAME_DECAY'] || '';
        const target_decay_time = parseInt(process.env['ROLE_DECAY_TIME_SECONDS'] || '60');
        const target_decay_interval_check = parseInt(process.env['ROLE_DECAY_INTERVAL_CEHCK_SECS'] || '60');
        const manager = RoleDecay(target_role, target_decay_time, target_decay_time);
        if (target_role && !isNaN(target_decay_time) && !isNaN(target_decay_interval_check)) {
            const guilds = await client.guilds.cache;
            for (const [guild_id, guild] of guilds) {
                manager.set_dont_want(guild, new Map()); // for now empty
                manager.set_last_interactions(guild, new Map()); // for now empty
                role_decay_cleanups.push(manager.monitor(client, guild, target_category));
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

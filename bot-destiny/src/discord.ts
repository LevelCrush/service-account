import { Client, Events, GatewayIntentBits, Partials, REST, Routes, Interaction } from 'discord.js';
import LeaderboardCommand from './commands/leaderboard';
import { Command } from './commands/base_command';

/**
 * Creates a Discord client based off .env variables
 * @returns
 */
export function create() {
    const client = new Client({
        partials: [Partials.Message, Partials.Channel, Partials.Reaction],
        intents: Object.keys(GatewayIntentBits) as any,
    });
    return client;
}

/**
 * Connect the bot to discord
 * @param client
 */
export async function connect(client: Client) {
    client.once(Events.ClientReady, (c) => {
        console.log('Bot connected', c.user.tag);
    });

    await client.login(process.env['DISCORD_BOT_TOKEN'] || '');
}

/**
 * Generate a map of slash commands and register them to discord
 * @returns A map of slash commands
 */
export async function slash_commands() {
    // todo!
    const command_map = new Map<string, Command>();
    command_map.set(LeaderboardCommand.name, LeaderboardCommand);

    // now parse commands
    const commands = [];
    for (const command of command_map.values()) {
        commands.push(command.data.toJSON());
    }

    try {
        console.log('Registering slash commands');
        const rest = new REST().setToken(process.env['DISCORD_BOT_TOKEN'] || '');

        const debug_guild = process.env['DEBUG_GUILD'] || '';
        if (debug_guild) {
            const data = await rest.put(
                Routes.applicationGuildCommands(process.env['DISCORD_CLIENT_ID'] || '', debug_guild),
                {
                    body: commands,
                },
            );
            console.log('Loaded ', (data as any).length, ' guild specific application commands');
        } else {
            const data = await rest.put(Routes.applicationCommands(process.env['DISCORD_CLIENT_ID'] || ''), {
                body: commands,
            });
            console.log('Loaded ', (data as any).length, ' global application commands');
        }
    } catch (err) {
        console.log('Could not register commands: ', err);
    }

    return command_map;
}

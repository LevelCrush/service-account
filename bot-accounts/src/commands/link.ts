import {
    SlashCommandBuilder,
    ChatInputCommandInteraction,
    InteractionResponse,
    Message,
    MessageFlags,
} from 'discord.js';
import { Command } from './base_command';
import type { APIResponse } from '@levelcrush';

const COMMAND_NAME = 'link';

export const LinkCommand = {
    /**
     * Name of the command
     */
    name: COMMAND_NAME,
    /**
     * Configure command
     */
    data: new SlashCommandBuilder()
        .setName(COMMAND_NAME)
        .setDescription('Link a supported platform to your discord')
        .addSubcommand((subcommand) => subcommand.setName('bungie').setDescription('Link your bungie account'))
        .addSubcommand((subcommand) => subcommand.setName('twitch').setDescription('Link your twitch account')),

    /**
     *  Execute command logic
     * @param interaction
     */
    execute: async (interaction: ChatInputCommandInteraction) => {
        const subcommand = interaction.options.getSubcommand(true);

        const user = interaction.user.id;
        await interaction.reply({
            content: 'Generating you a private link. One second!',
            ephemeral: true,
        });

        const gen_key = process.env['ACCOUNT_KEY'] || '';
        const endpoint = process.env['HOST_ACCOUNTS'] || '';
        const link_request = await fetch(endpoint + '/link/generate', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Account-Key': gen_key,
            },
            body: JSON.stringify({
                id: interaction.user.id,
            }),
        });

        let code = '';
        if (link_request.ok) {
            const json = (await link_request.json()) as APIResponse<{ code: string }>;
            code = json.response && json.response !== null ? json.response.code : '';
        }

        if (code) {
            const link = endpoint + '/link/platform/' + subcommand + '?code=' + encodeURIComponent(code);
            await interaction.followUp({
                content: 'Please follow this link to validate: ' + link,
                ephemeral: true,
                flags: [MessageFlags.SuppressEmbeds],
            });
        } else {
            await interaction.followUp({
                content: 'No code was provided. Cannot link currently',
                ephemeral: true,
            });
        }
    },
} as Command;

export default LinkCommand;

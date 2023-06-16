import { SlashCommandBuilder, ChatInputCommandInteraction } from 'discord.js';
import { Command } from './base_command';

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
        console.log(user, interaction.user);

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

        const msg = await interaction.reply({
            content: 'Generating you a private link. One second!',
            ephemeral: true,
        });

        await interaction.followUp({
            content: 'Now executing',
            ephemeral: true,
        });
    },
} as Command;

export default LinkCommand;

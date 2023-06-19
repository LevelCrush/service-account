import {
    SlashCommandBuilder,
    ChatInputCommandInteraction,
    InteractionResponse,
    Message,
    MessageFlags,
    AutocompleteInteraction,
    ApplicationCommandChoicesData,
    ApplicationCommandOptionChoiceData,
} from 'discord.js';
import { Command } from './base_command';
import type { APIResponse, ServiceDestiny } from '@levelcrush';
import { getDestinyModeGroups } from '@levelcrush/service-destiny';

const COMMAND_NAME = 'leaderboard';

export const LeaderboardCommand = {
    /**
     * Name of the command
     */
    name: COMMAND_NAME,
    /**
     * Configure command
     */
    data: new SlashCommandBuilder()
        .setName(COMMAND_NAME)
        .setDescription('Clan related Leaderboards')
        .addStringOption((option) =>
            option
                .setName('type')
                .setAutocomplete(true)
                .setDescription('The type of leaderboard/activity you want to get')
                .setRequired(true),
        ),

    /**
     * Auto complete ooptions for this command
     * @param interaction
     */
    autocomplete: async (interaction: AutocompleteInteraction) => {
        const focused = interaction.options.getFocused();

        const endpoint = process.env['HOST_DESTINY'] || '';
        let modes = await getDestinyModeGroups(endpoint);

        // remove the first mode
        modes = modes.slice(1);

        const filtered = modes.filter((choice) => choice.name.startsWith(focused));

        const respond_width = filtered.map((choice) => {
            return {
                name: choice.name,
                value: choice.name,
            };
        }) as ApplicationCommandOptionChoiceData[];
        await interaction.respond(respond_width);
    },
    /**
     *  Execute command logic
     * @param interaction
     */
    execute: async (interaction: ChatInputCommandInteraction) => {
        await interaction.reply({
            content: 'Placeholder',
            ephemeral: true,
        });
    },
} as Command;

export default LeaderboardCommand;

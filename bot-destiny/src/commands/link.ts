import {
    SlashCommandBuilder,
    ChatInputCommandInteraction,
    InteractionResponse,
    Message,
    MessageFlags,
} from 'discord.js';
import { Command } from './base_command';
import type { APIResponse } from '@levelcrush';

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

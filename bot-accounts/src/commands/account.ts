import { SlashCommandBuilder, ChatInputCommandInteraction, InteractionResponse, Message } from 'discord.js';
import { Command } from './base_command';
import type { APIResponse } from '@levelcrush';

const COMMAND_NAME = 'account';

export const AccountCommand = {
    /**
     * Name of command
     */
    name: COMMAND_NAME,

    /**
     * Command configuration
     */
    data: new SlashCommandBuilder().setName(COMMAND_NAME).setDescription('Get your linked account information'),

    /**
     * Execute command logic
     * @param interaction
     */
    execute: async (interaction: ChatInputCommandInteraction) => {
        interaction.reply({
            content: 'Checking for your linked level crush accounts',
            ephemeral: true,
        });
    },
} as Command;

export default AccountCommand;

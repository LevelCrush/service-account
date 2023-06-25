import {
    SlashCommandBuilder,
    ChatInputCommandInteraction,
    InteractionResponse,
    Message,
    MessageFlags,
} from 'discord.js';
import { Command } from './base_command';
import type { APIResponse } from '@levelcrush';

const COMMAND_NAME = 'role';

export const RoleCommand = {
    /**
     * Name of the command
     */
    name: COMMAND_NAME,
    /**
     * Configure command
     */
    data: new SlashCommandBuilder()
        .setName(COMMAND_NAME)
        .setDescription('Control your roles')
        .addSubcommand((subcommand) =>
            subcommand
                .setName('add')
                .setDescription('Add a role')
                .addRoleOption((option) =>
                    option.setName('role').setDescription('The role you want to give yourself').setRequired(true),
                ),
        )
        .addSubcommand((subcommand) =>
            subcommand
                .setName('remove')
                .setDescription('Remove role from yourself')
                .addRoleOption((option) =>
                    option.setName('role').setDescription('The role you want to remove').setRequired(true),
                ),
        )
        .addSubcommand((subcommand) =>
            subcommand
                .setName('deny')
                .setDescription("Don't allow a role to be assigned to you")
                .addRoleOption((option) =>
                    option.setName('role').setDescription('The role you want to never receive').setRequired(true),
                ),
        )
        .addSubcommand((subcommand) =>
            subcommand
                .setName('accept')
                .setDescription('Allow a role the potential to be assigned to you')
                .addRoleOption((option) =>
                    option
                        .setName('role')
                        .setDescription('The role you want the potential to receive')
                        .setRequired(true),
                ),
        ),
    /*  Execute command logic
     * @param interaction
     */
    execute: async (interaction: ChatInputCommandInteraction) => {
        const subcommand = interaction.options.getSubcommand(true);

        await interaction.reply({
            content: 'Place holder for now',
            ephemeral: true,
        });
    },
} as Command;

export default RoleCommand;

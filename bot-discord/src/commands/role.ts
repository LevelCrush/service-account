import {
    SlashCommandBuilder,
    ChatInputCommandInteraction,
    InteractionResponse,
    Message,
    MessageFlags,
} from 'discord.js';
import { Command } from './base_command';
import type { APIResponse } from '@levelcrush';
import { role_allow, role_deny } from '../api/settings';

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
                .setName('allow')
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

        await interaction.deferReply({
            ephemeral: true,
        });

        const role = interaction.options.getRole('role', true);

        if (subcommand === 'deny') {
            await role_deny(interaction.guildId || '0', interaction.user.id, role.name);
            await interaction.followUp({
                content: 'From now on, you will not be able to receive the @' + role.name + ' role',
                ephemeral: true,
            });
            interaction.client.emit('role_deny', interaction.guildId || '', interaction.user.id, role);
        } else if (subcommand === 'allow') {
            await role_allow(interaction.guildId || '0', interaction.user.id, role.name);
            await interaction.followUp({
                content: 'From now on, you will able to receive the @' + role.name + ' role',
                ephemeral: true,
            });
            interaction.client.emit('role_allow', interaction.guildId || '', interaction.user.id, role);
        } else if (interaction.guild && subcommand === 'add') {
            try {
                await interaction.guild.members.addRole({
                    user: interaction.user.id,
                    role: role.id,
                });
                await interaction.followUp({
                    content: 'You should now have the @' + role.name + ' role. Enjoy!',
                    ephemeral: true,
                });
                interaction.client.emit('role_added', interaction.guildId || '', interaction.user.id, role);
            } catch (err) {
                console.error(
                    'Unable to add role to  user',
                    interaction.user.username,
                    'in guild',
                    interaction.guildId,

                    'error is',
                    err,
                );
                await interaction.followUp({
                    content: 'Something prevent the role from being added to you',
                    ephemeral: true,
                });
            }
        } else if (interaction.guild && subcommand === 'remove') {
            try {
                await interaction.guild.members.removeRole({
                    user: interaction.user.id,
                    role: role.id,
                });
                await interaction.followUp({
                    content: 'You should no longer have the  @' + role.name + ' role.',
                    ephemeral: true,
                });
                interaction.client.emit('role_removed', interaction.guildId || '', interaction.user.id, role);
            } catch (err) {
                console.error(
                    'Unable to remove role from user',
                    interaction.user.username,
                    'in guild',
                    interaction.guildId,

                    'error is',
                    err,
                );
                await interaction.followUp({
                    content: 'Something prevent the role from being removed from you',
                    ephemeral: true,
                });
            }
        }
    },
} as Command;

export default RoleCommand;

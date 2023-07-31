import {
    SlashCommandBuilder,
    ChatInputCommandInteraction,
    InteractionResponse,
    Message,
    MessageFlags,
    ApplicationCommandOptionChoiceData,
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
                .addStringOption((option) =>
                    option
                        .setName('role')
                        .setDescription('The role you want to give yourself')
                        .setRequired(true)
                        .setAutocomplete(true),
                ),
        )
        .addSubcommand((subcommand) =>
            subcommand
                .setName('remove')
                .setDescription('Remove role from yourself')
                .addStringOption((option) =>
                    option
                        .setName('role')
                        .setDescription('The role you want to remove')
                        .setRequired(true)
                        .setAutocomplete(true),
                ),
        ),

    autocomplete: async (interaction) => {
        const focused = interaction.options.getFocused();

        const options = (process.env['ROLE_MANAGE_ALLOW'] || '').split(',');
        const filtered = options.filter((option) =>
            option.trim().toLowerCase().startsWith(focused.toLowerCase().trim()),
        );

        const respond_width = filtered.map((option) => {
            return { name: option.trim(), value: option.trim() };
        }) as ApplicationCommandOptionChoiceData[];

        await interaction.respond(respond_width);
    },
    /*  Execute command logic
     * @param interaction
     */
    execute: async (interaction: ChatInputCommandInteraction) => {
        const subcommand = interaction.options.getSubcommand(true);

        await interaction.deferReply({
            ephemeral: true,
        });

        const role_name = interaction.options.getString('role', true);
        console.log(role_name);
        const role = interaction.guild?.roles.cache.find(
            (v) => v.name.trim().toLowerCase() === role_name.toLowerCase().trim(),
        );
        if (!role) {
            await interaction.followUp({
                content: 'The role you choose. Could not be found',
                ephemeral: true,
            });
            return;
        }

        if (interaction.guild && subcommand === 'add') {
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

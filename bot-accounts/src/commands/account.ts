import {
    SlashCommandBuilder,
    ChatInputCommandInteraction,
    InteractionResponse,
    Message,
    EmbedBuilder,
    RestOrArray,
    APIEmbedField,
    APIEmbed,
} from 'discord.js';
import { Command } from './base_command';
import type { APIResponse, ServiceAccounts } from '@levelcrush';
import { link } from 'fs';

const COMMAND_NAME = 'account';

export const AccountCommand = {
    /**
     * Name of command
     */
    name: COMMAND_NAME,

    /**
     * Command configuration
     */
    data: new SlashCommandBuilder()
        .setName(COMMAND_NAME)
        .setDescription('Get linked account information')
        .addSubcommand((subcommand) =>
            subcommand.setName('me').setDescription('Display your own linked account information'),
        )
        .addSubcommand((subcommand) =>
            subcommand
                .setName('user')
                .setDescription('Query another users linked account information')
                .addUserOption((option) =>
                    option.setName('user').setDescription('The user you want to query').setRequired(true),
                ),
        ),
    /**
     * Execute command logic
     * @param interaction
     */
    execute: async (interaction: ChatInputCommandInteraction) => {
        const subcommand = interaction.options.getSubcommand(true);

        await interaction.deferReply();

        const user = subcommand === 'me' ? interaction.user.username : interaction.options.getUser('user')?.username;
        if (!user) {
            await interaction.followUp({
                content: 'No user was included in the request',
                ephemeral: true,
            });
            return;
        }

        const endpoint = process.env['HOST_ACCOUNTS'] || '';
        const search_request = await fetch(endpoint + '/search/by/discord/' + encodeURIComponent(user));
        if (search_request.ok) {
            const search_response =
                (await search_request.json()) as APIResponse<ServiceAccounts.AccountLinkedPlatformsResult>;

            if (search_response.response !== null) {
                const data = search_response.response;

                const linked = [] as APIEmbedField[];
                if (data.bungie) {
                    linked.push({
                        name: 'Bungie',
                        value: data.bungie,
                        inline: true,
                    });
                }
                if (data.twitch) {
                    linked.push({
                        name: 'Twitch',
                        value: data.twitch,
                        inline: true,
                    });
                }

                if (data.discord) {
                    linked.push({
                        name: 'Discord',
                        value: data.username,
                        inline: true,
                    });
                }

                const embed = new EmbedBuilder()
                    .setTitle(data.discord + ' Linked Accounts')
                    .addFields({ name: 'Linked Accounts', value: linked.length.toString(), inline: false })
                    .addFields(linked);

                await interaction.followUp({
                    embeds: [embed],
                });
            } else {
                await interaction.followUp({
                    content: 'No linked platforms found!',
                    ephemeral: true,
                });
            }
        }
    },
} as Command;

export default AccountCommand;

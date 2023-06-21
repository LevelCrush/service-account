import {
    SlashCommandBuilder,
    ChatInputCommandInteraction,
    InteractionResponse,
    Message,
    MessageFlags,
    AutocompleteInteraction,
    ApplicationCommandChoicesData,
    ApplicationCommandOptionChoiceData,
    EmbedBuilder,
} from 'discord.js';
import { Command } from './base_command';
import type { APIResponse, ServiceDestiny } from '@levelcrush';
import { Leaderboard, getDestinyModeGroups } from '@levelcrush/service-destiny';

const COMMAND_NAME = 'rank';

export const RankCommand = {
    /**
     * Name of the command
     */
    name: COMMAND_NAME,
    /**
     * Configure command
     */
    data: new SlashCommandBuilder()
        .setName(COMMAND_NAME)
        .setDescription('Get a rank on the leaderboards')
        .addSubcommand((subcommand) =>
            subcommand
                .setName('me')
                .setDescription('Get your rank on any supported leaderboards')
                .addStringOption((option) =>
                    option
                        .setName('type')
                        .setAutocomplete(true)
                        .setDescription('The type of leaderboard/activity you want to get')
                        .setRequired(true),
                ),
        )
        .addSubcommand((subcommand) =>
            subcommand
                .setName('user')
                .setDescription('Get another users rank on the leaderboard')
                .addStringOption((option) =>
                    option
                        .setName('type')
                        .setAutocomplete(true)
                        .setDescription('The type of leaderboard/activity you want to get')
                        .setRequired(true),
                )
                .addUserOption((option) =>
                    option.setName('user').setDescription('The user you want to query').setRequired(true),
                ),
        ),

    /**
     * Auto complete ooptions for this command
     * @param interaction
     */
    autocomplete: async (interaction: AutocompleteInteraction) => {
        const focused = interaction.options.getFocused();

        const endpoint = process.env['HOST_DESTINY'] || '';
        const modes = await getDestinyModeGroups(endpoint, 'leaderboards');
        const filtered = modes.filter((choice) => choice.name.toLowerCase().startsWith(focused.toLowerCase().trim()));

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
        await interaction.deferReply({
            ephemeral: false,
        });

        const subcommand = interaction.options.getSubcommand(true);
        let target_user = '';
        const target_guild = interaction.guild;
        if (target_guild === null) {
            await interaction.followUp({
                content: 'Could not find server',
                ephemeral: true,
            });
            return;
        }

        let target_username = '';
        if (subcommand === 'me') {
            console.log('Me detected');
            target_user = interaction.user.id;
        } else {
            console.log('User detected');
            const user_field = interaction.options.getUser('user', true);
            target_user = user_field.id;
        }

        const guild_member = await target_guild.members.fetch(target_user);
        target_username = guild_member.nickname || guild_member.displayName;

        let leaderboard_type = interaction.options.getString('type', true);
        const endpoint = process.env['HOST_DESTINY'] || '';
        const modes = await getDestinyModeGroups(endpoint, 'leaderboards');

        // one more layer of validation here
        const matching_modes = modes.filter(
            (v) => v.name.toLowerCase().trim() == leaderboard_type.toLowerCase().trim(),
        );

        leaderboard_type = matching_modes.length >= 1 ? matching_modes[0].name : '';
        if (!leaderboard_type) {
            await interaction.followUp({
                content: 'No leaderboard matching your input could be found',
                ephemeral: true,
            });
            return;
        }

        const leaderboard_request = await fetch(
            endpoint + '/rank/' + encodeURIComponent(leaderboard_type) + '/' + encodeURIComponent(target_user),
        );
        let leaderboard = null;
        if (leaderboard_request.ok) {
            const json = (await leaderboard_request.json()) as APIResponse<Leaderboard>;
            leaderboard = json.response;
        }

        if (leaderboard === null) {
            interaction.followUp({
                content: 'Try again later. This rank could not be fetched',
                ephemeral: false,
            });
        } else {
            // this is hacky, but it works for now for our needs
            // will be revistiing this sooner then later
            const is_pvp = leaderboard_type.toLowerCase().includes('pvp');
            const is_raid = leaderboard_type.toLowerCase().includes('raid');

            // this is redudant. But keeping it for now
            const possible_entries = leaderboard.entries.filter((v) => {
                return v.display_name == target_user;
            });

            const target_entry = possible_entries.length > 0 ? possible_entries[0] : null;

            if (target_entry !== null) {
                const embed = new EmbedBuilder()
                    .setTitle(target_username + ' rank on the ' + leaderboard_type + ' Leaderboard')
                    .setDescription(
                        'Your standing on this leaderboard is: #' +
                            target_entry.standing +
                            ' with ' +
                            (is_pvp
                                ? ' a win rate of ' + target_entry.amount
                                : is_raid
                                ? target_entry.amount + ' full completions'
                                : target_entry.amount + ' completions'),
                    )
                    .setColor('#1ABC9C');

                await interaction.followUp({
                    embeds: [embed],
                });
            } else {
                await interaction.followUp({
                    content: is_pvp
                        ? 'Requires a linked bungie account AND 100 matches in this group of modes. You can link your account by typing `/link bungie`'
                        : 'Not found on the leaderboard. Please make sure to link a bungie account via `/link bungie`',
                    ephemeral: true,
                });
            }
        }
    },
} as Command;

export default RankCommand;

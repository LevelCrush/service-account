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

        const leaderboard_request = await fetch(endpoint + '/leaderboard/' + encodeURIComponent(leaderboard_type));
        let leaderboard = null;
        if (leaderboard_request.ok) {
            const json = (await leaderboard_request.json()) as APIResponse<Leaderboard>;
            leaderboard = json.response;
        }

        if (leaderboard === null) {
            interaction.followUp({
                content: 'Try again later. This leaderboard could not be fetched',
                ephemeral: false,
            });
        } else {
            // this is hacky, but it works for now for our needs
            // will be revistiing this sooner then later
            const is_pvp = leaderboard.name.toLowerCase().includes('pvp');
            const top = leaderboard.entries.slice(0, 10);
            const standings = top.map(
                (val) =>
                    val.standing.toString().trim().padStart(2, '0') +
                    '. ' +
                    val.display_name +
                    ' *(' +
                    val.amount +
                    (is_pvp ? '%' : '') +
                    ')*',
            );

            const frontend_url = process.env['HOST_FRONTEND'] || '';
            const url = frontend_url + '/leaderboard/' + encodeURIComponent(leaderboard_type);
            const embed = new EmbedBuilder()
                .setColor('#1ABC9C')
                .setURL(url)
                .setTitle(leaderboard_type + ' Leaderboard')
                .setDescription(
                    (is_pvp ? 'Standings are Win Rate % based' : 'Standings are based on full completions' + '\r\n') +
                        standings.join('\r\n'),
                )
                .setFooter({
                    text: 'For the full leaderboard, visit ' + url,
                });

            interaction.followUp({
                embeds: [embed],
                ephemeral: false,
            });
        }
    },
} as Command;

export default LeaderboardCommand;

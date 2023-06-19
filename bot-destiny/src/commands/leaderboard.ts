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
        let modes = await getDestinyModeGroups(endpoint);

        // remove the first mode
        modes = modes.slice(1);

        const filtered = modes.filter((choice) => choice.name.startsWith(focused.trim()));

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

        const leaderboard_type = interaction.options.getString('type', true);
        const endpoint = process.env['HOST_DESTINY'] || '';
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
            const top = leaderboard.entries.slice(0, 10);
            const standings = top.map(
                (val) =>
                    val.standing.toString().trim().padStart(2, '0') +
                    '. ' +
                    val.display_name +
                    ' *(' +
                    val.amount +
                    ')*',
            );

            const embed = new EmbedBuilder()
                .setColor('#1ABC9C')
                .setTitle(leaderboard.name)
                .setDescription(standings.join('\r\n'));
            interaction.followUp({
                embeds: [embed],
                ephemeral: false,
            });
        }
    },
} as Command;

export default LeaderboardCommand;

import {
    AutocompleteInteraction,
    ChatInputCommandInteraction,
    CommandInteraction,
    SlashCommandBuilder,
} from 'discord.js';

export interface Command {
    name: string;
    data: SlashCommandBuilder;
    autocomplete?: (interaction: AutocompleteInteraction) => Promise<void>;
    execute: (interaction: ChatInputCommandInteraction) => Promise<void>;
}

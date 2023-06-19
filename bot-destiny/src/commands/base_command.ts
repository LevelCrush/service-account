import { ChatInputCommandInteraction, CommandInteraction, SlashCommandBuilder } from 'discord.js';

export interface Command {
    name: string;
    data: SlashCommandBuilder;
    execute: (interaction: ChatInputCommandInteraction) => Promise<void>;
}

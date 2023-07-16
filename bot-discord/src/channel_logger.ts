import { Events, Client, Guild, Message, PartialMessage, Collection } from 'discord.js';
import { log_channel } from './api/channel_log';

export type ChannelLoggerCleanup = () => void;

export interface ChannelLoggerManager {
    /**
     * Monitor the target guilds and channels for the role specified at time of construction.
     * Handles all role assignment and decaying
     * @param client
     * @param target_guild
     * @param target_channels
     * @returns
     */
    monitor: (client: Client, guild: Guild) => ChannelLoggerCleanup;
}

export function ChannelLogger() {
    const monitor: ChannelLoggerManager['monitor'] = (client, target_guild) => {
        const message_create = (message: Message | PartialMessage) => log_channel('message-create', message);
        const message_delete = (message: Message | PartialMessage) => log_channel('message-delete', message);
        const message_update = (message: Message | PartialMessage) => log_channel('message-update', message);

        const message_bulk_delete = (messages: Collection<string, Message | PartialMessage>) =>
            messages.forEach((message) => log_channel('message-delete', message));

        client.on(Events.MessageCreate, message_create);
        client.on(Events.MessageDelete, message_delete);
        client.on(Events.MessageUpdate, message_update);
        client.on(Events.MessageBulkDelete, message_bulk_delete);

        return () => {
            // any cleanup here
            client.off(Events.MessageCreate, message_create);
            client.off(Events.MessageDelete, message_delete);
            client.off(Events.MessageUpdate, message_update);
            client.off(Events.MessageBulkDelete, message_bulk_delete);
        };
    };

    return { monitor } as ChannelLoggerManager;
}

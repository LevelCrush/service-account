import type { CategoryChildChannel, Message, PartialMessage } from 'discord.js';
import { ServiceDiscord } from '@levelcrush';

export type ChannelLogType = 'message-create' | 'message-update' | 'message-delete';
export async function log_channel(type: ChannelLogType, message: Message | PartialMessage) {
    console.log('Attempting to log message', message.id, 'from guild', message.guildId);

    const message_channel = message.guild
        ? message.guild.channels.cache.find((v) => v.id == message.channelId)
        : undefined;

    const category = (message.channel as CategoryChildChannel).parent;
    const member = message.member;

    console.log('Timestamp: ', message.createdTimestamp, Math.ceil(message.createdTimestamp / 1000));
    const payload = {
        guild_id: message.guildId,
        category_id: category ? category.id : 0,
        channel_id: message.channelId,
        channel_name: message_channel ? message_channel.name : message.channelId,
        member_id: member !== null ? member.user.id : 0,
        message_id: message.id,
        message_timestamp: Math.ceil(message.createdTimestamp / 1000).toString(),
        event_type: type,
        data: JSON.stringify(message.toJSON()),
    } as ServiceDiscord.ChannelLogPayload;

    const endpoint = process.env['HOST_DISCORD'] || '';
    try {
        const request = await fetch(endpoint + '/channels/log', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(payload),
        });
        if (request.ok) {
            console.log('Done logging message', message.id, 'from guild', message.guildId);
        } else {
            console.log('Failed to log message', request);
        }
    } catch (err) {
        console.log('Failed to deliver payload to host ', endpoint, 'Error is', err);
    }
}

export default log_channel;

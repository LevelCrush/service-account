import { ServiceDiscord } from '@levelcrush';

export async function role_deny(guild_id: string, member_id: string, role_name: string) {
    const host = process.env['HOST_DISCORD'] || '';
    const payload = {
        guild_id: guild_id,
        member_id: member_id,
        role_name: role_name,
    } as ServiceDiscord.BotRoleSettingPayload;

    const request = await fetch(host + '/settings/bot/role/deny', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Access-Key': process.env['ACCESS_KEY'] || '',
        },
        body: JSON.stringify(payload),
    });

    if (request.ok) {
        console.error('Saved deny for user', member_id, 'in guild', guild_id, 'for role', role_name);
    } else {
        console.error('Unable to save deny for user', member_id, 'in guild', guild_id, 'for role', role_name);
    }
}

export async function role_allow(guild_id: string, member_id: string, role_name: string) {
    const host = process.env['HOST_DISCORD'] || '';
    const payload = {
        guild_id: guild_id,
        member_id: member_id,
        role_name: role_name,
    } as ServiceDiscord.BotRoleSettingPayload;

    const request = await fetch(host + '/settings/bot/role/allow', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Access-Key': process.env['ACCESS_KEY'] || '',
        },
        body: JSON.stringify(payload),
    });

    if (request.ok) {
        console.error('Saved deny for user', member_id, 'in guild', guild_id, 'for role', role_name);
    } else {
        console.error('Unable to save deny for user', member_id, 'in guild', guild_id, 'for role', role_name);
    }
}

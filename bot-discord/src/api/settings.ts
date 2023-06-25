import { APIResponse, ServiceDiscord } from '@levelcrush';
import { BotRoleDenySetting } from '@levelcrush/service-discord';

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

export async function role_get_denies(guild_id: string, role_name: string) {
    const host = process.env['HOST_DISCORD'] || '';

    const request = await fetch(
        host + '/settings/bot/role/denies/' + encodeURIComponent(guild_id) + '/' + encodeURIComponent(role_name),
        {
            method: 'GET',
            headers: {
                'Access-Key': process.env['ACCESS_KEY'] || '',
            },
        },
    );

    if (request.ok) {
        console.error('Denies retrieved for ', guild_id, 'for role', role_name);

        const json = (await request.json()) as APIResponse<BotRoleDenySetting[]>;
        const map = json.response ? json.response.map((v) => v.member_id) : [];
        return map;
    } else {
        console.error('Unable to get denies for ', guild_id, 'for role', role_name);
        return [];
    }
}

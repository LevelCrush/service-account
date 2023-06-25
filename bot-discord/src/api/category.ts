import { APIResponse, ServiceDiscord } from '@levelcrush';

export async function category_active_users(guild: string, category_name: string, timestamp: number) {
    const host = process.env['HOST_DISCORD'] || '';
    const request = await fetch(
        host +
            '/query/guilds/' +
            encodeURIComponent(guild) +
            '/categories/' +
            encodeURIComponent(category_name) +
            '/users/active?timestamp=' +
            encodeURIComponent(timestamp),
    );

    let users = [] as ServiceDiscord.CategoryActiveUser[];
    if (request.ok) {
        const json = (await request.json()) as APIResponse<ServiceDiscord.CategoryActiveUser[]>;
        users = json.response !== null ? json.response : [];
    }
    return users;
}

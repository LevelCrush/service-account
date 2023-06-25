import { Channel } from 'diagnostics_channel';
import {
    Events,
    Client,
    Guild,
    CategoryChildChannel,
    User,
    Message,
    Role,
    GuildMember,
    Collection,
    APIRole,
    PartialGuildMember,
} from 'discord.js';

export type UserMap = Map<string, User>;
export type UserTimestampMap = Map<string, number>;
export type RoleMonitorCleanup = () => void;

export interface RoleDecayManager {
    /**
     * Sets the map on a per guild basis of users who last intereacted. Intended to be called before monitor()
     * @param guild
     * @param users
     * @returns
     */
    set_last_interactions: (guild: Guild, users: Map<string, number>) => void;

    /**
     * A map of users per guild, that have explicitly stated to not receive the role. Intended to be called before monitor()
     * @param guild
     * @param users
     * @returns
     */
    set_dont_want: (guild: Guild, users: UserMap) => void;

    /**
     * Monitor the target guilds and channels for the role specified at time of construction.
     * Handles all role assignment and decaying
     * @param client
     * @param target_guild
     * @param target_channels
     * @returns
     */
    monitor: (client: Client, guild: Guild, category: string[], channels: string[]) => RoleMonitorCleanup;
}

/**
 * Constructs a role manager that targets a role by NAME. Handles decaying and checking on users for that decay
 * @param role The name of the role we want to target in the guild.
 * @param decay_time_seconds The max amount of time that a user can have this role without any interaction in the channel
 * @param decay_check_interval_sec How often we should check for role decay
 * @returns RoleDecayManager
 */
export function RoleDecay(role: string, decay_time_seconds: number, decay_check_interval_secs: number) {
    const target_role = role.toLowerCase();
    const users_last_interacted = new Map<string, UserTimestampMap>();
    const users_dont_want = new Map<string, UserMap>();

    const set_last_interactions: RoleDecayManager['set_last_interactions'] = (target_guild, users) => {
        users_last_interacted.set(target_guild.id, users);
    };

    const set_dont_want: RoleDecayManager['set_dont_want'] = (target_guild, users) => {
        users_dont_want.set(target_guild.id, users);
    };

    const monitor: RoleDecayManager['monitor'] = (client, target_guild, target_categories, target_channels) => {
        // this function will always query the guild we are targeting and find the role that matches the name
        // this is not the most efficient way to go about it, but it is the most flexible
        const find_role = () => {
            const roles = target_guild.roles.cache.filter((role) => role.name.toLowerCase() == target_role);
            return roles.size > 0 ? (roles.first() as Role) : null;
        };

        // anytime a message is created, this will fire
        const monitor_messages = (message: Message) => {
            // skip processing if the message received is not for our target guild or for our target channels or has explictly stated to not be included
            if (
                !message.member ||
                message.guildId !== target_guild.id ||
                users_dont_want.get(message.guildId)?.has(message.author.id)
            ) {
                return;
            }

            // check to see if this channel is a child of the target category
            const message_channel = message.channel as CategoryChildChannel;
            const is_in_category =
                message_channel.parent && target_categories.includes(message_channel.parent.name.toLowerCase());

            const channel = target_guild.channels.cache.find((v) => v.id === message.channel.id);
            const is_in_channel = channel ? target_channels.includes(channel.name.toLowerCase()) : false;

            // if we are neither in a designated channel AND not in category we dont want to worry abotu adding or tracking this
            if (!is_in_category && !is_in_channel) {
                return;
            }

            const has_role = message.member.roles.cache.some((role) => role.name.toLowerCase() === target_role);
            if (!has_role) {
                const server_role = find_role();
                if (server_role) {
                    message.member.roles
                        .add(server_role)
                        .then(() => {
                            console.log('Role added to ', message.author.username, ' in ', target_guild.name);
                        })
                        .catch((err) => {
                            console.log(
                                'Role could not be added to ',
                                message.author.username,
                                ' in ',
                                target_guild.name,
                                ' error ',
                                err,
                            );
                        });
                } else {
                    console.error('Role', target_role, 'not defined. Cannot assign to', message.author.username);
                }
            }

            if (users_last_interacted.has(message.guildId)) {
                users_last_interacted.get(message.guildId)?.set(message.author.id, Date.now() / 1000);
            }
        };

        const check_for_decay = async () => {
            const now_timestamp = Date.now() / 1000;
            const guild_interactions = users_last_interacted.get(target_guild.id);
            const decayed_role = find_role();

            if (decayed_role === null) {
                console.error('Role', target_role, 'not defined, cannot continue decay check');
                return;
            }
            // no entries in our guild map OR it was never defined in the first place. Just move on if that is the case.Log for errors
            if (!guild_interactions) {
                console.error('Guild not found in interaction map', target_guild.id);
                return;
            }

            let guild_members = new Collection<string, GuildMember>();
            try {
                guild_members = await target_guild.members.fetch();
            } catch (err) {
                guild_members = new Collection<string, GuildMember>();
                console.error('Unable to fetch guild members for', target_guild.name, 'error was', err);
            }

            for (const [discord_id, member] of guild_members) {
                const timestamp = guild_interactions.get(discord_id) || 0;
                const has_decayed = now_timestamp - timestamp > decay_time_seconds;
                const has_role = member.roles.cache.some((role) => role.id === decayed_role.id);
                const not_wanted = users_dont_want.get(target_guild.id)?.has(discord_id);
                if (has_role && (has_decayed || not_wanted)) {
                    try {
                        await member.roles.remove(decayed_role);
                        console.log(
                            'Removed role',
                            target_role,
                            'from user',
                            member.user.username,
                            'in server',
                            target_guild.name,
                        );
                    } catch (err) {
                        console.error('Error occurred either while fetching user or while modifying user role:', err);
                    }
                }
            }
        };

        const role_added = (guild: string, user: string, role: Role | APIRole) => {
            if (target_guild.id === guild && role.name.toLowerCase() === target_role) {
                users_last_interacted.get(guild)?.set(user, Date.now() / 1000);
            }
        };

        const role_allowed = (guild: string, user: string, role: Role | APIRole) => {
            if (target_guild.id === guild && role.name.toLowerCase() === target_role) {
                users_dont_want.get(guild)?.delete(user);
            }
        };

        const role_deny = async (guild: string, user: string, role: Role | APIRole) => {
            if (target_guild.id === guild && role.name.toLowerCase() === target_role) {
                const guild_member = target_guild.members.cache.find((v) => v.id === user);
                if (guild_member) {
                    users_dont_want.get(guild)?.set(user, guild_member.user);

                    const role = find_role();
                    if (role) {
                        try {
                            await target_guild.members.removeRole({
                                user: user,
                                role: role.id,
                            });
                            console.log(
                                'User had role',
                                role.name,
                                ' in ',
                                target_guild.name,
                                ' but is set to auto deny. Role has been removed',
                            );
                        } catch (err) {
                            console.error(
                                'Error removing role from user automatically',
                                role.name,
                                ' in ',
                                target_guild.name,
                                'error is',
                                err,
                            );
                        }
                    }
                }
            }
        };

        const member_updated = async (
            old_member: GuildMember | PartialGuildMember,
            new_member: GuildMember | PartialGuildMember,
        ) => {
            const role = find_role();
            if (
                role &&
                new_member.roles.cache.has(role.id) &&
                users_dont_want.get(new_member.guild.id)?.has(new_member.id)
            ) {
                try {
                    await new_member.guild.members.removeRole({
                        user: new_member.id,
                        role: role.id,
                    });
                    console.log(
                        'User had role',
                        role.name,
                        ' in ',
                        new_member.guild.name,
                        ' but is set to auto deny. Role has been removed',
                    );
                } catch (err) {
                    console.error(
                        'Error removing role from user automatically',
                        role.name,
                        ' in ',
                        new_member.guild.name,
                        'error is',
                        err,
                    );
                }
            }
        };

        // subscribe to events
        client.on(Events.MessageCreate, monitor_messages);
        client.on(Events.GuildMemberUpdate, member_updated);

        client.on('role_added', role_added);
        client.on('role_allow', role_allowed);
        client.on('role_deny', role_deny);

        const decay_interval = setInterval(() => {
            console.info('Checking', target_guild.name, 'for  any decay');
            check_for_decay();
        }, 1000 * decay_check_interval_secs);

        // cleanup
        return () => {
            client.off(Events.MessageCreate, monitor_messages);
            client.off(Events.GuildMemberUpdate, member_updated);
            client.off('role_added', role_added);
            client.off('role_allow', role_allowed);
            client.off('role_deny', role_deny);

            clearInterval(decay_interval);
        };
    };

    return { monitor, set_dont_want, set_last_interactions } as RoleDecayManager;
}

export default RoleDecay;

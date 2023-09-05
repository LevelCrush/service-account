import {
    ChannelType,
    Client,
    Events,
    Guild,
    GuildMember,
    OverwriteType,
    PermissionsBitField,
    VoiceChannel,
    VoiceState,
} from 'discord.js';

export interface JoinToCreateConfig {
    [channel_name: string]: {
        name: string;
        amount: number;
    };
}

export type JoinToCreateCleanup = () => void;

export interface JoinToCreateManager {
    /**
     * Provide configuration options for join to create
     * @param config
     * @returns
     */
    configure: (config: JoinToCreateConfig) => void;

    /**
     * Monitors voice channels in the specified guild with the supplied client
     * Will handle setting up anything that the guild needs to monitor successfully
     * @param client
     * @param guild
     * @returns
     */
    monitor: (client: Client, guild: Guild) => JoinToCreateCleanup;
}

export function JoinToCreate() {
    // managed vc information
    const managed_vc = {} as { [guild_id: string]: { [channel_id: string]: VoiceChannel } };
    const managed_vc_types = {} as { [guild_id: string]: { [channel_id: string]: string } };

    // owner permissions for vc
    const vc_owner_permissions = [
        PermissionsBitField.Flags.PrioritySpeaker,
        PermissionsBitField.Flags.MoveMembers,
        PermissionsBitField.Flags.ManageChannels,
    ];

    // configuration options
    let config = {} as JoinToCreateConfig;
    const configure: JoinToCreateManager['configure'] = (input_config) => {
        config = { ...input_config };
        console.log('Configurations set', config);
    };

    const monitor: JoinToCreateManager['monitor'] = (client, guild) => {
        managed_vc[guild.id] = {};
        managed_vc_types[guild.id] = {};

        // this event handler exlcusively just watches join to create voice channels and creates them as neccessary
        const jtc_watch = async (old_state: VoiceState, new_state: VoiceState) => {
            try {
                if (!new_state.channel) {
                    return;
                }
                if (new_state.guild.id !== guild.id) {
                    return;
                }
                const channel = new_state.channel;
                const channel_name = channel.name;
                if (typeof config[channel_name] !== 'undefined') {
                    // does this channel name exist in our voice configuration
                    const user = new_state.member;
                    if (!user) {
                        return;
                    }
                    const jtc_config = config[channel_name];
                    const category = channel.parent;

                    // count how many of this type there is
                    const similiar_vc_types = guild.voiceStates.cache.filter((chan) => {
                        if (chan.channelId === null) {
                            return false;
                        }

                        console.log(managed_vc_types[guild.id], managed_vc_types[guild.id][chan.channelId]);

                        return (
                            typeof managed_vc_types[guild.id] !== 'undefined' &&
                            typeof managed_vc_types[guild.id][chan.channelId] !== 'undefined' &&
                            managed_vc_types[guild.id][chan.channelId] === channel.name
                        );
                    });

                    console.log('Total Similar VC', similiar_vc_types);

                    const lookup_table = {
                        '{$username}': user.nickname || user.displayName,
                        '{$counter}': '1',
                    } as { [key: string]: string };

                    let name = '';
                    for (let i = 0; i < similiar_vc_types.size + 1; i++) {
                        name = jtc_config.name;
                        const is_using_counter = jtc_config.name.includes('{$counter}');
                        const internal_counter = is_using_counter ? (i + 1).toString() : i > 0 ? i.toString() : '';
                        lookup_table['{$counter}'] = internal_counter;
                        for (const lookup_var in lookup_table) {
                            name = name.replaceAll(lookup_var, lookup_table[lookup_var]);
                        }

                        const exactly_named = guild.voiceStates.cache.filter((vs) => {
                            if (vs.channel === null) {
                                return false;
                            }
                            return vs.channel.name === name;
                        });

                        if (exactly_named.size === 0) {
                            break;
                        }
                    }

                    console.log('Creating VC in', guild.name, 'Category', category ? category.id : 'No category');
                    const vc = await guild.channels.create({
                        type: ChannelType.GuildVoice,
                        parent: category,
                        name: name,
                        userLimit: jtc_config.amount > 0 ? jtc_config.amount : undefined,
                        permissionOverwrites: [
                            {
                                id: user.id,
                                allow: vc_owner_permissions,
                            },
                        ],
                        reason: 'User ' + (user.nickname || user.displayName) + ' requested to make a vc',
                    });

                    console.log('VC was created in', guild.name);

                    // move new member to vc
                    await user.voice.setChannel(vc, 'Moving user to their new VC: ' + vc.name);

                    // track
                    managed_vc[guild.id][vc.id] = vc;
                    managed_vc_types[guild.id][vc.id] = channel_name;
                }
            } catch (err) {
                console.log('An error occurred in ', guild.name, err);
            }
        };

        // this function exclusively handles watching join to create result channels and managing them
        const jtc_channel_orphans = async (old_state: VoiceState, new_state: VoiceState) => {
            try {
                const is_guild_vc = old_state.guild.id === guild.id;
                const old_member = old_state.member;
                const old_vc_is_managed = typeof managed_vc[guild.id][old_state.channelId || ''] !== 'undefined';
                const vc_changed = new_state.channelId !== old_state.channelId;
                if (old_vc_is_managed && vc_changed && is_guild_vc) {
                    const old_channel = old_state.channel;
                    if (old_channel === null || old_member === null) {
                        return;
                    }

                    const old_member_permissions = old_channel.permissionsFor(old_member.id);
                    const was_owner =
                        old_member_permissions !== null
                            ? old_member_permissions.has(PermissionsBitField.Flags.ManageChannels)
                            : false;
                    const other_member_permissions = old_channel.permissionOverwrites;
                    const other_owners = [] as string[];
                    other_member_permissions.cache.forEach((perm_overwrite) => {
                        if (
                            perm_overwrite.id !== old_member.id &&
                            perm_overwrite.allow.has(PermissionsBitField.Flags.ManageChannels)
                        ) {
                            other_owners.push(perm_overwrite.id);
                        }
                    });
                    // no one is left. This VC is orphaned. Delete it
                    try {
                        if (old_channel.members.size === 0) {
                            console.log('Deleting channel: ', old_channel.name);
                            await old_channel.delete('No members remaining. Removing');
                        } else if (was_owner && old_channel.members.size > 0 && other_owners.length > 0) {
                            console.log('Removing owner permissions for', old_channel.name);
                            await old_channel.permissionOverwrites.delete(
                                old_member.id,
                                'Owner no longer in vc.  Multiple owners exist',
                            );
                        } else if (was_owner && old_channel.members.size > 0 && other_owners.length === 0) {
                            // find a new owner
                            const member = old_channel.members.first();
                            if (member) {
                                console.log(
                                    'Assigning owner to: ',
                                    member.nickname || member.displayName,
                                    'in server: ',
                                    guild.name,
                                );

                                await old_channel.permissionOverwrites.edit(
                                    member,
                                    {
                                        ManageChannels: true,
                                        PrioritySpeaker: true,
                                        MoveMembers: true,
                                    },
                                    {
                                        reason: 'New VC Owner: ' + (member.nickname || member.displayName),
                                    },
                                );

                                console.log('Removing owner permissions for', old_channel.name);
                                await old_channel.permissionOverwrites.delete(
                                    old_member.id,
                                    'Transfered owner to ' + (member.nickname || member.displayName),
                                );
                            }
                        }
                    } catch (err) {
                        console.log('Unable to claim or modify the orphaned voice channel in', guild.name, err);
                    }
                }
            } catch (err) {
                console.error('An error occurred in ', guild.name, err);
            }
        };

        client.on(Events.VoiceStateUpdate, jtc_watch);
        client.on(Events.VoiceStateUpdate, jtc_channel_orphans);
        return () => {
            client.off(Events.VoiceStateUpdate, jtc_watch);
            client.off(Events.VoiceStateUpdate, jtc_channel_orphans);
        };
    };

    return { monitor, configure } as JoinToCreateManager;
}

export default JoinToCreate;

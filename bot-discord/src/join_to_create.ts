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
    const managed_vc = {} as { [channel_id: string]: VoiceChannel };

    // owner permissions for vc
    const vc_owner_permissions = [
        PermissionsBitField.Flags.ManageChannels,
        PermissionsBitField.Flags.DeafenMembers,
        PermissionsBitField.Flags.PrioritySpeaker,
        PermissionsBitField.Flags.ModerateMembers,
        PermissionsBitField.Flags.MuteMembers,
        PermissionsBitField.Flags.ManageRoles,
    ];

    // configuration options
    let config = {} as JoinToCreateConfig;
    const configure: JoinToCreateManager['configure'] = (input_config) => {
        config = { ...input_config };
        console.log('Configurations set', config);
    };

    const monitor: JoinToCreateManager['monitor'] = (client, guild) => {
        //

        // this event handler exlcusively just watches join to create voice channels and creates them as neccessary
        const jtc_watch = async (old_state: VoiceState, new_state: VoiceState) => {
            if (!new_state.channel) {
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
                const vc = await guild.channels.create({
                    type: ChannelType.GuildVoice,
                    parent: category,
                    name: jtc_config.name,
                    permissionOverwrites: [
                        {
                            id: user.id,
                            allow: vc_owner_permissions,
                        },
                    ],
                });

                // move new member to vc
                await user.voice.setChannel(vc.id, 'Auto VC Creation');

                // track
                managed_vc[vc.id] = vc;
            }
        };

        // this function exclusively handles watching join to create result channels and managing them
        const jtc_manage = async (old_state: VoiceState, new_state: VoiceState) => {
            //
            const old_member = old_state.member;
            const new_member = new_state.member;
            const old_vc_is_managed = typeof managed_vc[old_state.channelId || ''] !== 'undefined';
            const vc_changed = new_state.channelId !== old_state.channelId;
            if (old_vc_is_managed && vc_changed) {
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
                    if (perm_overwrite.allow.has(PermissionsBitField.Flags.ManageChannels)) {
                        other_owners.push(perm_overwrite.id);
                    }
                });
                // no one is left. This VC is orphaned. Delete it
                if (old_channel.members.size === 0) {
                    console.log('Deleting channel: ', old_channel.name);
                    await old_channel.delete('No members remaining. Removing');
                } else if (old_channel.members.size > 0 && other_owners.length > 0) {
                    console.log('Removing owner permissions for', old_channel.name);
                    await old_channel.permissionOverwrites.delete(old_member.id);
                } else if (old_channel.members.size > 0 && other_owners.length === 0) {
                    // find a new owner
                    const member = old_channel.members.first();
                    if (member) {
                        await old_channel.permissionOverwrites.edit(member, {
                            ManageRoles: true,
                            ManageChannels: true,
                            PrioritySpeaker: true,
                            DeafenMembers: true,
                            MuteMembers: true,
                            ModerateMembers: true,
                        });
                    }
                }
            }
        };

        client.on(Events.VoiceStateUpdate, jtc_watch);
        client.on(Events.VoiceStateUpdate, jtc_manage);

        return () => {
            client.off(Events.VoiceStateUpdate, jtc_watch);
            client.off(Events.VoiceStateUpdate, jtc_manage);
        };
    };

    return { monitor, configure } as JoinToCreateManager;
}

export default JoinToCreate;

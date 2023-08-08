import { useEffect, useState } from 'react';
import { MemberResponse, APIResponse } from '../ipc/bindings';
import { H4 } from '@components/elements/headings';
import { HyperlinkButton } from '@components/elements/button';
import Hyperlink from '@components/elements/hyperlink';
import { getMemberInfo } from '@ipc/service-destiny';

export interface DestinyMemberCardProps {
  data?: MemberResponse;
  display_name: string;
  className?: string;
  asHeaders?: boolean;
}

export const DestinyMemberCard = (props: DestinyMemberCardProps) => {
  const [memberData, setMemberData] = useState(props.data ? props.data : null);

  useEffect(() => {
    if (!props.data && !props.asHeaders) {
      const getMemberResponse = async () => {
        const data = await getMemberInfo(props.display_name);
        setMemberData(data.response);
      };

      getMemberResponse().finally(() => {
        console.log('Member search done');
      });
    }
  }, []);

  const defaultMemberInformation = {
    display_name: 'Not Found',
    display_name_platform: 'Not Found',
    known_display_names: {},
    membership_id: '0',
    membership_platform: 0,
    timestamp_last_played: 0,
    raid_report: '',
    clan: undefined,
  } as MemberResponse;

  const memberInfo =
    memberData !== null
      ? { ...defaultMemberInformation, ...memberData }
      : defaultMemberInformation;

  // what badges to display
  const badges = {} as { [name: string]: string };
  const badgeClanColors = {
    'Level Crush': 'bg-[#50AFE0] text-black',
    'Level Stomp': 'bg-[#44A8BD] text-black',
    'Righteous Indiggnation':
      'bg-gradient-to-r from-[#F988B6] to-[#7A4359] text-[#FAF2A2] border-[#F988B6] border-[1px]',
  } as { [clan: string]: string };

  if (memberInfo.clan) {
    switch (memberInfo.clan.role) {
      case 5: {
        badges['Leader'] = 'bg-red-600 text-white';
        break;
      }
      case 3: {
        badges['Admin'] = 'bg-yellow-400 text-black';
      }
      default: {
        // do nothing
        break;
      }
    }

    badges[memberInfo.clan.name] =
      typeof badgeClanColors[memberInfo.clan.name] !== 'undefined'
        ? badgeClanColors[memberInfo.clan.name]
        : 'bg-yellow-400 text-black';
  }

  return (
    <div
      className={
        'member-card destiny flex items-center justify-between lg:gap-8 flex-wrap  px-4 py-4 ' +
        (props.className || '') +
        ' ' +
        (props.asHeaders
          ? 'sticky top-[4.5rem] bg-[#141b27] border-b-[#2f405c] border-b-4 hidden lg:flex shadow-[0px_.1rem_.1rem_1px_rgba(0,0,0,0.7)]'
          : 'even:bg-[rgba(255,255,255,.05)] odd:bg-[rgba(0,0,0,.25)]')
      }
      data-display-name={props.display_name}
      data-membership-id={memberInfo.membership_id}
      data-membership-type={memberInfo.membership_platform}
    >
      <H4 className=" mb-4 lg:mt-0 lg:mb-0  grow-2 shrink-1  basis-full text-center lg:text-left lg:basis-0 w-full lg:min-w-[20rem] text-[1.25rem] ">
        {props.asHeaders ? (
          <>Bungie ID</>
        ) : (
          <>
            <span className="inline-block lg:hidden mr-2">Bungie ID: </span>
            <Hyperlink href={memberInfo.raid_report}>
              {memberInfo.display_name}
            </Hyperlink>
          </>
        )}
      </H4>

      <div className="flex-1 text-center flex flex-wrap justify-center gap-4 w-full lg:w-[20rem] ">
        {props.asHeaders ? (
          <H4 className="text-[1.25rem]">Role/Clan</H4>
        ) : (
          Object.keys(badges).map((badge, badgeIndex) => (
            <span
              key={'member_' + props.display_name + '_badge_' + badgeIndex}
              className={
                'mb-4 lg:my-0 shrink-0 grow-0 basis-auto px-2 py-1 text-sm align-middle inline-block h-auto w-auto w-min-[6rem] w-max-[10rem] self-start border-1 ' +
                badges[badge]
              }
            >
              {badge}
            </span>
          ))
        )}
      </div>
      <div className="my-4 lg:my-0 grow-0 shrink-0 basis-auto w-full lg:w-[30rem]">
        <div className="flex gap-4 justify-center flex-wrap">
          {props.asHeaders ? (
            <H4 className="basis-auto  text-center text-[1.25rem] md:flex-1 w-full self-start">
              Linked Platforms
            </H4>
          ) : (
            <></>
          )}
          {!props.asHeaders ? (
            <HyperlinkButton
              className="md:flex-1 w-full md:w-auto md:max-w-[8rem] text-sm text-ellipsis overflow-hidden whitespace-nowrap py-3 md:py-2  self-start"
              intention="attention"
              href={
                '/admin/member/' +
                encodeURIComponent(memberInfo.display_name) +
                '/lifetime/modes/all'
              }
            >
              Overview
            </HyperlinkButton>
          ) : (
            <></>
          )}
        </div>
      </div>
    </div>
  );
};

export default DestinyMemberCard;

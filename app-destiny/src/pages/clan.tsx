import DestinyMemberCard from '@components/destiny_member_card';
import Container from '@components/elements/container';
import { H3 } from '@components/elements/headings';
import { MemberResponse } from '@ipc/bindings';
import { getNetworkRoster } from '@ipc/service-destiny';
import { useEffect, useState } from 'react';

export const ClanPage = () => {
  const [networkRoster, setNetworkRoster] = useState([] as MemberResponse[]);

  // on component mount
  useEffect(() => {
    const init_load = async () => {
      const roster = await getNetworkRoster();
      setNetworkRoster(roster);
    };

    init_load();
  }, []);

  return (
    <Container>
      <H3 className="text-yellow-400">Level Crush Network Roster</H3>
      <div className="md:flex md:justify-between md:flex-wrap relative">
        <DestinyMemberCard
          asHeaders={true}
          display_name=""
          className="w-full max-w[30rem]"
        />
        {networkRoster.map((member, memberIndex) => (
          <DestinyMemberCard
            key={'network_clan_' + '_member_' + memberIndex}
            display_name={member.display_name}
            data={member}
            className="w-full max-w[30rem]"
          ></DestinyMemberCard>
        ))}
      </div>
    </Container>
  );
};
export default ClanPage;

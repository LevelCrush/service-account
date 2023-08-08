import Head from 'next/head';
import Hero from '@website/components/hero';
import { SiteHeader } from '@website/components/site_header';
import OffCanvas from '@website/components/offcanvas';
import { GetStaticProps } from 'next';
import ENV from '@website/core/env';
import { H3 } from '@website/components/elements/headings';
import Container from '@website/components/elements/container';

import DestinyMemberCard from '@website/components/destiny_member_card';
import { MemberResponse } from '@ipc/bindings';
import { getNetworkRoster } from '@ipc/service-destiny';
import { useEffect, useState } from 'react';

export const ClanDirectoryPage = () => {
  const [roster, setRoster] = useState([] as MemberResponse[]);

  useEffect(() => {
    getNetworkRoster().then((roster) => {
      setRoster(roster);
    });
  });

  return (
    <OffCanvas>
      <Head>
        <title>Network Clan Roster | Level Crush</title>
      </Head>
      <SiteHeader />
      <main>
        <Hero
          className="min-h-[40rem] overflow-hidden top-0 relative"
          youtubeID="7RhUVDmCPkY"
        ></Hero>
        <Container>
          <H3 className="text-yellow-400">Level Crush Network Roster</H3>
          <div className="md:flex md:justify-between md:flex-wrap relative">
            <DestinyMemberCard
              asHeaders={true}
              display_name=""
              className="w-full max-w[30rem]"
            />
            {roster.map((member, memberIndex) => (
              <DestinyMemberCard
                key={'network_clan_' + '_member_' + memberIndex}
                display_name={member.display_name}
                data={member}
                className="w-full max-w[30rem]"
              ></DestinyMemberCard>
            ))}
          </div>
        </Container>
      </main>
    </OffCanvas>
  );
};
export default ClanDirectoryPage;

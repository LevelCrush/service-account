import Head from 'next/head';
import React, { useEffect, useState } from 'react';
import Hero from '@website/components/hero';
import { SiteHeader } from '@website/components/site_header';
import OffCanvas from '@website/components/offcanvas';
import { H2, H3 } from '@website/components/elements/headings';
import Container from '@website/components/elements/container';

import DestinyMemberCard from '@website/components/destiny_member_card';
import { ClanInformation, MemberResponse } from '@ipc/bindings';
import { getClanInfo, getClanRoster } from '@ipc/service-destiny';
import { useRouter } from 'next/router';

export const ClanRoster = () => {
  const [clan, setClans] = useState(null as ClanInformation | null);
  const [roster, setRoster] = useState([] as MemberResponse[]);
  const router = useRouter();

  useEffect(() => {
    console.log(router.query);
    const slug = (router.query.slug as string) || '';
    Promise.all([getClanInfo(slug as string), getClanRoster(slug as string)])
      .then(([clan_response, roster_response]) => {
        if (clan_response.response !== null) {
          setClans(clan_response.response);
        }

        if (roster_response.response !== null) {
          setRoster(roster_response.response.roster);
        }
      })
      .catch((err) => {
        console.error(err);
      });
  }, []);

  return (
    <OffCanvas>
      <Head>
        <title>{(clan ? clan.name : '') + ' Roster | Level Crush'}</title>
      </Head>
      <SiteHeader />
      <main>
        <Hero className="min-h-[40rem] overflow-hidden top-0 relative">
          <Container>
            <H2 className="drop-shadow text-center">
              {(clan ? clan.name : '') + ' Clan Roster'}
            </H2>
          </Container>
        </Hero>
        <Container>
          <H3 className="text-yellow-400">
            {(clan ? clan.name : '') + ' Roster'}
          </H3>
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
export default ClanRoster;

import Head from 'next/head';
import Hero from '@website/components/hero';
import { SiteHeader } from '@website/components/site_header';
import OffCanvas from '@website/components/offcanvas';
import { GetServerSideProps } from 'next';
import ENV from '@website/core/env';
import { H3 } from '@website/components/elements/headings';
import Container from '@website/components/elements/container';
import {
  AccountLinkedPlatformMultiSearchResponse,
  AccountLinkedPlatformResultMap,
  DestinyMemberInformation,
} from '@website/core/api_responses';
import DestinyMemberCard from '@website/components/destiny_member_card';
import { getNetworkRoster } from '@levelcrush/service-destiny';

export interface NetworkRosterPageProps {
  roster: DestinyMemberInformation[];
  linkedPlaforms: AccountLinkedPlatformResultMap;
}

export const getServerSideProps: GetServerSideProps<
  NetworkRosterPageProps
> = async () => {
  //

  const network_roster = await getNetworkRoster(ENV.hosts.destiny);
  const bungie_names = network_roster.map((member) => member.display_name);

  const account_api = ENV.hosts.accounts;
  const search_response = await fetch(account_api + '/search/by/bungie', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(bungie_names),
  });

  const parsed_search = search_response.ok
    ? ((await search_response.json()) as AccountLinkedPlatformMultiSearchResponse)
    : null;

  return {
    props: {
      roster: network_roster,
      linkedPlaforms:
        parsed_search && parsed_search.response !== null
          ? parsed_search.response
          : {},
    },
  };
};

export const ClanDirectoryPage = (props: NetworkRosterPageProps) => (
  <OffCanvas>
    <Head>
      <title>Clan Roster | Level Crush</title>
    </Head>
    <SiteHeader />
    <main>
      <Hero className="min-h-[40rem] overflow-hidden top-0 relative"></Hero>
      <Container>
        <H3 className="text-yellow-400">Level Crush Network Roster</H3>
        <div className="md:flex md:justify-between md:flex-wrap relative">
          <DestinyMemberCard
            asHeaders={true}
            display_name=""
            className="w-full max-w[30rem]"
          />
          {props.roster.map((member, memberIndex) => (
            <DestinyMemberCard
              key={'network_clan_' + '_member_' + memberIndex}
              display_name={member.display_name}
              data={member}
              platforms={props.linkedPlaforms[member.display_name]}
              className="w-full max-w[30rem]"
            ></DestinyMemberCard>
          ))}
        </div>
      </Container>
    </main>
  </OffCanvas>
);

export default ClanDirectoryPage;

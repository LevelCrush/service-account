import Head from 'next/head';
import React from 'react';
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
  DestinyClanInformation,
  DestinyClanResponse,
  DestinyClanRosterResponse,
  DestinyMemberInformation,
} from '@website/core/api_responses';
import DestinyMemberCard from '@website/components/destiny_member_card';

export interface ClanRosterProps {
  clan: DestinyClanInformation;
  roster: DestinyMemberInformation[];
  linkedPlatforms: AccountLinkedPlatformResultMap;
}

export const getServerSideProps: GetServerSideProps<ClanRosterProps> = async (
  context
) => {
  //

  const slug = context.query.slug;

  const destiny_api = ENV.hosts.destiny;
  const response = await fetch(destiny_api + '/clan/' + slug);

  const clan_response = response.ok
    ? ((await response.json()) as DestinyClanResponse)
    : null;

  const api_roster_response = await fetch(
    destiny_api + '/clan/' + slug + '/roster'
  );

  const roster_response = api_roster_response.ok
    ? ((await api_roster_response.json()) as DestinyClanRosterResponse)
    : null;

  const bungie_names =
    roster_response !== null && roster_response.response !== null
      ? roster_response.response.roster.map((member) => member.display_name)
      : [];

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

  if (clan_response === null || clan_response.response === null) {
    return {
      notFound: true,
    };
  } else {
    return {
      props: {
        clan: clan_response.response,
        roster:
          roster_response && roster_response.response !== null
            ? roster_response.response.roster
            : [],
        linkedPlatforms:
          parsed_search && parsed_search.response !== null
            ? parsed_search.response
            : {},
      },
    };
  }
};

export const ClanRoster = (props: ClanRosterProps) => (
  <OffCanvas>
    <Head>
      <title>{props.clan.name + ' Roster | Level Crush'}</title>
    </Head>
    <SiteHeader />
    <main>
      <Hero className="min-h-[40rem] overflow-hidden top-0 relative"></Hero>
      <Container>
        <H3 className="text-yellow-400">{props.clan.name + ' Roster'}</H3>
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
              platforms={props.linkedPlatforms[member.display_name]}
              className="w-full max-w[30rem]"
            ></DestinyMemberCard>
          ))}
        </div>
      </Container>
    </main>
  </OffCanvas>
);

export default ClanRoster;

import Head from 'next/head';
import React from 'react';
import Hero from '@website/components/hero';
import { SiteHeader } from '@website/components/site_header';
import DiscordLink from '@website/components/discord_link';
import Container from '@website/components/elements/container';
import { H3 } from '@website/components/elements/headings';
import OffCanvas from '@website/components/offcanvas';

export const HomePage = () => (
  <OffCanvas>
    <Head>
      <title>Home | Level Crush</title>
    </Head>
    <SiteHeader />
    <main>
      <Hero
        className="min-h-[40rem] overflow-hidden top-0 relative"
        youtubeID="5FNd-W7iEAU"
      ></Hero>
      <div className="container mx-auto">
        <div className="flex flex-wrap justify-between">
          <div className="flex-[1_1_auto] lg:flex-[0_1_40%]">
            <Container>
              <H3>WHO ARE WE ?</H3>
              <p>
                Level Crush was spoken into existence between a small circle of
                friends. We began with one mission: To build a strong community
                that is diverse, friendly, and helpful!
                <br />
                <br />
                Our venture first took place with Overwatch on Twitch.tv, then
                moved to Beam.pro, and followed them to the exciting rebranding
                into Mixer where we soared within the Destiny community.
                Although we decided to step back from content creation as a
                community, our gamers continue to raid the Hive, loot
                King&apos;s Canyon, and creep through haunted farm houses at
                night. Come join us!
                <DiscordLink />
              </p>
            </Container>
            <Container>
              <H3>LOOKING FOR GROUP ?</H3>
              <p>
                Need another Guardian to take on Atheon? How about another
                squadmate to take on the kill leader? Or maybe run some casual
                creative modes?
                <br />
                <br />
                Well, what are you waiting for? Ready up!
                <DiscordLink />
              </p>
            </Container>
          </div>
          <div className="flex-[1_1_auto] lg:flex-[0_1_30%]">
            <Container>
              <H3>Discord Activity</H3>
              <iframe
                src="https://discord.com/widget?id=303862208419594240&theme=dark"
                width="100%"
                height="1000"
                frameBorder="0"
                loading="lazy"
                sandbox="allow-popups allow-popups-to-escape-sandbox allow-same-origin allow-scripts"
              ></iframe>
            </Container>
          </div>
        </div>
      </div>
    </main>
  </OffCanvas>
);

export default HomePage;

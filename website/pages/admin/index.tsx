import Head from 'next/head';
import { SiteHeader } from '@website/components/site_header';
import Container from '@website/components/elements/container';
import OffCanvas from '@website/components/offcanvas';
import LoginGuard from '@website/components/login_guard';
import { H3 } from '@website/components/elements/headings';

export const AdminIndexPage = () => {
  return (
    <OffCanvas>
      <Head>
        <title>Admin | Level Crush</title>
      </Head>
      <SiteHeader forceStickyStyle={true} />
      <main>
        <Container className="top-[4.5rem] relative">
          <LoginGuard admin={true}>
            <H3>Oh Hey there. Something will be here...eventually. Soon(tm)</H3>
          </LoginGuard>
        </Container>
      </main>
    </OffCanvas>
  );
};

export default AdminIndexPage;

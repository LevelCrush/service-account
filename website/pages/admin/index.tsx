import Head from 'next/head';
import { SiteHeader } from '../../components/site_header';
import Container from '../../components/elements/container';
import OffCanvas from '../../components/offcanvas';
import LoginGuard from '../../components/login_guard';
import { H3 } from '../../components/elements/headings';

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

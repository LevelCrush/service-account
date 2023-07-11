import Head from 'next/head';
import { SiteHeader } from '@website/components/site_header';
import Container from '@website/components/elements/container';
import OffCanvas from '@website/components/offcanvas';
import LoginGuard from '@website/components/login_guard';
import { H3 } from '@website/components/elements/headings';
import { GetServerSideProps } from 'next';

export const getServerSideProps: GetServerSideProps = async (context) => {
  return {
    redirect: {
      destination: '/admin/clan/network/lifetime',
      permanent: false,
    },
    props: {},
  };
};
export const AdminIndexPage = () => <></>;

export default AdminIndexPage;

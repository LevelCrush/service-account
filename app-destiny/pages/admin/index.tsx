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

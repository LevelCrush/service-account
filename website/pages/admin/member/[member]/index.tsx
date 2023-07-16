import { GetServerSideProps } from 'next';

export const getServerSideProps: GetServerSideProps<
  Record<string, never>
> = async (context) => {
  return {
    redirect: {
      destination:
        '/admin/member/' +
        encodeURIComponent(context.query.member as string) +
        '/lifetime/all',
      permanent: false,
    },
  };
};

export const RedirectToReport = () => <></>;

export default RedirectToReport;

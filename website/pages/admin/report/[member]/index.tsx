import { GetServerSideProps } from 'next';

export const getServerSideProps: GetServerSideProps<
  Record<string, never>
> = async (context) => {
  return {
    redirect: {
      destination:
        '/admin/report/' +
        encodeURIComponent(context.query.member as string) +
        '/lifetime',
      permanent: false,
    },
  };
};

export const RedirectToReport = () => <></>;

export default RedirectToReport;

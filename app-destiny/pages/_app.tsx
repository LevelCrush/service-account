import '../styles/globals.css';
import { config } from '@fortawesome/fontawesome-svg-core';
import '@fortawesome/fontawesome-svg-core/styles.css';
config.autoAddCss = false;

import type { AppProps } from 'next/app';
import smoothScroll from 'smoothscroll-polyfill';
import { useEffect, useState } from 'react';
import Head from 'next/head';
import NextNProgress from 'nextjs-progressbar';
import { useRouter } from 'next/router';

function MyApp({ Component, pageProps }: AppProps) {
  useEffect(() => {
    smoothScroll.polyfill();
  });

  const [isLoading, setLoading] = useState(false);

  const router = useRouter();

  useEffect(() => {
    // Handle route change
    router.events.on('routeChangeStart', (url) => {
      setLoading(true);
    });

    router.events.on('routeChangeError', () => {
      setTimeout(() => {
        setLoading(false);
      }, 2000);
    });

    router.events.on('routeChangeComplete', () => {
      setTimeout(() => {
        setLoading(false);
      }, 2000);
    });
  }, []);

  return (
    <>
      <Head>
        {/* added meta tags */}
        <meta httpEquiv="Content-Type" content="text/html; charset=utf-8" />
        <meta charSet="UTF-8" />
        <meta
          name="viewport"
          content="width=device-width,initial-scale=1,shrink-to-fit=no"
        />
      </Head>
      {isLoading ? (
        <NextNProgress
          color="linear-gradient(90deg, #22D3EE, #FACC15)"
          height={4}
          transformCSS={(css: string) => {
            return (
              <style>
                {css +
                  ' #nprogress { position: absolute; top: 4.5rem; z-index: 9999; width: 100vw; }'}
              </style>
            );
          }}
        />
      ) : (
        <></>
      )}
      <Component {...pageProps} />
    </>
  );
}

export default MyApp;

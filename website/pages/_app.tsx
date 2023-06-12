import "../styles/globals.css";
import { config } from "@fortawesome/fontawesome-svg-core";

import "@fortawesome/fontawesome-svg-core/styles.css";
config.autoAddCss = false;
import type { AppProps } from "next/app";

import smoothScroll from "smoothscroll-polyfill";
import { useEffect } from "react";
import Head from "next/head";

import SEO from "../next-seo.config";
import { DefaultSeo } from "next-seo";

function MyApp({ Component, pageProps }: AppProps) {
  useEffect(() => {
    smoothScroll.polyfill();
  });
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
      <DefaultSeo {...SEO} />
      <Component {...pageProps} />
    </>
  );
}

export default MyApp;

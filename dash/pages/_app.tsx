import { ChakraProvider } from "@chakra-ui/react";
import { extendTheme, ThemeConfig } from "@chakra-ui/react";
import Head from "next/head";
import Script from "next/script";
import { Container } from "../components/Container";
const config: ThemeConfig = {
  initialColorMode: "dark",
  useSystemColorMode: false,
};

const theme = extendTheme({
  config,
});

const meta = {
  description: "Ascella dashboard made using next.js - Ascella is the fastest image uploader",
  title: "Ascella",
  type: "website",
  themeColor: "#2C3748",
};

function UploadApp({ Component, pageProps }: any) {
  return (
    <>
      <Head>
        <meta name="theme-color" content={meta.themeColor} />

        <meta name="robots" content="follow, index" />
        <meta content={meta.description} name="description" />
        <link rel="icon" href="logo.svg"  />
        <link rel="icon" href="logo.png"  />
   
        <meta property="og:type" content={meta.type} />
        <meta property="og:site_name" content={meta.title} />
        <meta property="og:description" content={meta.description} />
        <meta property="og:title" content={meta.title} />
        <meta name="twitter:title" content={meta.title} />
        <meta name="twitter:description" content={meta.description} />
      </Head>
      <Script
        id="umami"
        async
        defer
        data-website-id="75176083-526f-437c-95c3-59a10bd49ac7"
        src="https://analytics.tricked.pro/umami.js"
      >
      </Script>
      <Container>
        <ChakraProvider theme={theme}>
          <Component {...pageProps} />
        </ChakraProvider>
      </Container>
    </>
  );
}

export default UploadApp;

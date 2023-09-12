import Image from "next/image";
import { Balsamiq_Sans } from "next/font/google";

const balsamiq = Balsamiq_Sans({
  weight: "400",
  subsets: ["latin"],
});

export default {
  logo: (
    <>
      <Image
        src="/images/trident-logo.png"
        alt="logo"
        width={38}
        height={38}
        style={{ marginLeft: ".4em" }}
      />
      <span
        className={`${balsamiq.className} logo`}
        style={{ marginLeft: ".4em", marginTop: ".15em", fontSize: "2em" }}
      >
        {" "}
        ShuLin
      </span>
    </>
  ),
  project: {
    link: "https://github.com/vyogami/shulin",
  },
  themeSwitch: {
    component() {
      return () => null;
    },
  },

  project: {
    link: "https://github.com/vyogami/shulin",
  },

  docsRepositoryBase: "https://github.com/vyogami/shulin/blob/main/docs",

  sidebar: {
    defaultMenuCollapseLevel: 1,
    toggleButton: true,
  },

  useNextSeoProps() {
    return {
      titleTemplate: "%s - ShuLin",
    };
  },

  nextThemes: {
    defaultTheme: "dark",
  },

  head: (
    <>
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      <meta property="og:title" content="ShuLin" />
      <meta property="og:description" content="Ubuntu hardening for SIH 2023" />
      <link rel="icon" href="/favicon/favicon.ico" />
    </>
  ),

  footer: {
    text: (
      <>
        <span style={{ marginLeft: "1em", marginTop: ".2em" }}>
          <a href="https://github.com/vyogami/shulin" target="_blank">
            Made with ❤️ by ShuLin Team
          </a>
        </span>
      </>
    ),
  },
};

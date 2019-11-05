import React from "react";
import NextApp, { AppContext, AppInitialProps } from "next/app";

import "@/assets/styles/tailwind.css";
import MainLayout from "@/layouts/main";

export default class RustMalWebApp extends NextApp {
  static async getInitialProps({ Component, ctx }: AppContext): Promise<AppInitialProps> {
    return {
      pageProps: Component.getInitialProps ? await Component.getInitialProps(ctx) : {},
    };
  }

  render() {
    const { Component, pageProps } = this.props;
    return (
      <MainLayout>
        <Component {...pageProps} />
      </MainLayout>
    );
  }
}

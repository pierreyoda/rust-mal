import React from "react";
import Head from "next/head";
import { NextPage } from "next";

import Nav from "../components/nav";

const Home: NextPage<{ rustMessage: string }> = ({ rustMessage }) => (
  <div>
    <Head>
      <title>Home</title>
    </Head>
    <Nav />
    <div className="hero">
      <h1 className="title">Welcome to Next.js!</h1>
      <p className="description">
        Hello from Rust:
        <code>{rustMessage}</code>
      </p>
    </div>
    <style jsx>
      {`
    .hero {
      width: 100%;
      color: #333;
    }
    .title {
      margin: 0;
      width: 100%;
      padding-top: 80px;
      line-height: 1.15;
      font-size: 48px;
    }
    .title,
    .description {
      text-align: center;
    }
  `}
    </style>
  </div>
);

Home.getInitialProps = async () => {
  const { greet } = await import("../../rust_mal_lib_wasm/pkg");
  return { rustMessage: greet() };
};

export default Home;

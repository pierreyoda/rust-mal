import React from 'react'
import Head from 'next/head'
import Nav from '../components/nav'

import { greet } from "rust-mal-lib-wasm";

const Home = ({ rustMessage }) => {
  (
    <div>
      <Head>
        <title>Home</title>
      </Head>

      <Nav />

      <div className='hero'>
        <h1 className='title'>Welcome to Next.js!</h1>
        <p className='description'>
          Hello from Rust: <code>{rustMessage}</code> \o/
      </p>
      </div>

      <style jsx>{`
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
    `}</style>
    </div>
  )
};

NextPage.getInitialProps = () => ({
  rustMessage: greet(),
});

export default Home

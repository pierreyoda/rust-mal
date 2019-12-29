import React from "react";
import Link from "next/link";

const Nav = () => (
  <nav className="w-full flex px-6 py-2">
    <ul className="flex-grow flex items-center justify-between">
      <li>
        <Link href="/">
          <a>Home</a>
        </Link>
      </li>
      <li>
        <Link href="/about">
          <a>About</a>
        </Link>
      </li>
      <li>
        <Link href="https://github.com/pierreyoda/rust-mal" prefetch={false}>
          <a target="_blank" rel="noopener">GitHub</a>
        </Link>
      </li>
    </ul>
  </nav>
);

export default Nav;

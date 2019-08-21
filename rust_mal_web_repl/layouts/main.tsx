import React, { FunctionComponent } from "react";

import Nav from "@/components/partials/nav";
import Head from "@/components/partials/head";

const MainLayout: FunctionComponent = ({ children }) => (
  <div>
    <Head
      title="Rust Lisp Interpreter REPL"
      description="Web-based interactive REPL front-end for the rust-mal Make A Lisp interpreter."
      url="TODO:"
      ogImage="TODO:"
    />
    <Nav />
    <main>
      {children}
    </main>
  </div>
);

export default MainLayout;

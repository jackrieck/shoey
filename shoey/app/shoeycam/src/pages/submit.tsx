import type { NextPage } from "next";
import Head from "next/head";
import { SubmitView } from "../views";

const Submit: NextPage = (props) => {
  return (
    <div>
      <Head>
        <title>Submit</title>
        <meta name="description" content="Submit Shoey Video" />
      </Head>
      <SubmitView />
    </div>
  );
};

export default Submit;

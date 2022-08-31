import type { NextPage } from "next";
import Head from "next/head";
import { ClaimView } from "../views";

const Claim: NextPage = (props) => {
  return (
    <div>
      <Head>
        <title>Claim</title>
        <meta
          name="description"
          content="Claim $DUST"
        />
      </Head>
      <ClaimView />
    </div>
  );
};

export default Claim;

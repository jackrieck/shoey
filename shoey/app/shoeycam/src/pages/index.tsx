import type { NextPage } from "next";
import Head from "next/head";
import { HomeView } from "../views";

const Home: NextPage = (props) => {
  return (
    <div>
      <Head>
        <title>ShoeyCam</title>
        <meta
          name="description"
          content="Shoey Cam"
        />
      </Head>
      <HomeView />
    </div>
  );
};

export default Home;

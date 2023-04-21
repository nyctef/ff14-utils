import { AppProps } from "next/app";

function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <style jsx global>{`
        body {
          background-color: #555;
          color: #eee;
          font-family: Verdana, Arial, sans-serif;
        }
      `}</style>
      <Component {...pageProps} />
    </>
  );
}

export default App;

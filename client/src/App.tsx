import ChatHeader from "@/components/ChatHeader.tsx";
import ChatMessages from "@/components/ChatMessages.tsx";
import ChatFooter from "@/components/ChatFooter.tsx";
import ReactParallaxTilt from "react-parallax-tilt";
import AppStateProvider from "./components/AppStateProvider";

function App() {
  return (
    <div className="h-screen w-screen fixed grid place-items-center bg-gradient-to-br from-slate-950 via-slate-800 to-slate-400">
      <AppStateProvider>
        <ReactParallaxTilt
          tiltMaxAngleX={2}
          tiltMaxAngleY={2}
          className="flex flex-col h-[95%] w-full max-w-[500px] overflow-hidden bg-slate-300 mx-auto rounded-md drop-shadow-2xl"
        >
          <ChatHeader />
          <ChatMessages />
          <ChatFooter />
        </ReactParallaxTilt>
      </AppStateProvider>
    </div>
  );
}

export default App;

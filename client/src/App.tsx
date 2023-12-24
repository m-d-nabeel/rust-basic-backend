import ChatHeader from "@/components/ChatHeader.tsx";
import ChatMessages from "@/components/ChatMessages.tsx";
import ChatFooter from "@/components/ChatFooter.tsx";
import ReactParallaxTilt from "react-parallax-tilt";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import DrawerComponent from "./components/Drawer";

type LocalData = {
  user_id: number;
  name: string;
};

function App() {
  const navigate = useNavigate();

  const searchParams = new URLSearchParams(window.location.search);
  const drawer = searchParams.get("drawer");
  useEffect(() => {
    if (localStorage.getItem("chatician") && drawer === "close") {
      const localData = JSON.parse(JSON.stringify(localStorage)) as LocalData;
      const searchParams = new URLSearchParams(window.location.search);
      searchParams.set("user_id", localData.user_id.toString());
      navigate(`?${searchParams.toString()}`);
    }
  }, []);

  return (
    <div className="h-screen w-screen fixed grid place-items-center bg-gradient-to-br from-slate-950 via-slate-800 to-slate-400">
      <ReactParallaxTilt
        tiltMaxAngleX={2}
        tiltMaxAngleY={2}
        className="flex flex-col h-[95%] w-full max-w-[500px] overflow-hidden bg-slate-300 mx-auto rounded-md drop-shadow-2xl"
      >
        <DrawerComponent />
        {drawer === "close" && (
          <>
            <ChatHeader />
            <ChatMessages />
            <ChatFooter />
          </>
        )}
      </ReactParallaxTilt>
    </div>
  );
}

export default App;

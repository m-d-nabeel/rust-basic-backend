import { useEffect, useRef, useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import MessageBox from "./MessageBox";

function ChatMessages() {
  const navigate = useNavigate();
  const [messages, setMessages] = useState<any[]>([]);
  const location = useLocation();

  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    async function getMessages(id: string) {
      try {
        const response = await fetch(
          `http://localhost:5000/chatician/chat/${id}`,
          {
            method: "GET",
          }
        );

        if (!response.ok) {
          throw new Error(`HTTP error! Status: ${response.status}`);
        }
        const jsonData = await response.json();
        setMessages(jsonData);
      } catch (error) {
        console.error("Error fetching messages:", error);
      }
    }

    const searchParams = new URLSearchParams(window.location.search);
    getMessages(searchParams.get("channel_id") || "1");
    if (searchParams.has("get_message")) {
      searchParams.delete("get_message");
      navigate(`?${searchParams.toString()}`);
    }
  }, [location]);

  const handleMenuRemoval = () => {
    const searchParams = new URLSearchParams(window.location.search);
    if (searchParams.has("tr_menu")) {
      searchParams.delete("tr_menu");
      navigate(`?${searchParams.toString()}`);
    }
  };
  bottomRef.current?.scrollIntoView({ behavior: "smooth" });

  return (
    <div
      className="flex-1 overflow-y-auto p-4 space-y-4"
      onClick={handleMenuRemoval}
    >
      <div className="flex flex-col space-y-2">
        {!!messages &&
          messages.map((message: any, index: number) => (
            <MessageBox message={message} key={index} />
          ))}
        <div ref={bottomRef} />
      </div>
    </div>
  );
}

export default ChatMessages;

// bg-gray-200

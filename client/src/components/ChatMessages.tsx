import { cn } from "@/lib/utils";
import { useEffect, useRef, useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";

function ChatMessages() {
  const navigate = useNavigate();
  const [messages, setMessages] = useState<any[]>([]);
  const location = useLocation();

  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    async function getMessages() {
      try {
        const response = await fetch("http://localhost:5000/chatician/send", {
          method: "GET",
        });

        if (!response.ok) {
          throw new Error(`HTTP error! Status: ${response.status}`);
        }
        const jsonData = await response.json();
        setMessages(jsonData);
      } catch (error) {
        console.error("Error fetching messages:", error);
      }
    }

    getMessages();
    const searchParams = new URLSearchParams(window.location.search);
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
            <div
              className={cn(
                "flex items-end justify-start",
                index % 2 === 0 && "justify-end"
              )}
              key={message.id}
            >
              <div className="max-w-xs rounded-lg bg-gray-200 p-3">
                <p className="text-sm">{message.message_text}</p>
                <p className="text-xs text-gray-500 mt-1">
                  {message.timestamp}
                </p>
              </div>
            </div>
          ))}
        <div ref={bottomRef} />
      </div>
    </div>
  );
}

export default ChatMessages;

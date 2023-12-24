import { PlusCircleIcon, SendIcon } from "lucide-react";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { Button } from "./ui/button";

function ChatFooter() {
  const [message, setMessage] = useState("");
  const navigate = useNavigate();

  const handleClick = async () => {
    const searchParams = new URLSearchParams(window.location.search);
    const channel_id = searchParams.get("channel_id") || "1";
    fetch(`http://localhost:5000/chatician/chat/${channel_id}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ message_text: message, user_id: 2 }),
    })
      .then((response) => {
        if (!response.ok) {
          throw new Error(`HTTP error! Status: ${response.status}`);
        }
        console.log(response);
        searchParams.set("get_message", "true");
        navigate(`?${searchParams.toString()}`);
      })
      .catch((err) => console.error("[[MESSAGE_SENDING_ERROR]]", err))
      .finally(() => setMessage(""));
  };

  return (
    <footer className="flex items-center p-4 bg-gray-100">
      <PlusCircleIcon className="text-gray-600" />
      <input
        aria-label="Type a message"
        className="flex-1 mx-4 p-2 border rounded-md"
        placeholder="Type a message"
        onKeyDownCapture={(e) => {
          if (e.key === "Enter") {
            handleClick();
          }
        }}
        type="text"
        value={message}
        onChange={(e) => setMessage(e.target.value)}
      />
      <Button variant="ghost" size="icon">
        <SendIcon className="text-gray-600" onClick={handleClick} />
      </Button>
    </footer>
  );
}

export default ChatFooter;

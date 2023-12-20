import { PlusCircleIcon, SendIcon } from "lucide-react";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

function ChatFooter() {
  const [message, setMessage] = useState("");
  const navigate = useNavigate();

  const handleClick = async () => {
    const options = {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "User-Agent": "insomnia/8.5.0",
      },
      body: JSON.stringify({ message_text: message }),
    };

    fetch("http://localhost:5000/chatician/send", options)
      .then((response) => response.json())
      .then((response) => {
        console.log(response);
        const searchParams = new URLSearchParams(window.location.search);
        searchParams.set("get_message", "true");
        navigate(`?${searchParams.toString()}`);
      })
      .catch((err) => console.error(err))
      .finally(() => setMessage(""));
  };

  return (
    <footer className="flex items-center p-4 bg-gray-100">
      <PlusCircleIcon className="text-gray-600" />
      <input
        aria-label="Type a message"
        className="flex-1 mx-4 p-2 border rounded-md"
        placeholder="Type a message"
        type="text"
        value={message}
        onChange={(e) => setMessage(e.target.value)}
      />
      <SendIcon className="text-gray-600" onClick={handleClick} />
    </footer>
  );
}

export default ChatFooter;

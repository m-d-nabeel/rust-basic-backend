import { cn } from "@/lib/utils";
import { useEffect, useState } from "react";

type MessageBoxProps = {
  message: {
    id: number;
    name: string;
    user_id: number;
    message_text: string;
    created_at: number[];
  };
};

export default function MessageBox({ message }: MessageBoxProps) {
  const timestamp = new Date(message.created_at[2]);
  timestamp.setSeconds(message.created_at[4]);
  timestamp.setMinutes(message.created_at[3]);
  const [side, setSide] = useState<"left" | "right">("left");
  useEffect(() => {
    const searchParams = new URLSearchParams(window.location.search);
    const user_id = searchParams.get("user_id") || "694201337";
    if (message.user_id.toString() === user_id) {
      setSide("right");
    }
  }, []);
  return (
    <div
      className={cn(
        "flex items-end justify-start",
        side === "left" && "justify-end"
      )}
      key={message.id}
    >
      <div
        className={cn(
          "max-w-xs rounded-lg p-3",
          side === "left" ? "bg-blue-100" : "bg-green-100"
        )}
      >
        <p className="text-xs text-gray-500">{message.name}</p>
        <p className="text-sm">{message.message_text}</p>
      </div>
      <p className="text-xs text-gray-500 mt-1">
        {timestamp.toLocaleTimeString()}
      </p>
    </div>
  );
}

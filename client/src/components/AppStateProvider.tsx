import React, {
  Dispatch,
  SetStateAction,
  createContext,
  useState,
} from "react";

interface AppState {
  messages: string[];
  setMessages: Dispatch<SetStateAction<string[]>>;
}

export const AppStateContext = createContext<AppState>({
  messages: [],
  setMessages: () => {},
});

export default function AppStateProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const [messages, setMessages] = useState<string[]>([]);

  return (
    <AppStateContext.Provider
      value={{
        messages,
        setMessages,
      }}
    >
      {children}
    </AppStateContext.Provider>
  );
}

import { createSignal, createContext, useContext } from "solid-js";

const MessageContext = createContext([{},() => {}]);

export function MessageProvider(props) {
  const [message, setMessage] = createSignal({});

  return (
    <MessageContext.Provider value={[message, setMessage]}>
      {props.children}
    </MessageContext.Provider>
  );
}

export function useContextMessage() { return useContext(MessageContext); }
import { useLocation, A } from "@solidjs/router";
import { useContextMessage } from "../common/MessageProvider";

export default function Message() { 
  const [message, setMessage] = useContextMessage();
  return(
    <Show when={Object.keys(message()).length > 0}>
      <div class="message">
        <p>
          Status: { message().status } <br/>
          Result: { message().result }
        </p>
        <Show when={message().status == 401 && useLocation().pathname != '/login'}>
          <p>Please <A href={ '/login?go=' + encodeURIComponent(useLocation().pathname) }>Login</A></p>
        </Show>
      </div>
    </Show>
  )
}

import { useParams, A } from "@solidjs/router";
import { MessageProvider } from "../common/MessageProvider";
import Navigation from "./Navigation";
import Message from "./Message";
import NonCompliantByEnvironmentBuChart from "../charts/NonCompliantByEnvironmentBuChart";
import NonCompliantByBuTable from "../charts/NonCompliantByBuTable";
import ErratasTable from "../charts/ErratasTable";

export default function Erratas() {
  const params = useParams();
  const ts = params.ts;

  return (
    <MessageProvider>
      <Navigation />
      <div class="contents">
        <div class="links">
          <A activeClass="active" end="true" href="/erratas">
            Current
          </A>
          <A activeClass="active" href="/erratas/180">
            More than 180 days
          </A>
        </div>
        <Message />
        <div class="flex">
          <NonCompliantByEnvironmentBuChart ts={ts} />
          <NonCompliantByBuTable ts={ts} />
        </div>
        <div style="display:inline-block">
          <ErratasTable ts={ts} />
        </div>
      </div>
    </MessageProvider>
  );
}

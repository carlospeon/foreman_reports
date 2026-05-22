import { MessageProvider } from '../common/MessageProvider';
import Navigation from './Navigation';
import Message from './Message';

import HostsByLocationEnvironmentChart from '../charts/HostsByLocationEnvironmentChart';
import HostsByBUChart from '../charts/HostsByBUChart';
import EnvironmentChart from '../charts/EnvironmentChart';
import LocationChart from '../charts/LocationChart';
import AdminGroupChart from '../charts/AdminGroupChart';
import DomainsChart from '../charts/DomainsChart';
import DomainsHistory from '../charts/DomainsHistory';
import HardwareChart from '../charts/HardwareChart';
import HostsResourcesByLocationTable from '../charts/HostsResourcesByLocationTable';


export default function Home() {

  return (
    <MessageProvider>
      <Navigation />
      <div class="contents">
        <Message />
        <div class="flex">
          <HostsResourcesByLocationTable />
          <LocationChart/>
          <EnvironmentChart/>
          <AdminGroupChart/>
          <HostsByBUChart />
          <HostsByLocationEnvironmentChart />
    {/*</div>
        <hr />
        <div class="flex">*/}
          <DomainsChart/>
          <DomainsHistory/>
    {/*</div>
        <hr />
        <div class="flex">*/}
          <HardwareChart/>
        </div>
      </div>
    </MessageProvider>
    
  )
}

// export default Home

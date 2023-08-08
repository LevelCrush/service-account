import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import {
  Route,
  Routes as ReactRouterRoutes,
  HashRouter,
  Link,
} from 'react-router-dom';
import Routes from '@config/routes';

import './App.css';

function App() {
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  return (
    <HashRouter>
      <div className="flex">
        <nav>
          <ul>
            {Routes.map((route, routeIndex) => (
              <li
                key={'application_menu_route_' + routeIndex + '_' + route.url}
              >
                <Link to={route.url}>{route.name}</Link>
              </li>
            ))}
          </ul>
        </nav>
        <main>
          <ReactRouterRoutes>
            {Routes.map((route, routeIndex) => (
              <Route
                index={routeIndex === 0}
                path={route.url}
                key={'application_route_' + routeIndex + '_' + route.url}
                Component={route.component}
              />
            ))}
          </ReactRouterRoutes>
        </main>
      </div>
    </HashRouter>
  );
}

export default App;

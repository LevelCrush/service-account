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
import {
  Navbar,
  NavbarContent,
  NavbarMenu,
  NavbarMenuItem,
  NavbarMenuToggle,
} from '@nextui-org/react';

function App() {
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  return (
    <HashRouter>
      <Navbar
        onMenuOpenChange={(isOpen) => setIsMenuOpen(isOpen ? true : false)}
      >
        <NavbarContent>
          <NavbarMenuToggle
            aria-label={isMenuOpen ? 'Close Menu' : 'Open Menu'}
          />
        </NavbarContent>
        <NavbarMenu>
          {Routes.map((route, routeIndex) => (
            <NavbarMenuItem
              key={`application_menu_route_${routeIndex}_${route.url}`}
            >
              <Link to={route.url}>{route.name}</Link>
            </NavbarMenuItem>
          ))}
        </NavbarMenu>
      </Navbar>
      <main>
        <ReactRouterRoutes>
          {Routes.map((route, routeIndex) => (
            <Route
              index={routeIndex === 0}
              path={route.url}
              key={`application_route_${routeIndex}_${route.url}`}
              Component={route.component}
            />
          ))}
        </ReactRouterRoutes>
      </main>
    </HashRouter>
  );
}

export default App;

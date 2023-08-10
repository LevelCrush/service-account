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
  NavbarBrand,
  NavbarContent,
  NavbarMenu,
  NavbarMenuItem,
  NavbarMenuToggle,
} from '@nextui-org/react';

function App() {
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  return (
    <HashRouter>
      <div className="flex flex-col min-h-screen h-auto">
        <Navbar
          className="grow-0 shrink-0 basis-auto w-full"
          onMenuOpenChange={(isOpen) => setIsMenuOpen(isOpen ? true : false)}
        >
          <NavbarContent justify="start">
            <NavbarMenuToggle
              aria-label={isMenuOpen ? 'Close Menu' : 'Open Menu'}
            />
          </NavbarContent>
          <NavbarContent justify="center">
            <NavbarBrand>
              <h1 className="font-headline text-2xl tracking-widest">
                Destiny App
              </h1>
            </NavbarBrand>
          </NavbarContent>
          <NavbarContent justify="end">&nbsp;</NavbarContent>
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
        <div className="content w-full flex-1 h-auto bg-slate-950">
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
        </div>
      </div>
    </HashRouter>
  );
}

export default App;

import ClanPage from '@pages/clan';
import HomePage from '@pages/home';
import { ComponentType } from 'react';

export interface RouteItem {
  url: string;
  name: string;
  component: ComponentType;
  children?: RouteItem[];
}

/** These are the standard routes, intended for top level site navigation  */
export const Routes = [
  {
    url: '/',
    name: 'Home',
    component: HomePage,
  },

  {
    url: '/clan',
    name: 'Clan Network',
    component: ClanPage,
  },
] as RouteItem[];

export default Routes;

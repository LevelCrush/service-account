import React, { useEffect, useState } from 'react';
import { faAngleDoubleRight, faBars } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { Routes, RouteItem } from '@website/core/routes';
import Hyperlink from '@website/components/elements/hyperlink';
import { H1 } from '@website/components/elements/headings';

export interface OffCanvasToggleProps {
  className?: string;
}

export interface OffCanvasProps {
  routes?: RouteItem[];
}

/**
 * Expected off canvas events
 */
export interface OffCanvasEvent {
  offcanvas_hide: null;
  offcanvas_request_toggle: null;
  offcanvas_show: null;
}

/**
 * Subscribes to off canvas specific events
 * @param event
 * @param cb
 */
export function subscribeOffCanvasEvent<K extends keyof OffCanvasEvent>(
  event: K,
  cb: (ev: CustomEvent) => void
) {
  document.addEventListener(event, cb as unknown as EventListenerObject);
}

/**
 * Unscribes from off canvas related events
 * @param event
 * @param cb
 */
export function unsubscribeOffCanvasEvent<K extends keyof OffCanvasEvent>(
  event: K,
  cb: (ev: CustomEvent) => void
) {
  document.removeEventListener(event, cb as unknown as EventListenerObject);
}

function dispatchEvent(event: keyof OffCanvasEvent) {
  document.dispatchEvent(new CustomEvent(event));
}

export const OffCanvasToggle = (props: OffCanvasToggleProps) => (
  <button
    className={'mr-0 ' + (props.className || '')}
    onClick={() => {
      dispatchEvent('offcanvas_request_toggle');
    }}
  >
    <FontAwesomeIcon icon={faBars}></FontAwesomeIcon>
  </button>
);

/**
 * Provides a pull out menu that is part of the page, toggling visibiliy is done through a hamburger menu
 * @param props
 * @constructor
 */
export const OffCanvas = (props: React.PropsWithChildren<OffCanvasProps>) => {
  const [showing, setShowing] = useState(false);
  const [isMember, setIsMember] = useState(false);
  const [isAdmin, setIsAdmin] = useState(false);
  const [routes, setRoutes] = useState(props.routes || Routes);

  const eventCanvasToggle = () => {
    dispatchEvent(showing ? 'offcanvas_hide' : 'offcanvas_show');
  };

  const eventCanvasShow = () => {
    setShowing(true);
  };

  const eventCanvasHide = () => {
    setShowing(false);
  };

  // component mount effect
  useEffect(() => {
    // offcanvas events
    subscribeOffCanvasEvent('offcanvas_request_toggle', eventCanvasToggle);
    subscribeOffCanvasEvent('offcanvas_show', eventCanvasShow);
    subscribeOffCanvasEvent('offcanvas_hide', eventCanvasHide);

    return () => {
      //cleanup

      // off canvas events
      unsubscribeOffCanvasEvent('offcanvas_request_toggle', eventCanvasToggle);
      unsubscribeOffCanvasEvent('offcanvas_hide', eventCanvasHide);
      unsubscribeOffCanvasEvent('offcanvas_show', eventCanvasShow);
    };
  }, []);

  return (
    <div
      className="offcanvas relative top-0 min-h-screen h-auto"
      data-showing={showing ? '1' : '0'}
      data-is-member={isMember ? '1' : '0'}
    >
      <nav
        data-offcanvas="main"
        className="offcanvas-menu bg-gradient-to-b from-white  to-slate-300 dark:bg-slate-900 dark:from-slate-800 dark:to-slate-900   shadow-[0px_1rem_1rem_2px_rgba(0,0,0,0.7)] border-r-cyan-400 border-r-2 border-r-solid bg-black text-black dark:text-white fixed z-[99999] top-0 -translate-x-full offcanvas-opened:-translate-x-0 w-[20rem]  transition-all duration-300 h-screen overflow-auto"
      >
        <H1
          className="align-middle text-black dark:text-yellow-400 text-center text-4xl font-headline font-bold uppercase tracking-widest my-4 transition duration-300"
          minimalCSS={true}
        >
          <Hyperlink className="!hover:no-underline" href="/" title="Go home">
            Level Crush
          </Hyperlink>
        </H1>
        <ul className="text-white font-bold">
          {(routes || []).map((routeItem, routeItemIndex) => {
            if (routeItem.loginOnly && isMember === false) {
              return <></>;
            }
            if (routeItem.adminOnly && isAdmin === false) {
              return <></>;
            }
            return (
              <li
                className="group text-black dark:text-white border-b-[1px] first:border-t-[1px] border-solid border-black dark:border-cyan-500"
                key={'routeitem_' + routeItemIndex + '_' + routeItem.url}
              >
                <Hyperlink
                  className="p-4 block hover:bg-black hover:bg-opacity-10 dark:hover:bg-white dark:hover:bg-opacity-10 transition duration-300"
                  href={routeItem.url}
                  data-has-sub={
                    (routeItem.children || []).length > 0 ? '1' : '0'
                  }
                  onClick={
                    routeItem.children
                      ? (ev) => {
                          if (ev.target) {
                            const closetLi = (
                              ev.target as HTMLAnchorElement
                            ).closest('li');
                            if (closetLi) {
                              closetLi.classList.toggle('expanded');
                              ev.preventDefault();
                              return false;
                            }
                          }
                        }
                      : undefined
                  }
                >
                  {(routeItem.children || []).length > 0 ? (
                    <div
                      className="inline-block float-right px-4"
                      key={'routeitem_' + routeItem.url + '_expansion_toggle'}
                    >
                      <FontAwesomeIcon
                        className="expanded:rotate-90 group-hover:rotate-90 transition-all duration-300"
                        icon={faAngleDoubleRight}
                      ></FontAwesomeIcon>
                    </div>
                  ) : (
                    <></>
                  )}
                  {routeItem.name}
                  <div className="clear-both"></div>
                </Hyperlink>

                <nav className="offcanvas-sub-menu max-h-0 h-auto overflow-hidden transition-all duration-300 ease-in-out expanded:max-h-[100rem]">
                  <ul className="text-white font-bold">
                    {(routeItem.children || []).map(
                      (subChild, subChildIndex) => (
                        <li
                          className="text-black dark:bg-slate-900 bg-yellow-100  dark:text-white border-b-[1px] last:border-b-0 first:border-t-[1px] border-black dark:border-cyan-500 border-solid"
                          key={
                            'route_item' +
                            routeItemIndex +
                            '_' +
                            routeItem.url +
                            '_sub_' +
                            subChild.url +
                            '_' +
                            subChildIndex
                          }
                        >
                          <Hyperlink
                            className="p-4 block hover:bg-black hover:bg-opacity-10 dark:hover:bg-white dark:hover:bg-opacity-10 transition duration-300"
                            href={subChild.url}
                          >
                            <span className="block border-l-2  border-solid border-black dark:border-cyan-500 pl-4">
                              {subChild.name}
                            </span>
                          </Hyperlink>
                        </li>
                      )
                    )}
                  </ul>
                </nav>
              </li>
            );
          })}
        </ul>
      </nav>
      <div className="offcanvas-content min-h-screen h-auto  block transition-all duration-300">
        {props.children}
      </div>
      <div
        className="offcanvas-background hidden transition-all duration-300 h-screen w-screen fixed opacity-0 top-0 bg-black offcanvas-opened:opacity-75 offcanvas-opened:block z-[99998]"
        onClick={(ev) => {
          dispatchEvent('offcanvas_hide');
        }}
      ></div>
    </div>
  );
};

export default OffCanvas;

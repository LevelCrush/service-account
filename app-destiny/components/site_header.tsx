import React, { useEffect } from 'react';
import Container from '@website/components/elements/container';
import { H1 } from '@website/components/elements/headings';
import Hyperlink from '@website/components/elements/hyperlink';
import { OffCanvasToggle } from '@website/components/offcanvas';

export interface SiteHeaderProps {
  forceStickyStyle?: boolean;
}

export const SiteHeader = (props: SiteHeaderProps) => {
  useEffect(() => {
    if (props.forceStickyStyle) {
      const el = document.querySelector('.navigation-bar');
      if (el) {
        el.classList.add('is-sticky');
      }
    } else {
      const el = document.querySelector('.navigation-bar');
      const observer = new IntersectionObserver(
        ([e]) =>
          e.target.classList.toggle('is-sticky', e.intersectionRatio < 1),
        { threshold: [1] }
      );
      if (el) {
        observer.observe(el);
      }

      return () => {
        if (el) {
          observer.unobserve(el);
        }
        observer.disconnect();
      };
    }
  }, []);

  return (
    <header className="top-[-1px] sticky z-[99] navigation-bar backdrop-blur-sm transition-all">
      <div className="min-h-[4.5rem] flex items-center h-auto transition-all bg-[rgba(0, 0, 0, 0.35);] sticky:bg-[rgba(0,33,52,1)] border-b-8 border-solid border-cyan-400 shadow-[0px_.5rem_.5rem_2px_rgba(0,0,0,0.7)] relative z-[99] ">
        <Container
          minimalCSS={true}
          className="relative flex-auto px-4 flex mx-auto my-0 justify-between items-center flex-wrap md:flex-nowrap "
        >
          <div className="flex-initial text-center md:text-left absolute">
            <OffCanvasToggle className="float-left text-yellow-400  text-4xl font-headline font-bold uppercase tracking-widest" />
            <div className="clear-both"></div>
          </div>

          <H1 className="flex-auto text-center transition-all">
            <Hyperlink className="!hover:no-underline" href="/" title="Go home">
              Level Crush
            </Hyperlink>
          </H1>

          <div className="right-4 absolute flex-auto basis-full md:basis-auto  text-center mt-8 mb-8 md:mt-0 md:mb-0 md:flex-initial md:text-right hidden md:block">
            &nbsp;
          </div>
        </Container>
      </div>
    </header>
  );
};

export default SiteHeader;

import React from 'react';

export interface HeroProps {
  backgroundUrl?: string;
  className?: string;
}

export const Hero = (props: React.PropsWithChildren<HeroProps>) => (
  <div
    className={
      'flex-auto basis-full relative top-0 left-0 hero bg-cover bg-center  h-auto flex flex-col items-center justify-center border-b-8 border-solid border-cyan-400 shadow-[0px_.3rem_1rem_2px_rgba(0,0,0,0.4)] ' +
      (props.className || '')
    }
    style={{
      backgroundImage: 'url(' + (props.backgroundUrl || '/hero.jpg') + ')',
    }}
  >
    <div className="absolute top-0 left-0 bg-black opacity-[.65] w-full h-full"></div>
    {props.children}
  </div>
);

export default Hero;

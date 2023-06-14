import React from 'react';

export interface HeroProps {
  backgroundUrl?: string;
  className?: string;
  youtubeID?: string;
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
    {props.youtubeID ? (
      <iframe
        id="heroEmbedFrame"
        width="1920"
        height="1080"
        className=" w-[400%]  h-[50rem] 720p:w-[80rem] 720p:h-[45rem] 1080p:w-[120rem] 1080p:h-[67.5rem] 2k:w-[144rem] 2k:h-[80rem] 1440p:w-[160rem]  1440p:h-[90rem] 4k:w-[240rem] 4k:h-[135rem]"
        src={
          'https://www.youtube-nocookie.com/embed/' +
          encodeURIComponent(props.youtubeID) +
          '?iv_load_policy=3controls=0&autoplay=1&disablekb=1&fs=0&showinfo=0&rel=0&loop=1&playlist=' +
          encodeURIComponent(props.youtubeID) +
          '&modestbranding=1&playsinline=1&mute=1'
        }
        title="YouTube video player"
        frameBorder="0"
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
      ></iframe>
    ) : (
      <></>
    )}
    {props.children}
  </div>
);

export default Hero;

import React from "react";

export interface ContainerProps {
  minimalCSS?: boolean;
  className?: string;
}

export const Container = (props: React.PropsWithChildren<ContainerProps>) => (
  <div
    className={
      "container " +
      (props.minimalCSS ? "" : " px-4 mx-auto mt-8 mb-16 ") +
      (props.className || "")
    }
  >
    {props.children}
  </div>
);

export default Container;

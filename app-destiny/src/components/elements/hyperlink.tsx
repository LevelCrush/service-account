import React from 'react';
import { Link } from 'react-router-dom';

export interface HyperLinkProps
  extends React.AnchorHTMLAttributes<HTMLAnchorElement> {
  href: string;
  target?: '_self' | '_blank';
}

export const Hyperlink = (props: HyperLinkProps) => (
  <Link
    {...props}
    className={' hover:underline ' + (props.className || '')}
    to={props.href}
    target={props.target || (props.href.includes('http') ? '_blank' : '_self')}
  >
    {props.children}
  </Link>
);

export default Hyperlink;

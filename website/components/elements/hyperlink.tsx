import Link from 'next/link';
import React from 'react';

export interface HyperLinkProps
  extends React.AnchorHTMLAttributes<HTMLAnchorElement> {
  href: string;
  target?: '_self' | '_blank';
}

export const Hyperlink = (props: HyperLinkProps) => (
  <Link
    {...props}
    className={' hover:underline ' + (props.className || '')}
    href={props.href}
    target={props.target || (props.href.includes('http') ? '_blank' : '_self')}
  >
    {props.children}
  </Link>
);

export default Hyperlink;
